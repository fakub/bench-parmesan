use std::error::Error;
use std::fs::{self,File,OpenOptions};
use std::path::Path;
use std::io::prelude::*;
use colored::Colorize;

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

extern crate chrono;
//~ use chrono::Utc;

fn main() {
    // run benchmark
    println!();
    simple_duration!(
        //~ ["🧀    {} {}    🧀   ", String::from("Parmesan").bold().yellow(), String::from("Benchmark").bold()],
        ["Parmesan Benchmark"],
        [
            let _x = bench();
        ]
    );

    println!();
}

fn bench() -> Result<(), Box<dyn Error>> {



    // =========================================================================
    //  Prepare userovo & cloudovo

    // not used at the moment
    #[cfg(not(feature = "sequential"))]
    println!("\n\n{}: {} threads\n", String::from("Parallel").bold().yellow(), rayon::current_num_threads());
    #[cfg(feature = "sequential")]
    println!("\n\n{}\n", String::from("Sequential").bold().yellow());

    // parameters
    let par = &params::PARM80__PI_5__D_22;   //  80    112    128

    simple_duration!(
        ["Setup keys"],
        //~ ["Setup/load keys"],
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



    // =========================================================================
    //  Generate & Encrypt inputs

    // 4 random 31-word sequences of {-1,0,1}
    let a: Vec<i32> = vec![1,0,1,-1,-1,0,-1,1,1,-1,1,1,1,-1,-1,0,0,1,1,0,0,0,0,-1,0,0,0,0,0,-1,0,0,];
    let b: Vec<i32> = vec![-1,0,0,-1,1,1,-1,1,-1,0,0,1,0,1,1,0,0,0,-1,0,0,1,0,0,-1,0,-1,-1,-1,1,1,0,];
    let c: Vec<i32> = vec![-1,1,-1,1,-1,1,0,0,-1,0,-1,1,0,0,1,1,1,1,1,0,-1,0,0,-1,1,0,1,1,-1,-1,0,];
    let d: Vec<i32> = vec![1,0,1,0,0,1,0,-1,0,1,-1,0,0,0,-1,0,1,-1,1,1,0,0,-1,-1,0,0,1,1,1,1,0,];

    // pairs of random 4-, 8- and 16-word sequences of {-1,0,1}
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
    let _k: [i32; 5] = [    // Hamming weight after optimization:
        -161,               // 3:   0   1   0   1   0   0   0   0   1
        0b11101111,         // 3:   1   0   0   0  -1   0   0   0  -1
        0b11100111,         // 4:   1   0   0  -1   0   1   0   0  -1
        0b10101010,         // 4:   0   1   0   1   0   1   0   1   0
        0b11011011,         // 4:   1   0   0  -1   0   0  -1   0  -1
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

    // encrypt values
    let _ca = pu.encrypt_vec(&a)?;
    let _cb = pu.encrypt_vec(&b)?;
    let _cc = pu.encrypt_vec(&c)?;
    let _cd = pu.encrypt_vec(&d)?;

    let _ca4  = pu.encrypt_vec(&a4 )?;
    let _cb4  = pu.encrypt_vec(&b4 )?;
    let _ca8  = pu.encrypt_vec(&a8 )?;
    let _cb8  = pu.encrypt_vec(&b8 )?;
    let _cc8  = pu.encrypt_vec(&c8 )?;
    let _cd8  = pu.encrypt_vec(&d8 )?;
    let _ca16 = pu.encrypt_vec(&a16)?;
    let _cb16 = pu.encrypt_vec(&b16)?;
    let _ca32 = pu.encrypt_vec(&a32)?;
    let _cb32 = pu.encrypt_vec(&b32)?;


    // =========================================================================
    //  Programmable Bootstrapping

    #[cfg(feature = "pbs")]
    let mut _c_pbs_id_a = ParmCiphertext::single(_ca[0].clone());
    #[cfg(feature = "pbs")]
    {
    // first level addition/subtraction:   a + b   ,   c - d
    simple_duration!(
        ["PBS {}x", PBS_N],
        [
        measure_duration!(
            ["Pbs {}x", PBS_N],
            //~ ["Programmable bootstrapping {}x", PBS_N],
            [
                for _ in 0..PBS_N {
                    _c_pbs_id_a = ParmCiphertext::single(pbs::id__pi_5(&pc.pub_keys, &_ca[0])?);
                }
            ]
        );
        ]
    );
    }


    // =========================================================================
    //  Addition

    #[cfg(feature = "add")]
    let c_add_a_b: ParmCiphertext;
    #[cfg(feature = "add")]
    let c_sub_c_d: ParmCiphertext;
    #[cfg(feature = "add")]
    let c_add_ab_cnd: ParmCiphertext;
    #[cfg(feature = "add")]
    {
    // first level addition/subtraction:   a + b   ,   c - d
    simple_duration!(
        ["Add (1st lvl)"],
        [
            c_add_a_b = ParmArithmetics::add(&pc, &_ca, &_cb);
        ]
    );
    simple_duration!(
        ["Sub (1st lvl)"],
        [
            c_sub_c_d = ParmArithmetics::sub(&pc, &_cc, &_cd);
        ]
    );

    // second level addition:   (a+b) + (c-d)
    simple_duration!(
        ["Add (2nd lvl, no refresh)"],
        [
            c_add_ab_cnd = ParmArithmetics::add_noisy(&pc, &c_add_a_b, &c_sub_c_d);
        ]
    );
    }


    // =========================================================================
    //  Signum

    #[cfg(feature = "sgn")]
    let c_sgn_a: ParmCiphertext;
    #[cfg(feature = "sgn")]
    let c_sgn_abcnd: ParmCiphertext;
    #[cfg(feature = "sgn")]
    {
    // first level signum
    simple_duration!(
        ["Sgn (no BS, 1st lvl)"],
        //~ ["1st level signum: sgn(a)   (no BS)"],
        [
            c_sgn_a = ParmArithmetics::sgn(&pc, &_ca);
        ]
    );

    // second level signum
    simple_duration!(
        ["Sgn (w BS, 2nd lvl)"],
        //~ ["2nd level signum: sgn((a+b) + (c-d))   (with BS)"],
        [
            c_sgn_abcnd = ParmArithmetics::sgn(&pc, &c_add_ab_cnd);
        ]
    );
    }


    // =========================================================================
    //  Rounding

    #[cfg(feature = "round")]
    let c_round_a: ParmCiphertext;
    #[cfg(feature = "round")]
    const ROUND_IDX: usize = 6;
    #[cfg(feature = "round")]
    {
    // first level rounding
    simple_duration!(
        ["Round"],
        [
            c_round_a = ParmArithmetics::round_at(&pc, &_ca, ROUND_IDX);
        ]
    );
    }


    // =========================================================================
    //  Maximum

    #[cfg(feature = "max")]
    let c_max_a_b: ParmCiphertext;
    #[cfg(feature = "max")]
    let c_max_c_d: ParmCiphertext;
    #[cfg(feature = "max")]
    let c_max_mab_mcd: ParmCiphertext;
    #[cfg(feature = "max")]
    {
    // first level maximum
    simple_duration!(
        ["Max (no BS, 1st lvl)"],
        //~ ["1st level maximum: max(a, b)   (no BS)"],
        [
            c_max_a_b = ParmArithmetics::max(&pc, &_ca, &_cb);
        ]
    );
    simple_duration!(
        ["Max (no BS, 1st lvl)"],
        //~ ["1st level maximum: max(c, d)   (no BS)"],
        [
            c_max_c_d = ParmArithmetics::max(&pc, &_cc, &_cd);
        ]
    );

    // second level maximum
    simple_duration!(
        ["Max (w BS, 2nd lvl)"],
        //~ ["2nd level maximum: max(m_ab, m_cd)   (with BS)"],
        [
            c_max_mab_mcd = ParmArithmetics::max(&pc, &c_max_a_b, &c_max_c_d);
        ]
    );
    }


    // =========================================================================
    //  Multiplication

    #[cfg(feature = "mul_light")]
    let c_mul4_a_b: ParmCiphertext;
    #[cfg(feature = "mul_light")]
    let c_mul8_a_b: ParmCiphertext;
    #[cfg(feature = "mul")]
    let c_mul16_a_b: ParmCiphertext;
    #[cfg(feature = "mul")]
    let c_mul32_a_b: ParmCiphertext;
    #[cfg(feature = "mul_light")]
    {
    // 4-word multiplication -> 8-word
    simple_duration!(
        ["Mul (4-word)"],
        //~ ["4-word multiplication: a4 × b4"],
        [
            c_mul4_a_b = ParmArithmetics::mul(&pc, &_ca4, &_cb4);
        ]
    );

    // 8-word multiplication -> 16-word
    simple_duration!(
        ["Mul (8-word)"],
        //~ ["8-word multiplication: a8 × b8"],
        [
            c_mul8_a_b = ParmArithmetics::mul(&pc, &_ca8, &_cb8);
        ]
    );
    }
    #[cfg(feature = "mul")]
    {
    // 16-word multiplication -> 33-word
    simple_duration!(
        ["Mul (16-word)"],
        //~ ["16-word multiplication: a16 × b16"],
        [
            c_mul16_a_b = ParmArithmetics::mul(&pc, &_ca16, &_cb16);
        ]
    );

    // 32-word multiplication -> 66-word (if there happen to be zeros at the leading positions after decryption, it PASSes)
    simple_duration!(
        ["Mul (32-word)"],
        //~ ["32-word multiplication: a32 × b32"],
        [
            c_mul32_a_b = ParmArithmetics::mul(&pc, &_ca32, &_cb32);
        ]
    );
    }


    // =========================================================================
    //  Squaring

    #[cfg(feature = "squ_light")]
    let c_squ_a4: ParmCiphertext;
    #[cfg(feature = "squ_light")]
    let c_squ_a8: ParmCiphertext;
    #[cfg(feature = "squ")]
    let c_squ_a16: ParmCiphertext;
    #[cfg(feature = "squ")]
    let c_squ_a32: ParmCiphertext;
    #[cfg(feature = "squ_light")]
    {
    // 4-word squaring -> 9-word (?)
    simple_duration!(
        ["Squ (4-word)"],
        //~ ["4-word squaring: a4 ^ 2"],
        [
            c_squ_a4 = ParmArithmetics::squ(&pc, &_ca4);
        ]
    );

    // 8-word squaring -> 18-word (?)
    simple_duration!(
        ["Squ (8-word)"],
        //~ ["8-word squaring: a8 ^ 2"],
        [
            c_squ_a8 = ParmArithmetics::squ(&pc, &_ca8);
        ]
    );
    }

    #[cfg(feature = "squ")]
    {
    // 16-word squaring -> 35-word (?)
    simple_duration!(
        ["Squ (16-word)"],
        //~ ["16-word squaring: a16 ^ 2"],
        [
            c_squ_a16 = ParmArithmetics::squ(&pc, &_ca16);
        ]
    );

    // 32-word squaring -> 68-word (?)
    simple_duration!(
        ["Squ (32-word)"],
        //~ ["32-word squaring: a32 ^ 2"],
        [
            c_squ_a32 = ParmArithmetics::squ(&pc, &_ca32);
        ]
    );
    }


    // =========================================================================
    //  Scalar Multiplication

    #[cfg(feature = "scm")]
    let mut c_scm16_a: Vec<ParmCiphertext> = Vec::new();
    #[cfg(feature = "scm")]
    {
    // scalar multiplication of 16-word
    for ki in _k {
        simple_duration!(
            ["Sc. Mul (w BS, {})", ki],
            //~ ["scalar multiplication: {}/0b{:b}/ × a16   (with BS)", ki, ki.abs()],
            [
                c_scm16_a.push(ParmArithmetics::scalar_mul(&pc, ki, &_ca16));
            ]
        );
    }
    }


    // =========================================================================
    //  NN Evaluation

    #[cfg(feature = "nn")]
    let c_nn_out: Vec<ParmCiphertext>;
    #[cfg(feature = "nn")]
    let m_nn_out: Vec<i64>;
    #[cfg(feature = "nn")]
    {
    // evaluation of a simple NN
    let c_nn_in           = vec![   _ca8,    _cb8,    _cc8,    _cd8];   // .clone()
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
        //~ ["neural network evaluation: NN(a8, b8, c8, d8)   (??? BS)"],
        [
            c_nn_out = demo_nn().eval(&pc, &c_nn_in);
        ]
    );
    m_nn_out = demo_nn().eval(&pc, &m_nn_in);
    }


    // =========================================================================
    //  Decrypt & Check Correctness

    #[allow(unused_mut)]
    let mut summary_text = format!("\n{}:", String::from("Results").bold().yellow());

    // decrypt & verify all results
    #[cfg(feature = "pbs")]
    {
    let pbs_id_a0   = pu.decrypt(&_c_pbs_id_a)?;
    summary_text = format!("{}\n\nProgrammable Bootstrapping:", summary_text);
    summary_text = format!("{}\nid(a[0])      = {:12} :: {} (exp. {})", summary_text,
                            pbs_id_a0,
                            if a[0] as i64 == pbs_id_a0 {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            a[0]
    );
    }

    #[cfg(feature = "add")]
    {
    let add_a_b     = pu.decrypt(&c_add_a_b     )?;
    let sub_c_d     = pu.decrypt(&c_sub_c_d     )?;
    let add_ab_cnd  = pu.decrypt(&c_add_ab_cnd  )?;
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
    }

    #[cfg(feature = "sgn")]
    {
    let sgn_a       = pu.decrypt(&c_sgn_a       )?;
    let sgn_abcnd   = pu.decrypt(&c_sgn_abcnd   )?;
    summary_text = format!("{}\n\nSignum:", summary_text);
    summary_text = format!("{}\nsgn(a)        = {:12} :: {} (exp. {})", summary_text,
                            sgn_a,
                            if sgn_a == a_val.signum() {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            a_val.signum()
    );
    summary_text = format!("{}\nsgn(a+b+c-d)  = {:12} :: {} (exp. {})", summary_text,
                            sgn_abcnd,
                            if sgn_abcnd == (a_val + b_val + c_val - d_val).signum() {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            (a_val + b_val + c_val - d_val).signum()
    );
    }

    #[cfg(feature = "round")]
    {
    let round_a       = pu.decrypt(&c_round_a       )?;
    // complex rounding of f64 in Rust:        sgn * (        abs          divide                     round                       )
    let round_a_val   = if a_val < 0 {-1} else {1} * (((a_val.abs() as f64 / (1 << ROUND_IDX) as f64).round() as u64) << ROUND_IDX) as i64;
    summary_text = format!("{}\n\nRounding:", summary_text);
    summary_text = format!("{}\nround(a)      = {:12} :: {} (exp. {})", summary_text,
                            round_a,
                            if round_a == round_a_val {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            round_a_val
    );
    }

    #[cfg(feature = "max")]
    {
    let max_a_b     = pu.decrypt(&c_max_a_b     )?;
    let max_c_d     = pu.decrypt(&c_max_c_d     )?;
    let max_mab_mcd = pu.decrypt(&c_max_mab_mcd )?;
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

    #[cfg(feature = "mul_light")]
    {
    let mul4_a_b    = pu.decrypt(&c_mul4_a_b    )?;
    let mul8_a_b    = pu.decrypt(&c_mul8_a_b    )?;
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
    }
    #[cfg(feature = "mul")]
    {
    let mul16_a_b   = pu.decrypt(&c_mul16_a_b   )?;
    let mul32_a_b   = pu.decrypt(&c_mul32_a_b   )?;
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

    #[cfg(feature = "squ_light")]
    {
    let squ_a4      = pu.decrypt(&c_squ_a4      )?;
    let squ_a8      = pu.decrypt(&c_squ_a8      )?;
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
    }
    #[cfg(feature = "squ")]
    {
    let squ_a16     = pu.decrypt(&c_squ_a16     )?;
    let squ_a32     = pu.decrypt(&c_squ_a32     )?;
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

    #[cfg(feature = "scm")]
    {
    let mut scm16_a: Vec<i64> = Vec::new();
    for ci in c_scm16_a {
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
    }

    #[cfg(feature = "nn")]
    {
    let mut nn_out_homo: Vec<i64> = Vec::new();
    for ci in c_nn_out {
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
