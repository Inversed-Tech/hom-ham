


//use tfhe::high_level_api::integers::types::base::GenericInteger;
use tfhe::{ConfigBuilder, generate_keys, set_server_key, FheUint8};
use tfhe::prelude::*;

use std::time::{Instant, Duration};


/*fn popcount_8bit<FheUint8Parameters>(i: GenericInteger<FheUint8Parameters>) -> u8 {
    //assert!(i < 0x100);
    let mut num = i as u32;
    num = num - ((num >> 1) & 0x55);
    num = (num & 0x33) + ((num >> 2) & 0x33);
    num = ((num + (num >> 4)) & 0xF) * 0x11;
    (num >> 8) as u8
}*/


fn main() {
    let config = ConfigBuilder::all_disabled()
        .enable_default_integers()
        .build();

    // Client-side
    let (client_key, server_key) = generate_keys(config);

    let size = 1;
    let mut cvec1 = Vec::new();
    let mut cvec2 = Vec::new();
    //let mut cvec = Vec::new();
    let total_sum = 0;

    let mut c = FheUint8::encrypt(0u8, &client_key);
    let s1 = FheUint8::encrypt(1u8, &client_key);
    let s2 = FheUint8::encrypt(2u8, &client_key);
    let s4 = FheUint8::encrypt(4u8, &client_key);
    let s8 = FheUint8::encrypt(8u8, &client_key);
    for _ in 0..size {
        let c1 = FheUint8::encrypt(42u8, &client_key);  // TODO: make it random
        let c2 = FheUint8::encrypt(143u8, &client_key);
        cvec1.push(c1);
        cvec2.push(c2);
    }

    set_server_key(server_key);

    let start_time = Instant::now();
    for i in 0..size {
        let xor_res = cvec1[i].clone() ^ &cvec2[i];

        // inline popcount
        let mut num = xor_res;
        /*
        let popcount = |num: u32| -> u32 {
            let num = num - ((num >> s1) & 0x55);
            let num = (num & 0x33) + ((num >> s2) & 0x33);
            let num = ((num + (num >> s4)) & 0xF) * 0x11;
            num >> s8
        };
        */

        num = num.clone() - ((num >> s1.clone()) & 0x55);
        num = (num.clone() & 0x33) + ((num >> s2.clone()) & 0x33);
        num = ((num.clone() + (num >> s4.clone())) & 0xF) * 0x11;
        num = num >> s8.clone();


        c += num;
    }
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Elapsed time: {:?}", elapsed_time);
    dbg!(total_sum);
    let clear_total: u64 = FheUint8::decrypt(&c, &client_key);
    println!("{}", clear_total)


}




/*use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32};
use tfhe::integer::U256;
use rand::Rng;
use std::time::{Instant, Duration};
use tfhe::ServerKey;
use tfhe::boolean::prelude::Ciphertext;
use tfhe::shortint::prelude::PARAM_MESSAGE_2_CARRY_2;


use tfhe::{FheUint8};

use tfhe::boolean::prelude::*;

fn popcount_8bit(i: u8) -> u8 {
    //assert!(i < 0x100);
    let mut num = i as u32;
    num = num - ((num >> 1) & 0x55);
    num = (num & 0x33) + ((num >> 2) & 0x33);
    num = ((num + (num >> 4)) & 0xF) * 0x11;
    (num >> 8) as u8
}

fn main() {
// We generate a set of client/server keys, using the default parameters:
    //let config = ConfigBuilder::all_disabled()
    //    .enable_default_uint8()
    //    .build();

    let config = ConfigBuilder::default().build();

    println!("Before");
    let (client_key, server_key) = gen_keys(config);
    println!("Keys");

// We use the client secret key to encrypt a message:

    let size = 2;
    let mut cvec1 = Vec::new();
    let mut cvec2 = Vec::new();
    let mut cvec = Vec::new();
    let mut total_sum = 0;

    //let czero = client_key.encrypt(false);
    for _ in 0..size {
        let c1 = client_key.encrypt(42);  // TODO: make it random
        let c2 = client_key.encrypt(143);
        cvec1.push(c1);
        cvec2.push(c2);
    }
    println!("Input");

    //let clear_res: u32 = 0;
    //let mut res = FheUint32::try_encrypt(clear_res, &client_key)?;
    //let czero = client_key.encrypt(false);
    let start_time = Instant::now();
    for i in 0..size {
        //cvec.push(server_key.xor(&cvec1[i], &cvec2[i]));
        total_sum += popcount_8bit(server_key.xor(&cvec1[i], &cvec2[i]))
    }
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Elapsed time: {:?}", elapsed_time);
    dbg!(total_sum);
    let clear_total: u64 = client_key.decrypt(&total_sum);
    dbg!(clear_total)
}*/

