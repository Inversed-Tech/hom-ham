# Exploring the Sorting Hat strategy

This [paper](https://eprint.iacr.org/2022/757.pdf) proposes an efficient solution to homomorphically evaluate a decision tree, where the Hamming distance is used along the process, and it is computed "in the exponent". In other words, the output of the homomorphic calculation is a polynomial `X^h` such that `h` is precisely the Hamming distance. In order to compute `X^h`, it is required to follow a sequencial procedure where at each step we multiply by a monomial `X` if both input vector differ at the current position, or multiply by `1` otherwise. Note that if the computation occurred in the clear, multiplying by 1 would be not needed, but when we do it homomorphically we must do it.

The paper proposes an efficient method to homomorphically compute comparisons. Namely, it takes
98 microseconds per comparison. However, computation of XOR must happen sequentially, and since each XOR takes 600 microseconds, it turns out that, for input having thousands of bits, this is the bootleneck.

Next code is a unit test added to file rlwe.rs from original [repo](https://github.com/KULeuven-COSIC/SortingHat). The purpose of this unit test is to measure time for comparisons, XORs, and additions.

```
    #[test]
    fn test_less_eq_ham() {
        use rand::Rng;
        let mut ctx = Context::default();
        let sk = ctx.gen_rlwe_sk();
        let mut buffers = FourierBuffers::new(ctx.poly_size, GlweSize(2));

        let m = ctx.poly_size.0/2;
        //dbg!(m);
        let mut ptxt_zero = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
        *ptxt_zero.as_mut_polynomial().get_mut_monomial(MonomialDegree(0)).get_mut_coefficient() = Scalar::one();
        //dbg!(ptxt_zero.clone().as_mut_polynomial().get_mut_monomial(MonomialDegree(0)).get_mut_coefficient());
        let mut ptxt_one = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
        *ptxt_one.as_mut_polynomial().get_mut_monomial(MonomialDegree(1)).get_mut_coefficient() = Scalar::one();

        //let mut ct_zero = RLWECiphertext::allocate(ctx.poly_size);
        //sk.binary_encrypt_rlwe(&mut ct_zero, &ptxt_zero, &mut ctx);
        //let mut ct_one = RLWECiphertext::allocate(ctx.poly_size);
        //sk.binary_encrypt_rlwe(&mut ct_one, &ptxt_one, &mut ctx);

        let mut ctx = Context::default();
        let sk = ctx.gen_rlwe_sk();
        let mut gsw_ct_zero = RGSWCiphertext::allocate(ctx.poly_size, ctx.base_log, ctx.level_count);
        let mut gsw_ct_one = RGSWCiphertext::allocate(ctx.poly_size, ctx.base_log, ctx.level_count);
        let mut lwe_ct = RLWECiphertext::allocate(ctx.poly_size);
        sk.encrypt_rgsw(&mut gsw_ct_zero, &ptxt_zero, &mut ctx); // 1
        sk.encrypt_rgsw(&mut gsw_ct_one, &ptxt_one, &mut ctx); // X
        sk.binary_encrypt_rlwe(&mut lwe_ct, &ptxt_one, &mut ctx); // X


        let mut prod = RLWECiphertext::allocate(ctx.poly_size);
        gsw_ct_one.external_product(&mut prod, &lwe_ct);
        let mut dec_prod = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
        sk.binary_decrypt_rlwe(&mut dec_prod, &prod);
        dbg!(dec_prod);

        let size = 1000;
        let k = 1;

        let mut prod_cts: Vec<RLWECiphertext> = Vec::with_capacity(size);
        let mut gsw_cts_orb: Vec<RGSWCiphertext> = Vec::with_capacity(size);
        let mut gsw_cts_db: Vec<RLWECiphertext> = Vec::with_capacity(size);
        let mut gsw_cts: Vec<RGSWCiphertext> = Vec::with_capacity(size);
        let mut lwe_cts: Vec<RLWECiphertext> = Vec::with_capacity(size);

        // Fill with random bits
        for _ in 0..size {
            prod_cts.push(RLWECiphertext::allocate(ctx.poly_size));
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

            // XOR
            let start = Instant::now();
            orb_ct.cmux(&mut xor_res, &db_ct, &db_ct_not);
            let elapsed = start.elapsed();

            lwe_cts.push(xor_res.clone());
            let mut xor_pt = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
            sk.binary_decrypt_rlwe(&mut xor_pt, &xor_res);
            println!("Elapsed time: {:?}", elapsed);
        }

        let start = Instant::now();

        gsw_ct_one.external_product(&mut prod_cts[0], &lwe_ct);
        for i in 1..size {
            let temp = prod_cts[i - 1].clone(); // Introduce a temporary variable. TODO: remove it
            gsw_ct_one.external_product(&mut prod_cts[i], &temp);
        }

        //let mut actual_pt = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
        //sk.binary_decrypt_rlwe(&mut actual_pt, &prod_cts[size-k]);
        //dbg!(actual_pt);
        prod_cts[size-k].less_eq_than(1100, &mut buffers);
        let elapsed = start.elapsed();


        let mut out = PlaintextList::allocate(Scalar::zero(), ctx.plaintext_count());
        sk.binary_decrypt_rlwe(&mut out, &prod_cts[size-k]);
        //dbg!(out.clone());
        println!("Elapsed time: {:?}", elapsed);
        assert_eq!(*out.as_polynomial().get_monomial(MonomialDegree(0)).get_coefficient(), Scalar::one());
    }
```

XOR operations were computed using CMUX, and a current line of work is to optimize this computation. 

With this information in our hands, we can already quickly sketch scenarios and estimate time for the homomorphic evaluations. For example, using fast comparisons (98 micsecs) and 1000 additions (400 ms) for counting bits, and 1000 XORs (600 ms, the bottleneck). Total is 1 second.
 If after filtering we have smaller features, and consequently more comparisons, then we can already estimate the cost. Consider 5 features of 200 bits, then 5 comparisons. Meaning the cost is `5*98 micsecs + ~80ms + ~120 ms = 200.5 ms`. As a second example, using lots of small features, say 64 features of 16 bits (closer to what was explored in the underlying paper), then the cost is: `64*98 micsecs + 16*0.4 ms + 16*0.6 ms = 6.3 ms + 16 ms = 22.3 ms`.
 
 Hence it is possible to reduce orders of magnitude just by finding the best approach and using the right tools for each step. A qualitative conclusion is that low resolution matching or filtering could benefit a lot from fast comparisons. But as the size of the input grows this situation quickly changes. For thousands of bits, i.e. for high resolution matching, we may need to use a different approach, like a taylored construction that can take advantage of batching techniques in order to avoid the penalty that comes from the inherently sequential method proposed by Sorting Hat.
