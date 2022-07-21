use std::error::Error;
use std::fs::{self,File,OpenOptions};
use std::path::Path;
#[cfg(feature = "concrete")]
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use colored::Colorize;

// Parmesan
use parmesan::*;
#[allow(unused_imports)]
use parmesan::ciphertexts::{ParmCiphertext, ParmCiphertextExt};

use parmesan::ParmesanUserovo;

#[cfg(feature = "pbs")]
use parmesan::cloudovo::pbs;
#[cfg(feature = "pbs")]
const PBS_N: usize = 10;

#[cfg(feature = "nn")]
use parmesan::cloudovo::neural_network::{NeuralNetwork, Perceptron, PercType};
use parmesan::ParmesanCloudovo;

#[allow(unused_imports)]
use parmesan::arithmetics::ParmArithmetics;

// Concrete v0.2.x
#[cfg(feature = "concrete")]
use concrete::*;
#[cfg(feature = "concrete")]
use concrete::prelude::*;

// timing measurements
extern crate chrono;
//~ use chrono::Utc;

fn main() {
    // run benchmark
    println!();
    simple_duration!(
        ["Benchmark: Parmesan vs. Concrete v0.2.x"],
        [
            let _x = bench();
        ]
    );

    println!();
}

fn bench() -> Result<(), Box<dyn Error>> {


    // =========================================================================
    //  Generate / Load Keys

    // not used at the moment
    #[cfg(not(feature = "sequential"))]
    println!("\n\n{}: {} threads\n", String::from("Parallel").bold().yellow(), rayon::current_num_threads());
    #[cfg(feature = "sequential")]
    println!("\n\n{}\n", String::from("Sequential").bold().yellow());

    // -------------------------------------------------------------------------
    //  Parmesan parameters & key setup

    let par = &params::PARM80__PI_5__D_22;   //  80    112    128
    simple_duration!(
        ["Setup Parmesan keys"],
        [
            // Userovo Scope & keys
            let pu = ParmesanUserovo::new(par)?;
            let pub_k = pu.export_pub_keys();

            // Cloudovo Scope
            let pc = ParmesanCloudovo::new(
                par,
                &pub_k,
            );
        ]
    );

    // -------------------------------------------------------------------------
    // Concrete parameters & key setup

    #[cfg(feature = "concrete")]
    let (_sek4,_puk4,_cfg4,_sek8,_puk8,_cfg8,_sek16,_puk16,_cfg16,_sek32,_puk32,_cfg32):
                (ClientKey,ServerKey,DynIntegerEncryptor,
                 ClientKey,ServerKey,DynIntegerEncryptor,
                 ClientKey,ServerKey,DynIntegerEncryptor,
                 ClientKey,ServerKey,DynIntegerEncryptor);
    #[cfg(feature = "concrete")]
    {
    // check consistency of features
    #[cfg(not(any(feature = "c4", feature = "c8", feature = "c16", feature = "c32")))]
    compile_error!("No precision selected for Concrete!");

    let concrete_key_path = Path::new("./keys/concrete-keys.key");
    // setup keys
    simple_duration!(
        ["Setup Concrete keys"],
        [
            (_sek4,_puk4,_cfg4,_sek8,_puk8,_cfg8,_sek16,_puk16,_cfg16,_sek32,_puk32,_cfg32) = if !concrete_key_path.is_file() {
                let mut u4_builder = ConfigBuilder::all_disabled();
                let cfg4 = u4_builder.add_integer_type(DynIntegerParameters {block_parameters: FheUint2Parameters::default().into(), num_block: 2});
                let mut u8_builder = ConfigBuilder::all_disabled();
                let cfg8 = u8_builder.add_integer_type(DynIntegerParameters {block_parameters: FheUint2Parameters::default().into(), num_block: 4});
                let mut u16_builder = ConfigBuilder::all_disabled();
                let cfg16 = u16_builder.add_integer_type(DynIntegerParameters {block_parameters: FheUint2Parameters::default().into(), num_block: 8});
                let mut u32_builder = ConfigBuilder::all_disabled();
                let cfg32 = u32_builder.add_integer_type(DynIntegerParameters {block_parameters: FheUint2Parameters::default().into(), num_block: 16});

                let (sek4,puk4) = generate_keys(u4_builder.build());
                let (sek8,puk8) = generate_keys(u8_builder.build());
                let (sek16,puk16) = generate_keys(u16_builder.build());
                let (sek32,puk32) = generate_keys(u32_builder.build());

                let keys_file = File::create(concrete_key_path).map(BufWriter::new)?;
                bincode::serialize_into(keys_file, &(&sek4,&puk4,&cfg4,&sek8,&puk8,&cfg8,&sek16,&puk16,&cfg16,&sek32,&puk32,&cfg32))?;

                (sek4,puk4,cfg4,sek8,puk8,cfg8,sek16,puk16,cfg16,sek32,puk32,cfg32)
            } else {
                let keys_file = File::open(concrete_key_path).map(BufReader::new)?;
                bincode::deserialize_from(keys_file)?
            };
        ]
    );
    }
    // choose the right one ..
    #[cfg(feature = "c4")]
    let (client_key, server_key, encryptor) = (_sek4, _puk4, _cfg4);
    #[cfg(feature = "c8")]
    let (client_key, server_key, encryptor) = (_sek4, _puk4, _cfg4);
    #[cfg(feature = "c16")]
    let (client_key, server_key, encryptor) = (_sek4, _puk4, _cfg4);
    #[cfg(feature = "c32")]
    let (client_key, server_key, encryptor) = (_sek4, _puk4, _cfg4);
    // .. and set as server key
    #[cfg(feature = "concrete")]
    set_server_key(server_key);
    //TODO setup a constant .. Concrete bit-len .. use in mul, squ, scm, decr check


    // =========================================================================
    //  Generate & Encrypt inputs

    // 4 random 31-word sequences of {-1,0,1}
    let a: Vec<i32> = vec![1,0,1,-1,-1,0,-1,1,1,-1,1,1,1,-1,-1,0,0,1,1,0,0,0,0,-1,0,0,0,0,0,-1,0,0,];
    let b: Vec<i32> = vec![-1,0,0,-1,1,1,-1,1,-1,0,0,1,0,1,1,0,0,0,-1,0,0,1,0,0,-1,0,-1,-1,-1,1,1,0,];
    let c: Vec<i32> = vec![-1,1,-1,1,-1,1,0,0,-1,0,-1,1,0,0,1,1,1,1,1,0,-1,0,0,-1,1,0,1,1,-1,-1,0,];
    let d: Vec<i32> = vec![1,0,1,0,0,1,0,-1,0,1,-1,0,0,0,-1,0,1,-1,1,1,0,0,-1,-1,0,0,1,1,1,1,0,];

    // pairs of random 4-, 8-, 16- and 32-word sequences of {-1,0,1}
    let a4:  Vec<i32> = vec![1,-1,-1,0,];
    let b4:  Vec<i32> = vec![0,0,1,-1,];
    let a8:  Vec<i32> = vec![1,1,-1,-1,0,0,-1,-1,];
    let b8:  Vec<i32> = vec![-1,0,1,0,0,-1,0,-1,];
    let c8:  Vec<i32> = vec![1,1,0,-1,0,1,-1,1,];
    let d8:  Vec<i32> = vec![-1,1,1,1,1,-1,-1,1,];
    let a16: Vec<i32> = vec![0,0,1,0,-1,1,-1,-1,0,1,1,0,0,1,-1,1,];
    let b16: Vec<i32> = vec![1,1,0,0,0,1,0,1,1,1,0,1,0,1,1,-1,];
    let a32: Vec<i32> = vec![-1,-1,1,0,-1,0,-1,0,1,-1,-1,0,1,-1,0,-1,0,-1,1,1,1,-1,1,-1,0,0,-1,0,0,1,1,0,];
    let b32: Vec<i32> = vec![1,-1,-1,-1,1,-1,1,-1,0,1,-1,0,1,0,1,0,-1,1,1,-1,1,-1,-1,0,0,-1,-1,0,-1,-1,-1,0,];

    // "random" scalars
    let _k: [i32; 6] = [    // optimal ASC*'s
        -161,               // /2/ [◖ 1 ◗, ◖ 5 =  1 + 1·2^2 ◗, ◖ 161 =  1 + 5·2^5 ◗]
        805,                // /3/ [◖ 1 ◗, ◖ 5 =  1 + 1·2^2 ◗, ◖ 25 =  5 + 5·2^2 ◗, ◖ 805 =  5 + 25·2^5 ◗]
        1173,               // /3/ [◖ 1 ◗, ◖ 17 =  1 + 1·2^4 ◗, ◖ 289 =  17 + 17·2^4 ◗, ◖ 1173 =  17 + 289·2^2 ◗]
        1209,               // /3/ [◖ 1 ◗, ◖ 31 = -1 + 1·2^5 ◗, ◖ 155 =  31 + 31·2^2 ◗, ◖ 1209 = -31 + 155·2^3 ◗]
        3195,               // /3/ [◖ 1 ◗, ◖ 5 =  1 + 1·2^2 ◗, ◖ 25 =  5 + 5·2^2 ◗, ◖ 3195 = -5 + 25·2^7 ◗]
        3813,               // /3/ [◖ 1 ◗, ◖ 31 = -1 + 1·2^5 ◗, ◖ 961 = -31 + 31·2^5 ◗, ◖ 3813 = -31 + 961·2^2 ◗]
    ];

    // convert to actual numbers
    let a_val = encryption::convert(&a)?;
    let b_val = encryption::convert(&b)?;
    let c_val = encryption::convert(&c)?;
    let d_val = encryption::convert(&d)?;

    let a4_val  = encryption::convert(&a4 )?;
    let b4_val  = encryption::convert(&b4 )?;
    let a8_val  = encryption::convert(&a8 )?;
    let b8_val  = encryption::convert(&b8 )?;
    let c8_val  = encryption::convert(&c8 )?;
    let d8_val  = encryption::convert(&d8 )?;
    let a16_val = encryption::convert(&a16)?;
    let b16_val = encryption::convert(&b16)?;
    let a32_val = encryption::convert(&a32)?;
    let b32_val = encryption::convert(&b32)?;

    // print inputs
    println!("\n{}:\n", String::from("Inputs").bold().yellow());
    println!("a   = {:12}", a_val);
    println!("b   = {:12}", b_val);
    println!("c   = {:12}", c_val);
    println!("d   = {:12}\n", d_val);

    println!("a4  = {:12}", a4_val );
    println!("b4  = {:12}", b4_val );
    println!("a8  = {:12}", a8_val );
    println!("b8  = {:12}", b8_val );
    println!("c8  = {:12}", c8_val );
    println!("d8  = {:12}", d8_val );
    println!("a16 = {:12}", a16_val);
    println!("b16 = {:12}", b16_val);
    println!("a32 = {:12}", a32_val);
    println!("b32 = {:12}\n", b32_val);

    // Parmesan encrypt values
    let _p_ca = pu.encrypt_vec(&a)?;
    let _p_cb = pu.encrypt_vec(&b)?;
    let _p_cc = pu.encrypt_vec(&c)?;
    let _p_cd = pu.encrypt_vec(&d)?;

    let _p_ca4  = pu.encrypt_vec(&a4 )?;
    let _p_cb4  = pu.encrypt_vec(&b4 )?;
    let _p_ca8  = pu.encrypt_vec(&a8 )?;
    let _p_cb8  = pu.encrypt_vec(&b8 )?;
    let _p_cc8  = pu.encrypt_vec(&c8 )?;
    let _p_cd8  = pu.encrypt_vec(&d8 )?;
    let _p_ca16 = pu.encrypt_vec(&a16)?;
    let _p_cb16 = pu.encrypt_vec(&b16)?;
    let _p_ca32 = pu.encrypt_vec(&a32)?;
    let _p_cb32 = pu.encrypt_vec(&b32)?;

    // Concrete encrypt values
    #[cfg(feature = "concrete")]
    let (_c_ca, _c_cb, _c_cc, _c_cd);
    #[cfg(feature = "concrete")]
    {
    _c_ca = encryptor.encrypt(a_val as u64, &client_key);
    _c_cb = encryptor.encrypt(b_val as u64, &client_key);
    _c_cc = encryptor.encrypt(c_val as u64, &client_key);
    _c_cd = encryptor.encrypt(d_val as u64, &client_key);
    }


    // =========================================================================
    //  Programmable Bootstrapping

    #[cfg(feature = "pbs")]
    let mut _p_c_pbs_id_a = ParmCiphertext::single(_p_ca[0].clone());
    #[cfg(feature = "pbs")]
    {
    simple_duration!(
        ["PBS {}x", PBS_N],
        [
        measure_duration!(
            ["Pbs {}x", PBS_N],
            [
                for _ in 0..PBS_N {
                    _p_c_pbs_id_a = ParmCiphertext::single(pbs::id__pi_5(&pc.pub_keys, &_p_ca[0])?);
                }
            ]
        );
        ]
    );
    }


    // =========================================================================
    //  Addition

    #[cfg(feature = "add")]
    let (p_add_a_b, p_sub_c_d, p_add_ab_cnd);
    #[cfg(all(feature = "add", feature = "concrete"))]
    let (c_add_a_b, c_sub_c_d, c_add_ab_cnd);
    #[cfg(feature = "add")]
    {
    // first level addition/subtraction:   a + b   ,   c - d
    simple_duration!(
        ["Parmesan::Add (1st lvl)"],
        [
            p_add_a_b = ParmArithmetics::add(&pc, &_p_ca, &_p_cb);
        ]
    );
    simple_duration!(
        ["Parmesan::Sub (1st lvl)"],
        [
            p_sub_c_d = ParmArithmetics::sub(&pc, &_p_cc, &_p_cd);
        ]
    );

    // second level addition:   (a+b) + (c-d)
    simple_duration!(
        ["Parmesan::Add (2nd lvl, no refresh)"],
        [
            p_add_ab_cnd = ParmArithmetics::add_noisy(&pc, &p_add_a_b, &p_sub_c_d);
        ]
    );

    #[cfg(feature = "concrete")]
    {
    // first level addition/subtraction:   a + b   ,   c - d
    simple_duration!(
        ["Concrete::Add (1st lvl)"],
        [
            c_add_a_b = _c_ca.clone() + _c_cb.clone();
        ]
    );
    simple_duration!(
        ["Concrete::Sub (1st lvl)"],
        [
            c_sub_c_d = _c_cc.clone() - _c_cd.clone();
        ]
    );

    // second level addition:   (a+b) + (c-d)
    simple_duration!(
        ["Concrete::Add (2nd lvl, no refresh)"],
        [
            c_add_ab_cnd = c_add_a_b.clone() + c_sub_c_d.clone();
        ]
    );
    }
    }


    // =========================================================================
    //  Scalar Multiplication

    #[cfg(feature = "scm")]
    let mut p_scm16_a: Vec<ParmCiphertext> = Vec::new();
    #[cfg(all(feature = "add", feature = "concrete"))]
    let mut c_scm16_a = Vec::new();
    #[cfg(feature = "scm")]
    {
    // scalar multiplication of 16-word
    for ki in _k {
        simple_duration!(
            ["Parmesan::Sc. Mul (16-word, by {})", ki],
            [
                p_scm16_a.push(ParmArithmetics::scalar_mul(&pc, ki, &_p_ca16));
            ]
        );
    }

    #[cfg(feature = "concrete")]
    {
    // scalar multiplication
    for ki in _k {
        simple_duration!(
            ["Concrete::Sc. Mul (k-word, by {})", ki],
            [
                let c_scmi = ki as u64 * _c_ca.clone();
                c_scm16_a.push(c_scmi);
            ]
        );
    }
    }
    }


    // =========================================================================
    //  Signum

    #[cfg(feature = "sgn")]
    let (p_sgn_a, p_sgn_ab);
    #[cfg(feature = "sgn")]
    {
    simple_duration!(
        ["Parmesan::Sgn a"],
        [
            p_sgn_a = ParmArithmetics::sgn(&pc, &_p_ca);
        ]
    );

    simple_duration!(
        ["Parmesan::Sgn (a+b)"],
        [
            p_sgn_ab = ParmArithmetics::sgn(&pc, &p_add_a_b);
        ]
    );

    //TODO Concrete signum?
    }


    // =========================================================================
    //  Rounding

    #[cfg(feature = "round")]
    let p_round_a: ParmCiphertext;
    #[cfg(feature = "round")]
    const ROUND_IDX: usize = 6;
    #[cfg(feature = "round")]
    {
    simple_duration!(
        ["Parmesan::Round"],
        [
            p_round_a = ParmArithmetics::round_at(&pc, &_p_ca, ROUND_IDX);
        ]
    );

    //TODO Concrete rounding?
    }


    // =========================================================================
    //  Maximum

    #[cfg(feature = "max")]
    let (p_max_a_b, p_max_c_d, p_max_mab_mcd);
    #[cfg(feature = "max")]
    {
    // first level maximum
    simple_duration!(
        ["Parmesan::Max (1st lvl)"],
        [
            p_max_a_b = ParmArithmetics::max(&pc, &_p_ca, &_p_cb);
        ]
    );
    simple_duration!(
        ["Parmesan::Max (1st lvl)"],
        [
            p_max_c_d = ParmArithmetics::max(&pc, &_p_cc, &_p_cd);
        ]
    );

    // second level maximum
    simple_duration!(
        ["Parmesan::Max (2nd lvl)"],
        [
            p_max_mab_mcd = ParmArithmetics::max(&pc, &p_max_a_b, &p_max_c_d);
        ]
    );

    //TODO Concrete maximum?
    }


    // =========================================================================
    //  Multiplication

    #[cfg(feature = "mul_light")]
    let (p_mul4_a_b, p_mul8_a_b);
    #[cfg(feature = "mul")]
    let (p_mul16_a_b, p_mul32_a_b);

    #[cfg(all(feature = "mul_light", feature = "concrete"))]
    let c_mul_a_b;

    #[cfg(feature = "mul_light")]
    {
    // 4-word multiplication -> 8-word
    simple_duration!(
        ["Parmesan::Mul (4-word)"],
        [
            p_mul4_a_b = ParmArithmetics::mul(&pc, &_p_ca4, &_p_cb4);
        ]
    );

    // 8-word multiplication -> 16-word
    simple_duration!(
        ["Parmesan::Mul (8-word)"],
        [
            p_mul8_a_b = ParmArithmetics::mul(&pc, &_p_ca8, &_p_cb8);
        ]
    );

    #[cfg(all(feature = "concrete", any(feature = "c4", feature = "c8")))]
    {
    // multiplication
    simple_duration!(
        ["Concrete::Mul (k-word)"],
        [
            c_mul_a_b = _c_ca.clone() * _c_cb.clone();
        ]
    );
    }
    }

    #[cfg(feature = "mul")]
    {
    // 16-word multiplication -> 33-word
    simple_duration!(
        ["Parmesan::Mul (16-word)"],
        [
            p_mul16_a_b = ParmArithmetics::mul(&pc, &_p_ca16, &_p_cb16);
        ]
    );

    // 32-word multiplication -> 66-word (if there happen to be zeros at the leading positions after decryption, it PASSes)
    simple_duration!(
        ["Parmesan::Mul (32-word)"],
        [
            p_mul32_a_b = ParmArithmetics::mul(&pc, &_p_ca32, &_p_cb32);
        ]
    );

    #[cfg(all(feature = "concrete", any(feature = "c16", feature = "c32")))]
    {
    // multiplication
    simple_duration!(
        ["Concrete::Mul (K-word)"],
        [
            c_mul_a_b = _c_ca.clone() * _c_cb.clone();
        ]
    );
    }
    }


    // =========================================================================
    //  Squaring

    #[cfg(feature = "squ_light")]
    let (p_squ_a4, p_squ_a8);
    #[cfg(feature = "squ")]
    let (p_squ_a16, p_squ_a32);

    #[cfg(all(feature = "squ_light", feature = "concrete"))]
    let c_squ_a_b;

    #[cfg(feature = "squ_light")]
    {
    // 4-word squaring -> 9-word (?)
    simple_duration!(
        ["Parmesan::Squ (4-word)"],
        [
            p_squ_a4 = ParmArithmetics::squ(&pc, &_p_ca4);
        ]
    );

    // 8-word squaring -> 18-word (?)
    simple_duration!(
        ["Parmesan::Squ (8-word)"],
        [
            p_squ_a8 = ParmArithmetics::squ(&pc, &_p_ca8);
        ]
    );

    #[cfg(all(feature = "concrete", any(feature = "c4", feature = "c8")))]
    {
    // squaring
    simple_duration!(
        ["Concrete::Squ (k-word)"],
        [
            c_squ_a_b = _c_ca.clone() * _c_ca.clone();
        ]
    );
    }
    }

    #[cfg(feature = "squ")]
    {
    // 16-word squaring -> 35-word (?)
    simple_duration!(
        ["Parmesan::Squ (16-word)"],
        [
            p_squ_a16 = ParmArithmetics::squ(&pc, &_p_ca16);
        ]
    );

    // 32-word squaring -> 68-word (?)
    simple_duration!(
        ["Parmesan::Squ (32-word)"],
        [
            p_squ_a32 = ParmArithmetics::squ(&pc, &_p_ca32);
        ]
    );

    #[cfg(all(feature = "concrete", any(feature = "c16", feature = "c32")))]
    {
    // squaring
    simple_duration!(
        ["Concrete::Squ (K-word)"],
        [
            c_squ_a_b = _c_ca.clone() * _c_ca.clone();
        ]
    );
    }
    }


    // =========================================================================
    //  NN Evaluation

    #[cfg(feature = "nn")]
    let p_nn_out: Vec<ParmCiphertext>;
    #[cfg(feature = "nn")]
    let m_nn_out: Vec<i64>;
    #[cfg(feature = "nn")]
    {
    // evaluation of a simple NN
    let c_nn_in           = vec![   _p_ca8,    _p_cb8,    _p_cc8,    _p_cd8];   // .clone()
    let m_nn_in: Vec<i64> = vec![a8_val, b8_val, c8_val, d8_val];   // .clone()

    let _nn = NeuralNetwork {
        layers: vec![
            vec![
                Perceptron {
                    t: PercType::MAX,
                    w: vec![1,-2,-2,],
                    b: 2,
                },
                Perceptron {
                    t: PercType::LIN,
                    w: vec![1,3,-1,],
                    b: -5,
                },
                Perceptron {
                    t: PercType::ACT,
                    w: vec![1,3,-1,],
                    b: 3,
                },
            ],
        ],
        n_inputs: 3,
    };

    simple_duration!(
        ["NN Eval (?? BS)"],
        [
            p_nn_out = demo_nn().eval(&pc, &c_nn_in);
        ]
    );
    m_nn_out = demo_nn().eval(&pc, &m_nn_in);
    }


    // =========================================================================
    //  Decrypt & Check Correctness

    #[allow(unused_mut)]
    let mut summary_text = format!("\n{}:", String::from("Results").bold().yellow());

    // decrypt plain Concrete's ciphertexts
    summary_text = format!("{}\n\nConcrete Decryption:", summary_text);
    #[cfg(feature = "concrete")]
    {
    let c_a_v  = _c_ca.decrypt(&client_key);
    summary_text = format!("{}\ndecr(encr(a)) = {:12} :: {} (exp. {})", summary_text,
                            c_a_v,
                            if c_a_v == a4_val as u64 & ((1 << 4) - 1) {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            a4_val as u64 & ((1 << 4) - 1)
    );
    }

    // decrypt & verify all results
    #[cfg(feature = "pbs")] // -------------------------------------------------
    {
    let pbs_id_a0   = pu.decrypt(&_p_c_pbs_id_a)?;
    summary_text = format!("{}\n\nProgrammable Bootstrapping:", summary_text);
    summary_text = format!("{}\nid(a[0])      = {:12} :: {} (exp. {})", summary_text,
                            pbs_id_a0,
                            if a[0] as i64 == pbs_id_a0 {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            a[0]
    );
    }

    #[cfg(feature = "add")] // -------------------------------------------------
    {
    let add_a_b     = pu.decrypt(&p_add_a_b     )?;
    let sub_c_d     = pu.decrypt(&p_sub_c_d     )?;
    let add_ab_cnd  = pu.decrypt(&p_add_ab_cnd  )?;
    summary_text = format!("{}\n\nAddition:", summary_text);
    summary_text = format!("{}\na + b         = {:12} :: {} (exp. {})", summary_text,
                            add_a_b,
                            if add_a_b == a_val + b_val {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            a_val + b_val
    );
    summary_text = format!("{}\nc - d         = {:12} :: {} (exp. {})", summary_text,
                            sub_c_d,
                            if sub_c_d == c_val - d_val {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            c_val - d_val
    );
    summary_text = format!("{}\n(a+b) + (c-d) = {:12} :: {} (exp. {})", summary_text,
                            add_ab_cnd,
                            if add_ab_cnd == add_a_b + sub_c_d {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            add_a_b + sub_c_d
    );
    #[cfg(feature = "concrete")]
    {
    //TODO
    }
    }

    #[cfg(feature = "sgn")] // -------------------------------------------------
    {
    let sgn_a       = pu.decrypt(&p_sgn_a       )?;
    let sgn_ab      = pu.decrypt(&p_sgn_ab   )?;
    summary_text = format!("{}\n\nSignum:", summary_text);
    summary_text = format!("{}\nsgn(a)        = {:12} :: {} (exp. {})", summary_text,
                            sgn_a,
                            if sgn_a == a_val.signum() {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            a_val.signum()
    );
    summary_text = format!("{}\nsgn(a+b+c-d)  = {:12} :: {} (exp. {})", summary_text,
                            sgn_ab,
                            if sgn_ab == (a_val + b_val + c_val - d_val).signum() {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            (a_val + b_val + c_val - d_val).signum()
    );
    }

    #[cfg(feature = "round")] // -----------------------------------------------
    {
    let round_a       = pu.decrypt(&p_round_a       )?;
    // complex rounding of f64 in Rust:        sgn * (        abs          divide                     round                       )
    let round_a_val   = if a_val < 0 {-1} else {1} * (((a_val.abs() as f64 / (1 << ROUND_IDX) as f64).round() as u64) << ROUND_IDX) as i64;
    summary_text = format!("{}\n\nRounding:", summary_text);
    summary_text = format!("{}\nround(a)      = {:12} :: {} (exp. {})", summary_text,
                            round_a,
                            if round_a == round_a_val {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            round_a_val
    );
    }

    #[cfg(feature = "max")] // -------------------------------------------------
    {
    let max_a_b     = pu.decrypt(&p_max_a_b     )?;
    let max_c_d     = pu.decrypt(&p_max_c_d     )?;
    let max_mab_mcd = pu.decrypt(&p_max_mab_mcd )?;
    summary_text = format!("{}\n\nMaximum:", summary_text);
    summary_text = format!("{}\nmax{{a, b}}     = {:12} :: {} (exp. {})", summary_text,
                            max_a_b,
                            if max_a_b == std::cmp::max(a_val, b_val) {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            std::cmp::max(a_val, b_val)
    );
    summary_text = format!("{}\nmax{{c, d}}     = {:12} :: {} (exp. {})", summary_text,
                            max_c_d,
                            if max_c_d == std::cmp::max(c_val, d_val) {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            std::cmp::max(c_val, d_val)
    );
    summary_text = format!("{}\nmax{{m_ab,m_cd}}= {:12} :: {} (exp. {})", summary_text,
                            max_mab_mcd,
                            if max_mab_mcd == std::cmp::max(max_a_b, max_c_d) {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            std::cmp::max(max_a_b, max_c_d)
    );
    }

    #[cfg(feature = "mul_light")] // -------------------------------------------
    {
    let mul4_a_b    = pu.decrypt(&p_mul4_a_b    )?;
    let mul8_a_b    = pu.decrypt(&p_mul8_a_b    )?;
    summary_text = format!("{}\n\nMultiplication:", summary_text);
    summary_text = format!("{}\na4 × b4       = {:22} :: {} (exp. {})", summary_text,
                            mul4_a_b,
                            if mul4_a_b == a4_val * b4_val {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            a4_val * b4_val
    );
    summary_text = format!("{}\na8 × b8       = {:22} :: {} (exp. {})", summary_text,
                            mul8_a_b,
                            if mul8_a_b == a8_val * b8_val {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            a8_val * b8_val
    );
    #[cfg(feature = "concrete")]
    {
    let c_mul_a_b_v = c_mul_a_b.decrypt(&client_key);
    summary_text = format!("{}\na × b (Conc)  = {:22} :: {} (exp. {})", summary_text,
                            c_mul_a_b_v,
                            if c_mul_a_b_v == a_val as u64 * b_val as u64 {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            a_val as u64 * b_val as u64
    );
    }
    }
    #[cfg(feature = "mul")] // -------------------------------------------------
    {
    let mul16_a_b   = pu.decrypt(&p_mul16_a_b   )?;
    let mul32_a_b   = pu.decrypt(&p_mul32_a_b   )?;
    summary_text = format!("{}\na16 × b16     = {:22} :: {} (exp. {})", summary_text,
                            mul16_a_b,
                            if mul16_a_b == a16_val * b16_val {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            a16_val * b16_val
    );
    summary_text = format!("{}\na32 × b32     = {:22} :: {} (exp. {})", summary_text,
                            mul32_a_b,
                            if mul32_a_b == a32_val * b32_val {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            a32_val * b32_val
    );
    }

    #[cfg(feature = "squ_light")] // -------------------------------------------
    {
    let squ_a4      = pu.decrypt(&p_squ_a4      )?;
    let squ_a8      = pu.decrypt(&p_squ_a8      )?;
    summary_text = format!("{}\n\nSquaring:", summary_text);
    summary_text = format!("{}\na4 ^ 2        = {:22} :: {} (exp. {})", summary_text,
                            squ_a4,
                            if squ_a4 == a4_val * a4_val {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            a4_val * a4_val
    );
    summary_text = format!("{}\na8 ^ 2        = {:22} :: {} (exp. {})", summary_text,
                            squ_a8,
                            if squ_a8 == a8_val * a8_val {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            a8_val * a8_val
    );
    #[cfg(feature = "concrete")]
    {
    //TODO
    }
    }
    #[cfg(feature = "squ")] // -------------------------------------------------
    {
    let squ_a16     = pu.decrypt(&p_squ_a16     )?;
    let squ_a32     = pu.decrypt(&p_squ_a32     )?;
    summary_text = format!("{}\na16 ^ 2       = {:22} :: {} (exp. {})", summary_text,
                            squ_a16,
                            if squ_a16 == a16_val * a16_val {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            a16_val * a16_val
    );
    summary_text = format!("{}\na32 ^ 2       = {:22} :: {} (exp. {})", summary_text,
                            squ_a32,
                            if squ_a32 == a32_val * a32_val {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            a32_val * a32_val
    );
    }

    #[cfg(feature = "scm")] // -------------------------------------------------
    {
    let mut scm16_a: Vec<i64> = Vec::new();
    for ci in p_scm16_a {
        scm16_a.push(pu.decrypt(&ci)?);
    }
    summary_text = format!("{}\n\nScalar Multiplication:", summary_text);
    for (ki, scmi) in _k.iter().zip(scm16_a.iter()) {
        summary_text = format!("{}\n{:7} × a16 = {:12} :: {} (exp. {})", summary_text,
                                ki, scmi,
                                if *scmi == (*ki as i64) * a16_val {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                                (*ki as i64) * a16_val
        );
    }
    #[cfg(feature = "concrete")]
    {
    //TODO
    }
    }

    #[cfg(feature = "nn")] // --------------------------------------------------
    {
    let mut nn_out_homo: Vec<i64> = Vec::new();
    for ci in p_nn_out {
        nn_out_homo.push(pu.decrypt(&ci)?);
    }
    summary_text = format!("{}\n\nNeural Network Evaluation:", summary_text);
    summary_text = format!("{}\nNN(a8, b8, c8, d8) = {:?} :: {} (exp. {:?})", summary_text,
                            nn_out_homo,
                            if nn_out_homo == m_nn_out {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            m_nn_out
    );
    }

    println!("{}\n", summary_text);

    Ok(())
}
