use std::error::Error;
use colored::Colorize;

use parmesan::params;
use parmesan::ciphertexts::ParmCiphertext;

use parmesan::userovo::*;
use parmesan::ParmesanUserovo;

//~ use parmesan::cloudovo::*;
use parmesan::ParmesanCloudovo;

fn main() {
    println!();
    parmesan::simple_duration!(
        ["🧀    {} {}    🧀   ", String::from("Parmesan").bold().yellow(), String::from("Benchmark").bold()],
        [
            let _x = bench();
        ]
    );
    println!();
}

fn bench() -> Result<(), Box<dyn Error>> {



    // =========================================================================
    //  Prepare userovo & cloudovo

    #[cfg(not(feature = "sequential"))]
    println!("\n\n{}: {} threads\n", String::from("Parallel").bold().yellow(), rayon::current_num_threads());
    #[cfg(feature = "sequential")]
    println!("\n\n{}\n", String::from("Sequential").bold().yellow());

    // parameters
    let par = &params::PARM90__PI_5__D_20__LEN_32;   //     PARM90__PI_5__D_20__LEN_32      PARMXX__TRIVIAL

    parmesan::simple_duration!(
        ["Setup keys"],
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

    // 4 random 32-word sequences of {-1,0,1}
    let a: Vec<i32> = vec![1,0,1,-1,-1,0,-1,1,1,-1,1,1,1,-1,-1,0,0,1,1,0,0,0,0,-1,0,0,0,0,0,-1,0,0,];
    let b: Vec<i32> = vec![-1,0,0,-1,1,1,-1,1,-1,0,0,1,0,1,1,0,0,0,-1,0,0,1,0,0,-1,0,-1,-1,-1,1,1,0,];
    let c: Vec<i32> = vec![-1,1,-1,1,-1,1,0,0,-1,0,-1,1,0,0,1,1,1,1,1,0,-1,0,0,-1,1,0,1,1,-1,-1,0,];
    let d: Vec<i32> = vec![1,0,1,0,0,1,0,-1,0,1,-1,0,0,0,-1,0,1,-1,1,1,0,0,-1,-1,0,0,1,1,1,1,0,];

    // pairs of random 4-, 8- and 16-word sequences of {-1,0,1}
    let a4:  Vec<i32> = vec![1,-1,-1,0,];
    let b4:  Vec<i32> = vec![0,0,1,-1,];
    let a8:  Vec<i32> = vec![1,1,-1,-1,0,0,-1,-1,];
    let b8:  Vec<i32> = vec![-1,0,1,0,0,-1,0,-1,];
    let a16: Vec<i32> = vec![0,0,1,0,-1,1,-1,-1,0,1,1,0,0,1,-1,1,];
    let b16: Vec<i32> = vec![1,1,0,0,0,1,0,1,1,1,0,1,0,1,1,-1,];
    let a32: Vec<i32> = vec![-1,-1,1,0,-1,0,-1,0,1,-1,-1,0,1,-1,0,-1,0,-1,1,1,1,-1,1,-1,0,0,-1,0,0,1,1,0,];
    let b32: Vec<i32> = vec![1,-1,-1,-1,1,-1,1,-1,0,1,-1,0,1,0,1,0,-1,1,1,-1,1,-1,-1,0,0,-1,-1,0,-1,-1,-1,0,];

    // random scalars
    let k: [i32; 3] = [-161, 128, 1023];

    // convert to actual numbers
    let a_val = encryption::convert(&a)?;
    let b_val = encryption::convert(&b)?;
    let c_val = encryption::convert(&c)?;
    let d_val = encryption::convert(&d)?;

    let a4_val  = encryption::convert(&a4 )?;
    let b4_val  = encryption::convert(&b4 )?;
    let a8_val  = encryption::convert(&a8 )?;
    let b8_val  = encryption::convert(&b8 )?;
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
    println!("a16 = {:12}", a16_val);
    println!("b16 = {:12}", b16_val);
    println!("a32 = {:12}", a32_val);
    println!("b32 = {:12}\n", b32_val);

    // encrypt values
    let ca = pu.encrypt_vec(&a)?;
    let cb = pu.encrypt_vec(&b)?;
    let cc = pu.encrypt_vec(&c)?;
    let cd = pu.encrypt_vec(&d)?;

    let ca4  = pu.encrypt_vec(&a4 )?;
    let cb4  = pu.encrypt_vec(&b4 )?;
    let ca8  = pu.encrypt_vec(&a8 )?;
    let cb8  = pu.encrypt_vec(&b8 )?;
    let ca16 = pu.encrypt_vec(&a16)?;
    let cb16 = pu.encrypt_vec(&b16)?;
    let ca32 = pu.encrypt_vec(&a32)?;
    let cb32 = pu.encrypt_vec(&b32)?;



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
    parmesan::simple_duration!(
        ["1st level addition: a + b   (no BS)"],
        [
            c_add_a_b = pc.add(&ca, &cb)?;
        ]
    );
    parmesan::simple_duration!(
        ["1st level subtraction: c - d   (no BS)"],
        [
            c_sub_c_d = pc.sub(&cc, &cd)?;
        ]
    );

    // second level addition:   (a+b) + (c-d)
    //TODO bootstrap 1 !!
    parmesan::simple_duration!(
        ["2nd level addition: (a+b) + (c-d)   (with BS)"],
        [
            c_add_ab_cnd = pc.add(&c_add_a_b, &c_sub_c_d)?;
            //TODO bootstrap 2 !!
            // idea: have add do bootstrap implicitly, add_dirty without bootstrap
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
    parmesan::simple_duration!(
        ["1st level signum: sgn(a)   (no BS)"],
        [
            c_sgn_a = pc.sgn(&ca)?;
        ]
    );

    // second level signum
    parmesan::simple_duration!(
        ["2nd level signum: sgn((a+b) + (c-d))   (with BS)"],
        [
            c_sgn_abcnd = pc.sgn(&c_add_ab_cnd)?;
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
    parmesan::simple_duration!(
        ["1st level maximum: max(a, b)   (no BS)"],
        [
            c_max_a_b = pc.max(&ca, &cb)?;
        ]
    );
    parmesan::simple_duration!(
        ["1st level maximum: max(c, d)   (no BS)"],
        [
            c_max_c_d = pc.max(&cc, &cd)?;
        ]
    );

    // second level maximum
    parmesan::simple_duration!(
        ["2nd level maximum: max(m_ab, m_cd)   (with BS)"],
        [
            c_max_mab_mcd = pc.max(&c_max_a_b, &c_max_c_d)?;
        ]
    );
    }


    // =========================================================================
    //  Multiplication

    #[cfg(feature = "mul")]
    let c_mul4_a_b: ParmCiphertext;
    #[cfg(feature = "mul")]
    let c_mul8_a_b: ParmCiphertext;
    #[cfg(feature = "mul")]
    let c_mul16_a_b: ParmCiphertext;
    #[cfg(feature = "mul")]
    let c_mul32_a_b: ParmCiphertext;
    #[cfg(feature = "mul")]
    {
    // 4-word multiplication -> 8-word (congruent mod 2^8, can have negative sign, not good for comparison .. ???)
    parmesan::simple_duration!(
        ["4-word multiplication: a4 × b4"],
        [
            c_mul4_a_b = pc.mul(&ca4, &cb4)?;
        ]
    );

    // 8-word multiplication -> 16-word (...)
    parmesan::simple_duration!(
        ["8-word multiplication: a8 × b8"],
        [
            c_mul8_a_b = pc.mul(&ca8, &cb8)?;
        ]
    );

    // 16-word multiplication -> 32-word (...)
    parmesan::simple_duration!(
        ["16-word multiplication: a16 × b16"],
        [
            c_mul16_a_b = pc.mul(&ca16, &cb16)?;
        ]
    );

    // 32-word multiplication -> 64-word (...)
    parmesan::simple_duration!(
        ["32-word multiplication: a32 × b32"],
        [
            c_mul32_a_b = pc.mul(&ca32, &cb32)?;
        ]
    );
    }


    // =========================================================================
    //  Scalar Multiplication

    #[cfg(feature = "scm")]
    let mut c_scm16_a = Vec::new();
    #[cfg(feature = "scm")]
    {
    // scalar multiplication of 16-word
    for ki in k {
        parmesan::simple_duration!(
            ["scalar multiplication: {}/0b{:b}/ × a16   (with BS)", ki, ki.abs()],
            [
                c_scm16_a.push(pc.scalar_mul(ki, &ca16)?);
            ]
        );
    }
    }


    // =========================================================================
    //  TODO: NN Evaluation

    // some very simple NN


    // =========================================================================
    //  Decrypt & Check Correctness

    let mut summary_text = format!("\n{}:", String::from("Results").bold().yellow());

    // decrypt & verify all results
    #[cfg(feature = "add")]
    {
    let add_a_b     = pu.decrypt(&c_add_a_b     )?;
    let sub_c_d     = pu.decrypt(&c_sub_c_d     )?;
    let add_ab_cnd  = pu.decrypt(&c_add_ab_cnd  )?;
    summary_text = format!("{}\n\nAddition:", summary_text);
    summary_text = format!("{}\na + b         = {:12} :: {} (exp. {} % {})", summary_text,
                            add_a_b,
                            if (a_val + b_val - add_a_b) % (1 << 32) == 0 {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            (a_val + b_val) % (1 << 32), 1u64 << 32
    );
    summary_text = format!("{}\nc - d         = {:12} :: {} (exp. {} % {})", summary_text,
                            sub_c_d,
                            if (c_val - d_val - sub_c_d) % (1 << 32) == 0 {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            (c_val - d_val) % (1 << 32), 1u64 << 32
    );
    summary_text = format!("{}\n(a+b) + (c-d) = {:12} :: {} (exp. {} % {})", summary_text,
                            add_ab_cnd,
                            if (add_a_b + sub_c_d - add_ab_cnd) % (1 << 32) == 0 {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            (add_a_b + sub_c_d) % (1 << 32), 1u64 << 32
    );
    }

    #[cfg(feature = "sgn")]
    {
    let sgn_a       = pu.decrypt(&c_sgn_a       )?;
    let sgn_abcnd   = pu.decrypt(&c_sgn_abcnd   )?;
    summary_text = format!("{}\n\nSignum:", summary_text);
    summary_text = format!("{}\nsgn(a)        = {:12} :: {} (exp. {} % {})", summary_text,
                            sgn_a,
                            if sgn_a == a_val.signum() {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            a_val.signum() % (1 << 32), 1u64 << 32
    );
    summary_text = format!("{}\nsgn(a+b+c-d)  = {:12} :: {} (exp. {} % {})", summary_text,
                            sgn_abcnd,
                            if sgn_abcnd == (a_val + b_val + c_val - d_val).signum() {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            (a_val + b_val + c_val - d_val).signum() % (1 << 32), 1u64 << 32
    );
    }

    #[cfg(feature = "max")]
    {
    let max_a_b     = pu.decrypt(&c_max_a_b     )?;
    let max_c_d     = pu.decrypt(&c_max_c_d     )?;
    let max_mab_mcd = pu.decrypt(&c_max_mab_mcd )?;
    summary_text = format!("{}\n\nMaximum:", summary_text);
    summary_text = format!("{}\nmax{{a, b}}     = {:12} :: {} (exp. {} % {})", summary_text,
                            max_a_b,
                            if max_a_b == std::cmp::max(a_val, b_val) {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            std::cmp::max(a_val, b_val) % (1 << 32), 1u64 << 32
    );
    summary_text = format!("{}\nmax{{c, d}}     = {:12} :: {} (exp. {} % {})", summary_text,
                            max_c_d,
                            if max_c_d == std::cmp::max(c_val, d_val) {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            std::cmp::max(c_val, d_val) % (1 << 32), 1u64 << 32
    );
    summary_text = format!("{}\nmax{{m_ab,m_cd}}= {:12} :: {} (exp. {} % {})", summary_text,
                            max_mab_mcd,
                            if max_mab_mcd == std::cmp::max(max_a_b, max_c_d) {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            std::cmp::max(max_a_b, max_c_d) % (1 << 32), 1u64 << 32
    );
    }

    #[cfg(feature = "mul")]
    {
    let mul4_a_b    = pu.decrypt(&c_mul4_a_b    )?;
    let mul8_a_b    = pu.decrypt(&c_mul8_a_b    )?;
    let mul16_a_b   = pu.decrypt(&c_mul16_a_b   )?;
    let mul32_a_b   = pu.decrypt(&c_mul32_a_b   )?;
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

    #[cfg(feature = "scm")]
    {
    let mut scm16_a = Vec::new();
    for ci in c_scm16_a {
        scm16_a.push(pu.decrypt(&ci)?);
    }
    summary_text = format!("{}\n\nScalar Multiplication:", summary_text);
    for (ki, scmi) in k.iter().zip(scm16_a.iter()) {
        summary_text = format!("{}\n{:7} × a16 = {:12} :: {} (exp. {})", summary_text,
                                ki, scmi,
                                if *scmi == (*ki as i64) * a16_val {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                                (*ki as i64) * a16_val
        );
    }
    }

    println!("{}\n", summary_text);

    Ok(())
}
