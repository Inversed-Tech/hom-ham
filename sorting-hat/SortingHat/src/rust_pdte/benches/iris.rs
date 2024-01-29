//! Custom benchmark code added by Inversed Tech.

#![allow(deprecated)]

use rand::Rng;

use concrete_commons::parameters::*;
use concrete_core::backends::core::private::crypto::{
    bootstrap::FourierBuffers, encoding::PlaintextList,
};
use homdte::{rgsw::*, rlwe::*, *};
use num_traits::identities::*;

use criterion::{criterion_group, criterion_main, Criterion};

/// The number of bits in each operation.
pub const BIT_SIZE: usize = 1000;

// Configure Criterion:
// Define one group for each equivalent operation, so we can compare their times.
criterion_group! {
    name = bench_xor;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().sample_size(30);
    // Add alternative xor implementations here.
    targets = bench_rgsw_xor
}

criterion_group! {
    name = bench_leq;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().sample_size(10);
    targets = bench_rlwe_less_eq_than
}

criterion_main!(bench_xor, bench_leq);

/// Run rgsw_xor() within Criterion.
fn bench_rgsw_xor(c: &mut Criterion) {
    c.bench_function(&format!("RGSW XOR, single bit (x{})", BIT_SIZE), |b| {
        b.iter(rgsw_xor)
    });
}

/// Run rlwe_less_eq_than() within Criterion.
fn bench_rlwe_less_eq_than(c: &mut Criterion) {
    c.bench_function(&format!("RLWE <=, {} bits", BIT_SIZE), |b| {
        b.iter(rlwe_less_eq_than)
    });
}

/// Run one bit of an XOR operation using RGSW.
fn rgsw_xor() {
    // Setup
    let mut ctx = Context::default();
    let _sk = ctx.gen_rlwe_sk();
    let _m = ctx.poly_size.0 / 2;
    //dbg!(_m);

    let mut ptxt_zero = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
    *ptxt_zero
        .as_mut_polynomial()
        .get_mut_monomial(MonomialDegree(0))
        .get_mut_coefficient() = Scalar::one();
    //dbg!(ptxt_zero.clone().as_mut_polynomial().get_mut_monomial(MonomialDegree(0)).get_mut_coefficient());

    let mut ptxt_one = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
    *ptxt_one
        .as_mut_polynomial()
        .get_mut_monomial(MonomialDegree(1))
        .get_mut_coefficient() = Scalar::one();

    let mut ctx = Context::default();
    let sk = ctx.gen_rlwe_sk();

    //let mut ct_zero = RLWECiphertext::allocate(ctx.poly_size);
    //sk.binary_encrypt_rlwe(&mut ct_zero, &ptxt_zero, &mut ctx);
    //let mut ct_one = RLWECiphertext::allocate(ctx.poly_size);
    //sk.binary_encrypt_rlwe(&mut ct_one, &ptxt_one, &mut ctx);

    //let mut gsw_ct_zero = RGSWCiphertext::allocate(ctx.poly_size, ctx.base_log, ctx.level_count);
    //sk.encrypt_rgsw(&mut gsw_ct_zero, &ptxt_zero, &mut ctx); // 1

    //let mut gsw_cts_orb: Vec<RGSWCiphertext> = Vec::with_capacity(BIT_SIZE);
    //let mut gsw_cts_db: Vec<RLWECiphertext> = Vec::with_capacity(BIT_SIZE);
    //let mut _gsw_cts: Vec<RGSWCiphertext> = Vec::with_capacity(BIT_SIZE);
    //let mut lwe_cts: Vec<RLWECiphertext> = Vec::with_capacity(BIT_SIZE);

    // Fill with random bits
    //for _ in 0..BIT_SIZE {

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
        //sk.encrypt_rgsw(&mut db_ct, &ptxt_one, &mut ctx); // X
        sk.binary_encrypt_rlwe(&mut db_ct, &ptxt_one, &mut ctx); // X
        sk.binary_encrypt_rlwe(&mut db_ct_not, &ptxt_zero, &mut ctx); // 1
    } else {
        //sk.encrypt_rgsw(&mut db_ct, &ptxt_zero, &mut ctx); // X
        sk.binary_encrypt_rlwe(&mut db_ct, &ptxt_zero, &mut ctx); // 1
        sk.binary_encrypt_rlwe(&mut db_ct_not, &ptxt_one, &mut ctx); // X
    };

    //gsw_cts_orb.push(RGSWCiphertext::allocate(ctx.poly_size, ctx.base_log, ctx.level_count));
    //gsw_cts_orb.push(orb_ct.clone());
    //gsw_cts_db.push(RGSWCiphertext::allocate(ctx.poly_size, ctx.base_log, ctx.level_count));
    //gsw_cts_db.push(db_ct.clone());

    let mut xor_res = RLWECiphertext::allocate(ctx.poly_size);

    // Bench XOR inside loop
    orb_ct.cmux(&mut xor_res, &db_ct, &db_ct_not);

    // Collect results
    //lwe_cts.push(xor_res.clone());

    // Check XOR returns the correct value
    let mut xor_pt = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
    sk.binary_decrypt_rlwe(&mut xor_pt, &xor_res);

    assert_eq!(
        *xor_pt
            .as_polynomial()
            .get_monomial(MonomialDegree(0))
            .get_coefficient(),
        Scalar::from(random_bit_orb ^ random_bit_db),
    );
}

/// Run one "Less than or equal to" comparison using RLWE.
fn rlwe_less_eq_than() {
    // Setup
    let mut ctx = Context::default();
    let sk = ctx.gen_rlwe_sk();
    //let _m = ctx.poly_size.0 / 2;
    //dbg!(_m);

    let mut buffers = FourierBuffers::new(ctx.poly_size, GlweSize(2));

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
    let mut rwle_ct_zero = RLWECiphertext::allocate(ctx.poly_size);
    sk.binary_encrypt_rlwe(&mut rwle_ct_zero, &ptxt_zero, &mut ctx);
    // Constant value of X (X^1)
    let mut rwle_ct_one = RLWECiphertext::allocate(ctx.poly_size);
    sk.binary_encrypt_rlwe(&mut rwle_ct_one, &ptxt_one, &mut ctx);

    // Constant value of 1 (X^0)
    let mut gsw_ct_zero = RGSWCiphertext::allocate(ctx.poly_size, ctx.base_log, ctx.level_count);
    sk.encrypt_rgsw(&mut gsw_ct_zero, &ptxt_zero, &mut ctx);

    // Use random bits
    let mut rng = rand::thread_rng();
    let mut xor_res: Vec<RLWECiphertext> = Vec::with_capacity(BIT_SIZE);
    for _ in 0..BIT_SIZE {
        let mut xor_bit = RLWECiphertext::allocate(ctx.poly_size);

        let random_bit_lwe: bool = rng.gen();
        if random_bit_lwe {
            // Multiply by X
            sk.binary_encrypt_rlwe(&mut xor_bit, &ptxt_one, &mut ctx);
        } else {
            // Multiply by 1
            sk.binary_encrypt_rlwe(&mut xor_bit, &ptxt_zero, &mut ctx);
        };

        xor_res.push(xor_bit);
    }

    //let mut prod = RLWECiphertext::allocate(ctx.poly_size);
    //gsw_ct_one.external_product(&mut prod, &lwe_ct);

    //let mut dec_prod = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
    //sk.binary_decrypt_rlwe(&mut dec_prod, &prod);
    //dbg!(dec_prod);

    // TODO: is keeping copies of the cumulative totals necessary?
    let mut prod_cts: Vec<RLWECiphertext> = Vec::with_capacity(BIT_SIZE);
    for _ in 0..BIT_SIZE {
        prod_cts.push(RLWECiphertext::allocate(ctx.poly_size));
    }

    // Bench
    let k = 1;

    // Create the first cumulative product by multiplying by the first xor bit.
    xor_res[0].external_product(&mut prod_cts[0], &lwe_ct_zero);
    for i in 1..BIT_SIZE {
        // Introduce a temporary variable. TODO: remove it
        //prod_cts[i - 1].clone_into(&mut temp);
        let temp = prod_cts[i - 1].clone();
        xor_res[0].external_product(&mut prod_cts[i], &temp);
    }

    //let mut actual_pt = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
    //sk.binary_decrypt_rlwe(&mut actual_pt, &prod_cts[size-k]);
    //dbg!(actual_pt);

    prod_cts[BIT_SIZE - k].less_eq_than(BIT_SIZE, &mut buffers);

    // Check <= returns the correct value
    let mut out = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
    sk.binary_decrypt_rlwe(&mut out, &prod_cts[BIT_SIZE - k]);
    //dbg!(out.clone());
    assert_eq!(
        *out.as_polynomial()
            .get_monomial(MonomialDegree(0))
            .get_coefficient(),
        Scalar::one()
    );
}
