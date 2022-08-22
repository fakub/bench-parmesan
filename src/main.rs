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
use parmesan::ciphertexts::{ParmCiphertext, ParmCiphertextImpl};

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

#[cfg(feature = "4bit")]
pub const BITLEN: usize = 4;
#[cfg(feature = "8bit")]
pub const BITLEN: usize = 8;
#[cfg(feature = "16bit")]
pub const BITLEN: usize = 16;
#[cfg(feature = "32bit")]
pub const BITLEN: usize = 32;

fn main() {
    // run benchmark
    println!();
    simple_duration!(
        ["Benchmark: Parmesan vs. Concrete v0.2 ({}-bit inputs)", BITLEN],
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

    let par = &params::PAR_CNCR_V0_2__M2_C3;
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
    let concrete_key_path = Path::new("./keys/concrete-keys-encryptors__4-8-16-32.key");
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
    #[cfg(all(feature = "concrete", feature = "4bit"))]
    let (client_key, server_key, encryptor) = (_sek4, _puk4, _cfg4);
    #[cfg(all(feature = "concrete", feature = "8bit"))]
    let (client_key, server_key, encryptor) = (_sek8, _puk8, _cfg8);
    #[cfg(all(feature = "concrete", feature = "16bit"))]
    let (client_key, server_key, encryptor) = (_sek16, _puk16, _cfg16);
    #[cfg(all(feature = "concrete", feature = "32bit"))]
    let (client_key, server_key, encryptor) = (_sek32, _puk32, _cfg32);
    // .. and set as server key
    #[cfg(feature = "concrete")]
    set_server_key(server_key);


    // =========================================================================
    //  Generate & Encrypt inputs

    //~ // 4 random 31-bit sequences of {-1,0,1}
    //~ vec![1,0,1,-1,-1,0,-1,1,1,-1,1,1,1,-1,-1,0,0,1,1,0,0,0,0,-1,0,0,0,0,0,-1,0,0,];
    //~ vec![-1,0,0,-1,1,1,-1,1,-1,0,0,1,0,1,1,0,0,0,-1,0,0,1,0,0,-1,0,-1,-1,-1,1,1,0,];
    //~ vec![-1,1,-1,1,-1,1,0,0,-1,0,-1,1,0,0,1,1,1,1,1,0,-1,0,0,-1,1,0,1,1,-1,-1,0,];
    //~ vec![1,0,1,0,0,1,0,-1,0,1,-1,0,0,0,-1,0,1,-1,1,1,0,0,-1,-1,0,0,1,1,1,1,0,];

    // pairs of random 4-, 8-, 16- and 32-bit sequences of {-1,0,1}
    let (a, b, c, d): (Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>);

    #[cfg(feature = "4bit")]
    {
    a = vec![1,-1,-1,0,];
    b = vec![0,0,1,-1,];
    c = vec![-1,0,-1,0,];
    d = vec![1,0,-1,1,];
    }
    #[cfg(feature = "8bit")]
    {
    a = vec![1,1,-1,-1,0,0,-1,-1,];
    b = vec![-1,0,1,0,0,-1,0,-1,];
    c = vec![1,1,0,-1,0,1,-1,1,];
    d = vec![-1,1,1,1,1,-1,-1,1,];
    }
    #[cfg(feature = "16bit")]
    {
    a = vec![0,0,1,0,-1,1,-1,-1,0,1,1,0,0,1,-1,1,];
    b = vec![1,1,0,0,0,1,0,1,1,1,0,1,0,1,1,-1,];
    c = vec![0,-1,0,-1,0,1,-1,-1,0,1,-1,0,-1,0,-1,1,];
    d = vec![-1,1,-1,1,-1,1,0,0,-1,0,-1,1,0,0,1,1,];
    }
    #[cfg(feature = "32bit")]
    {
    a = vec![-1,-1,1,0,-1,0,-1,0,1,-1,-1,0,1,-1,0,-1,0,-1,1,1,1,-1,1,-1,0,0,-1,0,0,1,1,0,];
    b = vec![1,-1,-1,-1,1,-1,1,-1,0,1,-1,0,1,0,1,0,-1,1,1,-1,1,-1,-1,0,0,-1,-1,0,-1,-1,-1,0,];
    c = vec![-1,1,-1,1,-1,1,0,0,-1,0,-1,1,0,0,1,1,1,1,1,0,-1,0,0,-1,1,0,1,1,-1,-1,0,1,];
    d = vec![1,0,1,0,0,1,0,-1,0,1,-1,0,0,0,-1,0,1,-1,1,1,0,0,-1,-1,0,0,1,1,1,1,0,-1,];
    }

    // "random" scalars
    let _k: [i32; 5] = [    // optimal ASC*'s
        // -161,               // /2/ [◖ 1 ◗, ◖ 5 =  1 + 1·2^2 ◗, ◖ 161 =  1 + 5·2^5 ◗]
        4095,               // /1/ [◖ 1 ◗, ◖ 4095 = -1 + 1·2^12 ◗]
        4096,               // /0/ [◖ 1·2^12 ◗ ]
        4097,               // /1/ [◖ 1 ◗, ◖ 4097 =  1 + 1·2^12 ◗]
        805,                // /3/ [◖ 1 ◗, ◖ 5 =  1 + 1·2^2 ◗, ◖ 25 =  5 + 5·2^2 ◗, ◖ 805 =  5 + 25·2^5 ◗]
        //~ 1173,               // /3/ [◖ 1 ◗, ◖ 17 =  1 + 1·2^4 ◗, ◖ 289 =  17 + 17·2^4 ◗, ◖ 1173 =  17 + 289·2^2 ◗]
        //~ 1209,               // /3/ [◖ 1 ◗, ◖ 31 = -1 + 1·2^5 ◗, ◖ 155 =  31 + 31·2^2 ◗, ◖ 1209 = -31 + 155·2^3 ◗]
        3195,               // /3/ [◖ 1 ◗, ◖ 5 =  1 + 1·2^2 ◗, ◖ 25 =  5 + 5·2^2 ◗, ◖ 3195 = -5 + 25·2^7 ◗]
        //~ 3813,               // /3/ [◖ 1 ◗, ◖ 31 = -1 + 1·2^5 ◗, ◖ 961 = -31 + 31·2^5 ◗, ◖ 3813 = -31 + 961·2^2 ◗]
    ];

    // convert to actual numbers
    let a_val = encryption::convert_from_vec(&a)?;
    let b_val = encryption::convert_from_vec(&b)?;
    let c_val = encryption::convert_from_vec(&c)?;
    let d_val = encryption::convert_from_vec(&d)?;

    // print inputs
    println!("\n{}:\n", String::from("Inputs").bold().yellow());
    println!("a   = {:12}", a_val);
    println!("b   = {:12}", b_val);
    println!("c   = {:12}", c_val);
    println!("d   = {:12}\n", d_val);

    // Parmesan encrypt values
    let _p_ca = pu.encrypt_vec(&a)?;
    let _p_cb = pu.encrypt_vec(&b)?;
    let _p_cc = pu.encrypt_vec(&c)?;
    let _p_cd = pu.encrypt_vec(&d)?;

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
    let mut _p_c_pbs_id_a = vec![_p_ca[0].clone()];
    #[cfg(feature = "pbs")]
    {
    simple_duration!(
        ["PBS {}x", PBS_N],
        [
        measure_duration!(
            ["Pbs {}x", PBS_N],
            [
                for _ in 0..PBS_N {
                    _p_c_pbs_id_a = ParmCiphertext::single(pbs::id__pi_5(&pc, &_p_ca[0])?);
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
    let (_c_add_a_b, _c_sub_c_d, _c_add_ab_cnd);
    #[cfg(feature = "add")]
    {
    // Parmesan first level addition/subtraction:   a + b   ,   c - d
    simple_duration!(
        ["Parmesan::Add (1st lvl, {}-bit)", BITLEN],
        [
            p_add_a_b = ParmArithmetics::add(&pc, &_p_ca, &_p_cb);
        ]
    );
    simple_duration!(
        ["Parmesan::Sub (1st lvl, {}-bit)", BITLEN],
        [
            p_sub_c_d = ParmArithmetics::sub(&pc, &_p_cc, &_p_cd);
        ]
    );

    // Parmesan second level addition:   (a+b) + (c-d)
    simple_duration!(
        ["Parmesan::Add (2nd lvl, no refresh, {}-bit)", BITLEN],
        [
            p_add_ab_cnd = ParmArithmetics::add_noisy(&pc, &p_add_a_b, &p_sub_c_d);
        ]
    );

    #[cfg(feature = "concrete")]
    {
    // Concrete first level addition/subtraction:   a + b   ,   c - d
    simple_duration!(
        ["Concrete::Add (1st lvl, {}-bit)", BITLEN],
        [
            _c_add_a_b = _c_ca.clone() + _c_cb.clone();
        ]
    );
    simple_duration!(
        ["Concrete::Sub (1st lvl, {}-bit)", BITLEN],
        [
            _c_sub_c_d = _c_cc.clone() - _c_cd.clone();
        ]
    );

    // Concrete second level addition:   (a+b) + (c-d)
    simple_duration!(
        ["Concrete::Add (2nd lvl, {}-bit)", BITLEN],
        [
            _c_add_ab_cnd = _c_add_a_b.clone() + _c_sub_c_d.clone();
        ]
    );
    }
    }


    // =========================================================================
    //  Scalar Multiplication

    #[cfg(feature = "scm")]
    let mut p_scm_a: Vec<ParmCiphertext> = Vec::new();
    #[cfg(all(feature = "add", feature = "concrete"))]
    let mut _c_scm_a = Vec::new();
    #[cfg(feature = "scm")]
    {
    // Parmesan scalar multiplication: k * a
    for ki in _k {
        simple_duration!(
            ["Parmesan::Sc. Mul (by {}, {}-bit)", ki, BITLEN],
            [
                p_scm_a.push(ParmArithmetics::scalar_mul(&pc, ki, &_p_ca));
            ]
        );
    }

    #[cfg(feature = "concrete")]
    {
    // Concrete scalar multiplication: k * a
    for ki in _k {
        simple_duration!(
            ["Concrete::Sc. Mul (by {}, {}-bit)", ki, BITLEN],
            [
                let c_scmi = ki as u64 * _c_ca.clone();
                _c_scm_a.push(c_scmi);
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
    // Parmesan 1st level signum
    simple_duration!(
        ["Parmesan::Sgn a ({}-bit)", BITLEN],
        [
            p_sgn_a = ParmArithmetics::sgn(&pc, &_p_ca);
        ]
    );

    // Parmesan 2nd level signum
    simple_duration!(
        ["Parmesan::Sgn (a+b, {}-bit)", BITLEN],
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
    const ROUND_IDX: usize = 5;   // this is convenient for sgn
    #[cfg(feature = "round")]
    {
    // Parmesan rounding
    simple_duration!(
        ["Parmesan::Round (at {}, {}-bit)", ROUND_IDX, BITLEN],
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
    // Parmesan first level maximum
    simple_duration!(
        ["Parmesan::Max (1st lvl, {}-bit)", BITLEN],
        [
            p_max_a_b = ParmArithmetics::max(&pc, &_p_ca, &_p_cb);
        ]
    );
    simple_duration!(
        ["Parmesan::Max (1st lvl, {}-bit)", BITLEN],
        [
            p_max_c_d = ParmArithmetics::max(&pc, &_p_cc, &_p_cd);
        ]
    );

    // Parmesan second level maximum
    simple_duration!(
        ["Parmesan::Max (2nd lvl, {}-bit)", BITLEN],
        [
            p_max_mab_mcd = ParmArithmetics::max(&pc, &p_max_a_b, &p_max_c_d);
        ]
    );

    //TODO Concrete maximum?
    }


    // =========================================================================
    //  Multiplication

    #[cfg(any(feature = "mul", all(feature = "mul_light", any(feature = "4bit", feature = "8bit"))))]
    let p_mul_a_b;

    #[cfg(all(feature = "concrete", any(feature = "mul", all(feature = "mul_light", any(feature = "4bit", feature = "8bit")))))]
    let _c_mul_a_b;

    #[cfg(any(feature = "mul", all(feature = "mul_light", any(feature = "4bit", feature = "8bit"))))]
    {
        // Parmesan k-bit multiplication -> 2k+-bit
        simple_duration!(
            ["Parmesan::Mul ({}-bit)", BITLEN],
            [
                p_mul_a_b = ParmArithmetics::mul(&pc, &_p_ca, &_p_cb);
            ]
        );

        #[cfg(feature = "concrete")]
        {
            // Concrete k-bit multiplication -> 2k-bit
            simple_duration!(
                ["Concrete::Mul ({}-bit)", BITLEN],
                [
                    _c_mul_a_b = _c_ca.clone() * _c_cb.clone();
                ]
            );
        }
    }


    // =========================================================================
    //  Squaring

    #[cfg(any(feature = "squ", all(feature = "squ_light", any(feature = "4bit", feature = "8bit"))))]
    let p_squ_a;

    #[cfg(all(feature = "concrete", any(feature = "squ", all(feature = "squ_light", any(feature = "4bit", feature = "8bit")))))]
    let _c_squ_a;

    #[cfg(any(feature = "squ", all(feature = "squ_light", any(feature = "4bit", feature = "8bit"))))]
    {
        // Parmesan k-bit squaring -> 2k+-bit
        simple_duration!(
            ["Parmesan::Squ ({}-bit)", BITLEN],
            [
                p_squ_a = ParmArithmetics::squ(&pc, &_p_ca);
            ]
        );

        #[cfg(feature = "concrete")]
        {
            // Concrete k-bit squaring -> 2k+-bit
            simple_duration!(
                ["Concrete::Squ ({}-bit)", BITLEN],
                [
                    _c_squ_a = _c_ca.clone() * _c_ca.clone();
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
                            if c_a_v == a_val as u64 & ((1 << BITLEN) - 1) {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            a_val as u64 & ((1 << BITLEN) - 1)
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
    //~ #[cfg(feature = "concrete")]
    //~ {
    //~ }
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
    summary_text = format!("{}\nsgn(a+b)      = {:12} :: {} (exp. {})", summary_text,
                            sgn_ab,
                            if sgn_ab == (a_val + b_val).signum() {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            (a_val + b_val).signum()
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

    #[cfg(any(feature = "mul", all(feature = "mul_light", any(feature = "4bit", feature = "8bit"))))]
    {
    let mul_a_b     = pu.decrypt(&p_mul_a_b    )?;
    summary_text = format!("{}\n\nMultiplication:", summary_text);
    summary_text = format!("{}\na × b         = {:22} :: {} (exp. {})", summary_text,
                            mul_a_b,
                            if mul_a_b == a_val * b_val {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            a_val * b_val
    );
    #[cfg(all(feature = "concrete", any(feature = "mul", all(feature = "mul_light", any(feature = "4bit", feature = "8bit")))))]
    {
    let c_mul_a_b_v = _c_mul_a_b.decrypt(&client_key);
    summary_text = format!("{}\na × b (Conc)  = {:22} :: {} (exp. {})", summary_text,
                            c_mul_a_b_v,
                            if c_mul_a_b_v == (a_val as u64 * b_val as u64) % (1 << BITLEN) {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            (a_val * b_val) % (1 << BITLEN)
    );
    }
    }

    #[cfg(any(feature = "squ", all(feature = "squ_light", any(feature = "4bit", feature = "8bit"))))]
    {
    let squ_a       = pu.decrypt(&p_squ_a)?;
    summary_text = format!("{}\n\nSquaring:", summary_text);
    summary_text = format!("{}\na ^ 2         = {:22} :: {} (exp. {})", summary_text,
                            squ_a,
                            if squ_a == a_val * a_val {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            a_val * a_val
    );
    #[cfg(all(feature = "concrete", any(feature = "squ", all(feature = "squ_light", any(feature = "4bit", feature = "8bit")))))]
    {
    let c_squ_a_v = _c_squ_a.decrypt(&client_key);
    summary_text = format!("{}\na ^ 2 (Conc)  = {:22} :: {} (exp. {})", summary_text,
                            c_squ_a_v,
                            if c_squ_a_v == (a_val as u64 * a_val as u64) % (1 << BITLEN) {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            (a_val * a_val) % (1 << BITLEN)
    );
    }
    }

    #[cfg(feature = "scm")] // -------------------------------------------------
    {
    let mut scm_a: Vec<i64> = Vec::new();
    for ci in p_scm_a {
        scm_a.push(pu.decrypt(&ci)?);
    }
    summary_text = format!("{}\n\nScalar Multiplication:", summary_text);
    for (ki, scmi) in _k.iter().zip(scm_a.iter()) {
        summary_text = format!("{}\n{:7} × a   = {:12} :: {} (exp. {})", summary_text,
                                ki, scmi,
                                if *scmi == (*ki as i64) * a_val {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                                (*ki as i64) * a_val
        );
    }
    //~ #[cfg(feature = "concrete")]   //TODO
    //~ {
    //~ }
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
