//! Custom benchmark code added by Inversed Tech.

#![allow(deprecated)]

use rand::Rng;

use concrete_commons::parameters::*;
use concrete_core::backends::core::private::crypto::{
    bootstrap::FourierBuffers, encoding::PlaintextList,
};
use homdte::{rgsw::*, rlwe::*, *};
use num_traits::identities::*;

//use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// The number of bits in each operation.
pub const BIT_SIZE: usize = 1000;

fn main() {
    rgsw_xor();
    rlwe_less_eq_than();
}

/// Run one XOR operation using RGSW.
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

    let mut gsw_cts_orb: Vec<RGSWCiphertext> = Vec::with_capacity(BIT_SIZE);
    let mut gsw_cts_db: Vec<RLWECiphertext> = Vec::with_capacity(BIT_SIZE);
    let mut _gsw_cts: Vec<RGSWCiphertext> = Vec::with_capacity(BIT_SIZE);
    let mut lwe_cts: Vec<RLWECiphertext> = Vec::with_capacity(BIT_SIZE);

    // Fill with random bits
    for _ in 0..BIT_SIZE {
        // Setup inside loop
        let mut orb_ct = RGSWCiphertext::allocate(ctx.poly_size, ctx.base_log, ctx.level_count);
        let mut db_ct = RLWECiphertext::allocate(ctx.poly_size);
        let mut db_ct_not = RLWECiphertext::allocate(ctx.poly_size);

        let mut rng = rand::thread_rng();
        let random_bit_orb: bool = rng.gen();
        let random_bit_db: bool = rng.gen();
        if random_bit_orb {
            sk.encrypt_rgsw(&mut orb_ct, &ptxt_one, &mut ctx); // X
        } else {
            sk.encrypt_rgsw(&mut orb_ct, &ptxt_zero, &mut ctx); // X
        };
        if random_bit_db {
            //sk.encrypt_rgsw(&mut db_ct, &ptxt_one, &mut ctx); // X
            sk.binary_encrypt_rlwe(&mut db_ct, &ptxt_one, &mut ctx); // X
            sk.binary_encrypt_rlwe(&mut db_ct_not, &ptxt_zero, &mut ctx); // X
        } else {
            //sk.encrypt_rgsw(&mut db_ct, &ptxt_zero, &mut ctx); // X
            sk.binary_encrypt_rlwe(&mut db_ct, &ptxt_zero, &mut ctx); // X
            sk.binary_encrypt_rlwe(&mut db_ct_not, &ptxt_one, &mut ctx); // X
        };

        //gsw_cts_orb.push(RGSWCiphertext::allocate(ctx.poly_size, ctx.base_log, ctx.level_count));
        gsw_cts_orb.push(orb_ct.clone());
        //gsw_cts_db.push(RGSWCiphertext::allocate(ctx.poly_size, ctx.base_log, ctx.level_count));
        gsw_cts_db.push(db_ct.clone());

        let mut xor_res = RLWECiphertext::allocate(ctx.poly_size);

        // Bench XOR inside loop
        orb_ct.cmux(&mut xor_res, &db_ct, &db_ct_not);

        // Collect results
        lwe_cts.push(xor_res.clone());

        // TODO: check xor_pt
        let mut xor_pt = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
        sk.binary_decrypt_rlwe(&mut xor_pt, &xor_res);
    }
}

/// Run one "Less than or equal to" comparison using RLWE.
fn rlwe_less_eq_than() {
    // Setup
    let mut ctx = Context::default();
    let sk = ctx.gen_rlwe_sk();
    //let _m = ctx.poly_size.0 / 2;
    //dbg!(_m);

    let mut buffers = FourierBuffers::new(ctx.poly_size, GlweSize(2));

    let mut ptxt_one = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
    *ptxt_one
        .as_mut_polynomial()
        .get_mut_monomial(MonomialDegree(1))
        .get_mut_coefficient() = Scalar::one();

    let mut gsw_ct_one = RGSWCiphertext::allocate(ctx.poly_size, ctx.base_log, ctx.level_count);
    sk.encrypt_rgsw(&mut gsw_ct_one, &ptxt_one, &mut ctx); // X

    // TODO: benchmark a random combination of ptxt_zero and ptxt_one, or use lwe_cts
    let mut lwe_ct = RLWECiphertext::allocate(ctx.poly_size);
    sk.binary_encrypt_rlwe(&mut lwe_ct, &ptxt_one, &mut ctx); // X

    //let mut prod = RLWECiphertext::allocate(ctx.poly_size);
    //gsw_ct_one.external_product(&mut prod, &lwe_ct);

    //let mut dec_prod = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
    //sk.binary_decrypt_rlwe(&mut dec_prod, &prod);
    //dbg!(dec_prod);

    let mut prod_cts: Vec<RLWECiphertext> = Vec::with_capacity(BIT_SIZE);
    for _ in 0..BIT_SIZE {
        prod_cts.push(RLWECiphertext::allocate(ctx.poly_size));
    }

    // Bench
    let k = 1;

    gsw_ct_one.external_product(&mut prod_cts[0], &lwe_ct);

    for i in 1..BIT_SIZE {
        let temp = prod_cts[i - 1].clone(); // Introduce a temporary variable. TODO: remove it
        gsw_ct_one.external_product(&mut prod_cts[i], &temp);
    }

    //let mut actual_pt = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
    //sk.binary_decrypt_rlwe(&mut actual_pt, &prod_cts[size-k]);
    //dbg!(actual_pt);
    prod_cts[BIT_SIZE - k].less_eq_than(1100, &mut buffers);

    // Check
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
