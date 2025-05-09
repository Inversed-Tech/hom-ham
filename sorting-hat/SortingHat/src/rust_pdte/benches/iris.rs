//! Custom benchmark code added by Inversed Tech.

#![allow(deprecated)]

use rand::Rng;

use concrete_commons::parameters::*;
use concrete_core::backends::core::private::crypto::{
    bootstrap::FourierBuffers, encoding::PlaintextList,
};
use homdte::{rgsw::*, rlwe::*, *};
use num_traits::identities::*;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

/// The number of bits in each operation.
pub const BIT_SIZE: usize = 1000;

// Configure Criterion:
// Define one group for each equivalent operation, so we can compare their times.
criterion_group! {
    name = bench_xor;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().sample_size(10);
    // Add alternative xor implementations here.
    targets = bench_cmux_xor
}

criterion_group! {
    name = bench_leq;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().sample_size(10);
    targets = bench_exponent_less_eq_than
}

criterion_main!(bench_xor, bench_leq);

/// Run cmux_xor() within Criterion.
fn bench_cmux_xor(c: &mut Criterion) {
    // Setup
    let mut ctx = Context::default();
    let sk = ctx.gen_rlwe_sk();

    let mut ptxt_zero = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
    *ptxt_zero
        .as_mut_polynomial()
        .get_mut_monomial(MonomialDegree(0))
        .get_mut_coefficient() = Scalar::one();

    let mut ptxt_one = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
    *ptxt_one
        .as_mut_polynomial()
        .get_mut_monomial(MonomialDegree(1))
        .get_mut_coefficient() = Scalar::one();

    // Use random bits
    let mut orb_ct = RGSWCiphertext::allocate(ctx.poly_size, ctx.base_log, ctx.level_count);
    let mut db_ct = RLWECiphertext::allocate(ctx.poly_size);
    let mut db_ct_not = RLWECiphertext::allocate(ctx.poly_size);

    let mut rng = rand::thread_rng();
    let random_bit_orb: bool = rng.gen();
    let random_bit_db: bool = rng.gen();
    if random_bit_orb {
        sk.encrypt_rgsw(&mut orb_ct, &ptxt_one, &mut ctx); // X
    } else {
        sk.encrypt_rgsw(&mut orb_ct, &ptxt_zero, &mut ctx); // 1
    };
    if random_bit_db {
        sk.binary_encrypt_rlwe(&mut db_ct, &ptxt_one, &mut ctx); // X
        sk.binary_encrypt_rlwe(&mut db_ct_not, &ptxt_zero, &mut ctx); // 1
    } else {
        sk.binary_encrypt_rlwe(&mut db_ct, &ptxt_zero, &mut ctx); // 1
        sk.binary_encrypt_rlwe(&mut db_ct_not, &ptxt_one, &mut ctx); // X
    };

    c.bench_with_input(
        BenchmarkId::new(
            format!("cmux XOR, single bit (actual {BIT_SIZE}x larger)"),
            format!("orb: {random_bit_orb}, db: {random_bit_db}"),
        ),
        &(
            orb_ct,
            db_ct,
            db_ct_not,
            ctx,
            sk,
            random_bit_orb,
            random_bit_db,
        ),
        |b, (orb_ct, db_ct, db_ct_not, ctx, sk, random_bit_orb, random_bit_db)| {
            b.iter_with_large_drop(|| {
                cmux_xor(
                    orb_ct,
                    db_ct,
                    db_ct_not,
                    ctx,
                    sk,
                    *random_bit_orb,
                    *random_bit_db,
                )
            })
        },
    );
}

/// Run exponent_less_eq_than() within Criterion.
fn bench_exponent_less_eq_than(c: &mut Criterion) {
    // Setup
    let mut ctx = Context::default();
    let sk = ctx.gen_rlwe_sk();

    let mut ptxt_zero = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
    *ptxt_zero
        .as_mut_polynomial()
        .get_mut_monomial(MonomialDegree(0))
        .get_mut_coefficient() = Scalar::one();

    let mut ptxt_one = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
    *ptxt_one
        .as_mut_polynomial()
        .get_mut_monomial(MonomialDegree(1))
        .get_mut_coefficient() = Scalar::one();

    // Constant value of 1 (X^0)
    let mut lwe_ct_zero = RLWECiphertext::allocate(ctx.poly_size);
    sk.binary_encrypt_rlwe(&mut lwe_ct_zero, &ptxt_zero, &mut ctx);

    // Use random bits
    // TODO: should these be RWLE bits to match the XOR result?
    let mut rng = rand::thread_rng();
    let mut xor_res: Vec<RGSWCiphertext> = Vec::with_capacity(BIT_SIZE);
    for _ in 0..BIT_SIZE {
        let mut xor_bit = RGSWCiphertext::allocate(ctx.poly_size, ctx.base_log, ctx.level_count);

        let random_bit: bool = rng.gen();
        if random_bit {
            // Multiply by X
            sk.encrypt_rgsw(&mut xor_bit, &ptxt_one, &mut ctx);
        } else {
            // Multiply by 1
            sk.encrypt_rgsw(&mut xor_bit, &ptxt_zero, &mut ctx);
        };

        xor_res.push(xor_bit);
    }

    c.bench_with_input(
        BenchmarkId::new(
            "exponent less than or equal",
            format!("{BIT_SIZE} random bits"),
        ),
        &(xor_res, lwe_ct_zero, ctx, sk),
        |b, (xor_res, lwe_ct_zero, ctx, sk)| {
            b.iter_with_large_drop(|| exponent_less_eq_than(xor_res, lwe_ct_zero, ctx, sk))
        },
    );
}

/// Run one bit of an XOR operation using RGSW.
/// 1000 bits is too slow to benchmark effectively.
fn cmux_xor(
    orb_ct: &RGSWCiphertext,
    db_ct: &RLWECiphertext,
    db_ct_not: &RLWECiphertext,
    ctx: &Context,
    _sk: &RLWESecretKey,
    _random_bit_orb: bool,
    _random_bit_db: bool,
) -> RLWECiphertext {
    // TODO: exclude this allocation from the benchmark using iter_batched_ref().
    let mut xor_res = RLWECiphertext::allocate(ctx.poly_size);

    // Benchmark XOR.
    // This operation is run BIT_SIZE times inside a loop to produce a single XOR result.
    orb_ct.cmux(&mut xor_res, db_ct, db_ct_not);

    // Check XOR returns the correct value.
    //
    // TODO: run this check during warmups but not the benchmark itself.
    // We can use AtomicUsize to skip the check after N runs, or return a value that checks on drop,
    // and use Bencher.iter_with_large_drop().
    if cfg!(check) {
        let mut xor_pt = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
        _sk.binary_decrypt_rlwe(&mut xor_pt, &xor_res);

        assert_eq!(
            *xor_pt
                .as_polynomial()
                .get_monomial(MonomialDegree(0))
                .get_coefficient(),
            Scalar::from(_random_bit_orb ^ _random_bit_db),
        );
    }

    // Drop this outside the timed benchmark.
    xor_res
}

/// Run one "Less than or equal to" comparison using RLWE.
#[allow(clippy::ptr_arg)]
fn exponent_less_eq_than(
    xor_res: &Vec<RGSWCiphertext>,
    lwe_ct_zero: &RLWECiphertext,
    ctx: &Context,
    _sk: &RLWESecretKey,
) -> (FourierBuffers<Scalar>, Vec<RLWECiphertext>) {
    // TODO: exclude both these allocations from the benchmark using iter_batched_ref().
    let mut buffers = FourierBuffers::new(ctx.poly_size, GlweSize(2));

    // TODO: is keeping copies of the cumulative totals necessary?
    let mut prod_cts: Vec<RLWECiphertext> = Vec::with_capacity(BIT_SIZE);
    for _ in 0..BIT_SIZE {
        prod_cts.push(RLWECiphertext::allocate(ctx.poly_size));
    }

    // Benchmark <= operation
    //
    // TODO: make this a Rust const?
    let k = 1;

    // Create the first cumulative product by multiplying 1 (X^0) by the first xor bit (1 or X).
    xor_res[0].external_product(&mut prod_cts[0], lwe_ct_zero);
    for i in 1..BIT_SIZE {
        // Create the each cumulative product by multiplying the cumulative product by the next xor bit
        // (1 or X).
        //
        // Introduce a temporary variable. TODO: remove it
        //prod_cts[i - 1].clone_into(&mut temp);
        let temp = prod_cts[i - 1].clone();
        xor_res[0].external_product(&mut prod_cts[i], &temp);
    }

    prod_cts[BIT_SIZE - k].less_eq_than(BIT_SIZE, &mut buffers);

    // Check <= returns the correct value
    //
    // TODO: run this check during warmups but not the benchmark itself.
    // We can use AtomicUsize to skip the check after N runs, or return a value that checks on drop,
    // and use Bencher.iter_with_large_drop().
    if cfg!(check) {
        let mut out = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
        _sk.binary_decrypt_rlwe(&mut out, &prod_cts[BIT_SIZE - k]);
        assert_eq!(
            *out.as_polynomial()
                .get_monomial(MonomialDegree(0))
                .get_coefficient(),
            Scalar::one()
        );
    }

    // Drop this outside the timed benchmark.
    (buffers, prod_cts)
}
