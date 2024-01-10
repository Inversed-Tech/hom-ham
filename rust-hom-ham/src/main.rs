use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32};
use tfhe::integer::U256;
use rand::Rng;
use std::time::{Instant, Duration};
use tfhe::ServerKey;
use tfhe::boolean::prelude::Ciphertext;
use tfhe::shortint::prelude::PARAM_MESSAGE_2_CARRY_2;


use tfhe::boolean::prelude::*;

/*fn fhe_hamming_distance(
    sks: ServerKey,
    x: &[Ciphertext],
    y: &[Ciphertext],
) -> Result<Ciphertext, Box<dyn std::error::Error>> {
    // TODO: - use checked_bitxor and checked_add
    // TODO: - don't use unwrap
    Ok(x.iter()
        .zip(y.iter())
        .map(|(l, r)| sks.xor(l, r))
        .reduce(|l, r| sks.checked_add(&l, &r))
        .unwrap())
}*/

fn main() {
// We generate a set of client/server keys, using the default parameters:
    //let (client_key, server_key) = gen_keys();
    let (client_key, server_key) = gen_keys();

// We use the client secret key to encrypt a message:

    let size = 1000;
    let mut cvec1 = Vec::new();
    let mut cvec2 = Vec::new();
    let mut cvec = Vec::new();

    //let czero = client_key.encrypt(false);
    for _ in 0..size {
        let c1 = client_key.encrypt(true);
        let c2 = client_key.encrypt(true);
        cvec1.push(c1);
        cvec2.push(c2);
    }

    //let clear_res: u32 = 0;
    //let mut res = FheUint32::try_encrypt(clear_res, &client_key)?;
    let czero = client_key.encrypt(false);
    let start_time = Instant::now();
    for i in 0..size {
        cvec.push(server_key.xor(&cvec1[i], &cvec2[i]));
        //server_key.and(czero, server_key.xor(&cvec1[i], &cvec2[i]));
    }
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Elapsed time: {:?}", elapsed_time);
    //let ct_1 = client_key.encrypt(true);
    //let ct_2 = client_key.encrypt(true);

// We use the server public key to execute the NOT gate:
    //let ct_not = server_key.not(&ct_1);
    //let ct_xor = server_key.xor(&ct_1, &ct_2);

// We use the client key to decrypt the output of the circuit:
    //let output1 = client_key.decrypt(&ct_not);
    //let output = client_key.decrypt(&ct_xor);
    //assert_eq!(output1, false);
    //assert_eq!(output2, false);
}

/*use tfhe::integer::gen_keys_radix;
use tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2_KS_PBS;
fn main() {
    let size = 4;
    let (cks, sks) = gen_keys_radix(PARAM_MESSAGE_2_CARRY_2_KS_PBS, size);
    let msg1 = 41;
    let msg2 = 101;
    let ct1 = cks.encrypt(msg1);
    let ct2 = cks.encrypt(msg2);
    let ct_res = sks.unchecked_bitxor(&ct1, &ct2);
    /*match ct_res {
        Err(x) => panic!("{:?}", x),
        Ok(y) => {
            let clear: u64 = cks.decrypt(&y);
            assert_eq!(msg1 ^ msg2, clear);
        }
    }*/
}*/




/*fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut rng = rand::thread_rng();

    let config = ConfigBuilder::all_disabled().enable_default_integers().build();
    let (keys, server_keys) = generate_keys(config);
    set_server_key(server_keys);


    let clear_a = U256::from([
        14069555808489703714,
        3908590842307590452,
        9855978718424440147,
        13896218366640697882,
    ]);
    let clear_b = U256::from([
        14069555808489703714,
        3908590842307590452,
        9855978718424440147,
        13896218366640697882,
    ]);
    let clear_c = U256::from([
        14069555808489703714,
        3908590842307590452,
        9855978718424440147,
        13896218366640697882,
    ]);
    //let clear_a = 164;
    //let clear_b = 212;

    //let clear_a: u32 = rng.gen();
    //let clear_b: u32 = rng.gen();
    //let clear_c: u32 = rng.gen();
    let clear_d: u32 = 0;

    let mut a = FheUint256::try_encrypt(clear_a, &keys)?;
    let mut b = FheUint256::try_encrypt(clear_b, &keys)?;
    let mut c = FheUint256::try_encrypt(clear_c, &keys)?;
    let mut d = FheUint256::try_encrypt(clear_d, &keys)?;

    //a = a ^ &b;
    //b = b ^ &a;
    //a = a ^ &b;


    let start_time = Instant::now();
    for _ in 0..4 {
        d += (a.clone() ^ &b) & c.clone();
    }
    //let res = fhe_hamming_distance(server_keys, &[a, b], &[a, b]);
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Elapsed time: {:?}", elapsed_time);


    let dec_a: U256 = a.decrypt(&keys);
    let dec_b: U256 = b.decrypt(&keys);
    let dec_c: U256 = c.decrypt(&keys);
    let dec_d: U256 = d.decrypt(&keys);

    dbg!(dec_a);
    dbg!(dec_b);
    dbg!(dec_c);
    dbg!(dec_d);

    // We homomorphically swapped values using bitwise operations
    //assert_eq!(dec_a, clear_b);
    //assert_eq!(dec_b, clear_a);

    Ok(())
}*/




/*fn main() {
    use tfhe::boolean::gen_keys;
    use tfhe::boolean::prelude::*;

    let (mut client_key, mut server_key) = gen_keys();
    let ct_1 = client_key.encrypt(true);
    let ct_2 = client_key.encrypt(false);
    let ct_3 = server_key.not(&ct_2);
    let ct_4 = server_key.and(&ct_1, &ct_2);
    let ct_5 = server_key.nand(&ct_3, &ct_4);
    let ct_6 = server_key.mux(&ct_5, &ct_3, &ct_4);
    let output_1 = client_key.decrypt(&ct_6);
    assert_eq!(output_1, true);
    let ct_7 = server_key.and(&ct_6, true);
    let output_2 = client_key.decrypt(&ct_7);
    assert_eq!(output_2, true);
    let ct_8 = server_key.trivial_encrypt(false);
    let ct_9 = server_key.mux(&ct_7, &ct_3, &ct_8);
    let output_3 = client_key.decrypt(&ct_9);
    assert_eq!(output_3, true);
}*/
