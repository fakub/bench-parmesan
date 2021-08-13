use std::error::Error;
use colored::Colorize;

use parmesan::params;
use parmesan::ciphertexts::ParmCiphertext;
use parmesan::userovo::*;
use parmesan::ParmesanUserovo;
use parmesan::cloudovo::*;
use parmesan::ParmesanCloudovo;

//DBG
use concrete::LWE;

fn main() {

    //DBG
    let _q = parmesan::arith_demo();
    println!(">>> parmesan::arith_demo DONE");
    let _p = bench_demo();
    println!(">>> bench_demo DONE");



    // =========================================================================
    //  Prepare userovo & cloudovo

    #[cfg(not(feature = "sequential"))]
    println!("\nParallel ({} threads)\n", rayon::current_num_threads());
    #[cfg(feature = "sequential")]
    println!("\nSequential\n");

    // ---------------------------------
    //  Parameters
    let par = &params::PARM90__PI_5__D_20__LEN_32;   //     PARM90__PI_5__D_20__LEN_32      PARMXX__TRIVIAL

    parmesan::simple_duration!(
        ["Setup keys"],
        [
            // ---------------------------------
            //  Userovo Scope
            let pu = ParmesanUserovo::new(par).expect("ParmesanUserovo::new failed.");
            let pub_k = pu.export_pub_keys();

            // ---------------------------------
            //  Cloudovo Scope
            let pc = ParmesanCloudovo::new(
                par,
                &pub_k,
            );
        ]
    );


    // =========================================================================
    //  Generate & Encrypt inputs

    // take 4 random 32-word sequences of {-1,0,1} (possibly other lengths, too)
    //~ let mp: Vec<i32> = vec![1,0,1,-1,-1,0,-1,1,1,-1,1,1,1,-1,-1,0,0,1,1,0,0,0,0,-1,0,0,0,0,0,-1,0,0,];
    //~ let mn: Vec<i32> = vec![-1,0,0,-1,1,1,-1,1,-1,0,0,1,0,1,1,0,0,0,-1,0,0,1,0,0,-1,0,-1,-1,-1,1,1,0,];
    let mut mp: Vec<i32> = vec![1,0,0,1,1,0,1,1,1,0,1,1,]; mp.reverse();   // [0,1,1,1,1,1,1,0,1,1,0,0,1,0,0,1,0,0,1,1,1,0,0,1,1,0,1,1,1,0,1,1,];
    let mut mn: Vec<i32> = vec![1,1,1,1,0,0,1,0,0,0,0,0,]; mn.reverse();   // [0,0,1,1,0,0,1,0,0,0,1,1,1,1,1,0,0,1,1,0,1,1,1,1,0,0,1,0,0,0,0,0,];
    let mp_val = encryption::convert(&mp).expect("encryption::convert failed.");
    let mn_val = encryption::convert(&mn).expect("encryption::convert failed.");
    println!("mp_in = {}",   mp_val);
    println!("mn_in = {}\n", mn_val);
    //~ let cp = pu.encrypt_vec(&mp).expect("encrypt_vec failed.");
    //~ let cn = pu.encrypt_vec(&mn).expect("encrypt_vec failed.");

    //~ //DBG
    let cp = pu.encrypt(2491, 12).expect("encrypt_vec failed.");
    let cn = pu.encrypt(3872, 12).expect("encrypt_vec failed.");
    //~ let mp_d = pu.decrypt(&cp).expect("decrypt failed.");
    //~ let mn_d = pu.decrypt(&cn).expect("decrypt failed.");
    //~ println!("mp_dc = {}", mp_d);
    //~ println!("mn_dc = {}\n", mn_d);
    // for most operations
    let m: [i64; 3] = [
         0b01111110110010010011100110111011,
         0b00110010001111100110111100100000,
        -0b01000100001010010111100000010101,
    ];
    let mut m_as: [i64; 3] = [0,0,0];
    let mut c: [parmesan::ParmCiphertext; 3] = [
        vec![LWE::zero(0).expect("... failed."); 12],
        vec![LWE::zero(0).expect("... failed."); 12],
        vec![LWE::zero(0).expect("... failed."); 12],
    ];
    for (ci, (mi, mi_as)) in c.iter_mut().zip(m.iter().zip(m_as.iter_mut())) {
        *ci = pu.encrypt(*mi, 12).expect("... failed.");
        *mi_as = (*mi).signum() * ((*mi).abs() % (1 << 12));
    }
    let c_add  = pc.add(&c[0], &c[1]).expect("... failed.");
    let c_sub  = pc.sub(&c[1], &c[0]).expect("... failed.");
    let m_add  = pu.decrypt(&c_add ).expect("... failed.");
    let m_sub  = pu.decrypt(&c_sub ).expect("... failed.");
    println!(" + .. {}\n - .. {}", m_add, m_sub);


    // =========================================================================
    //  Addition

    // first-level addition/subtraction:   a + b   ,   c + d
    parmesan::simple_duration!(
        ["1st level addition"],
        [
            let c_a1 = pc.add(&cp, &cn).expect("add failed.");
        ]
    );
    //DBG
    println!("c_a1 = {:?}", c_a1);
    parmesan::simple_duration!(
        ["1st level subtraction"],
        [
            let c_s1 = pc.sub(&cn, &cp).expect("sub failed.");
        ]
    );
    //DBG
    println!("c_s1 = {:?}", c_s1);

    // second level addition   (a+b) + (c+d)


    // =========================================================================
    //  Signum

    // signum of fresh

    // signum of result


    // =========================================================================
    //  Maximum

    // first-level maximum

    // second-level maximum


    // =========================================================================
    //  Multiplication

    // 4-word multiplication -> 8-word (congruent mod 2^8, can have negative sign, not good for comparison)

    // 8-word multiplication -> 16-word (...)

    // 16-word multiplication -> 32-word (...)

    // 32-word multiplication -> 64-word (...)


    // =========================================================================
    //  TODO: Scalar Multiplication

    // 4-bit scalar times 4-word ciphertext

    // 8-bit scalar times 8-word ciphertext

    // 16-bit scalar times 16-word ciphertext


    // =========================================================================
    //  TODO: NN Evaluation

    // some very simple NN


    // =========================================================================
    //  Decrypt & Check Correctness

    // decrypt all results
    let m_a1 = pu.decrypt(&c_a1).expect("decrypt failed.");
    let m_s1 = pu.decrypt(&c_s1).expect("decrypt failed.");

    // print summary
    let mut summary_text = format!("\n{}", String::from("Results:").bold().yellow());
    summary_text = format!("{}\nmp + mn     = {:12} :: {} (exp. {} % {})", summary_text,
                            m_a1,
                            if (mp_val + mn_val - m_a1) % (1 << 32) == 0 {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            (mp_val + mn_val) % (1 << 32), 1 << 32
    );
    summary_text = format!("{}\nmp - mn     = {:12} :: {} (exp. {} % {})", summary_text,
                            m_s1,
                            if (mp_val - mn_val - m_s1) % (1 << 32) == 0 {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            (mp_val - mn_val) % (1 << 32), 1 << 32
    );   //TODO something like DEMO_BITLEN

}

fn bench_demo() -> Result<(), Box<dyn Error>> {

    // move to Cloudovo initialization (makes no sense at user, but now I want to have it on the top)
    #[cfg(not(feature = "sequential"))]
    println!("Parallel ({} threads)", rayon::current_num_threads());
    #[cfg(feature = "sequential")]
    println!("Sequential");


    // =================================
    //  Initialization

    // ---------------------------------
    //  Global Scope
    let par = &params::PARM90__PI_5__D_20__LEN_32;   //     PARM90__PI_5__D_20__LEN_32      PARMXX__TRIVIAL

    // ---------------------------------
    //  Userovo Scope
    let pu = ParmesanUserovo::new(par)?;
    let pub_k = pu.export_pub_keys();

    const DEMO_BITLEN: usize = 12;
    const DEMO_N_MSGS: usize = 3;

    // ---------------------------------
    //  Cloudovo Scope
    let pc = ParmesanCloudovo::new(
        par,
        &pub_k,
    );


    // =================================
    //  U: Encryption

    // for most operations
    let m: [i64; DEMO_N_MSGS] = [
         0b01111110110010010011100110111011,
         0b00110010001111100110111100100000,
        -0b01000100001010010111100000010101,
    ];
    let mut m_as: [i64; DEMO_N_MSGS] = [0,0,0];
    // for multiplication
    let m_x1 : i64 =  0b1;
    let m_y1 : i64 = -0b1;
    let m_x4 : i64 =  0b1110;                   //    14
    let m_y4 : i64 =  0b1001;                   //     9    ->         126
    let m_x8 : i64 =  0b10010111;               //   151
    let m_y8 : i64 =  0b10111010;               //   186    ->       28086
    let m_x16: i64 =  0b110000101101011;        // 24939
    let m_y16: i64 =  0b100011010100001;        // 18081    ->   450922059
    let m_x17: i64 =  0b1111011001001001;       // 63049
    let m_y17: i64 =  0b1001000111110011;       // 37363    ->  2355699787 which is more than 2^31 - 1
    let m_x32: i64 =  0b01100110010010111011011001100110;   // 1716237926
    let m_y32: i64 =  0b01001011100111010100110001010100;   // 1268599892   ->  2177219247569903992 which fits 63 bits (i64)

    // encrypt all values
    let mut c: [ParmCiphertext; DEMO_N_MSGS] = [
        vec![LWE::zero(0)?; DEMO_BITLEN],
        vec![LWE::zero(0)?; DEMO_BITLEN],
        vec![LWE::zero(0)?; DEMO_BITLEN],
    ];
    for (ci, (mi, mi_as)) in c.iter_mut().zip(m.iter().zip(m_as.iter_mut())) {
        *ci = pu.encrypt(*mi, DEMO_BITLEN)?;
        *mi_as = (*mi).signum() * ((*mi).abs() % (1 << DEMO_BITLEN));
    }
    let cx1 = pu.encrypt(m_x1,   1)?;
    let cy1 = pu.encrypt(m_y1,   1)?;
    let cx4 = pu.encrypt(m_x4,   4)?;
    let cy4 = pu.encrypt(m_y4,   4)?;
    let cx8 = pu.encrypt(m_x8,   8)?;
    let cy8 = pu.encrypt(m_y8,   8)?;
    let cx16= pu.encrypt(m_x16, 16)?;
    let cy16= pu.encrypt(m_y16, 16)?;
    let cx17= pu.encrypt(m_x17, 17)?;
    let cy17= pu.encrypt(m_y17, 17)?;
    let cx32= pu.encrypt(m_x32, 32)?;
    let cy32= pu.encrypt(m_y32, 32)?;

    // print message
    let mut intro_text = format!("{} messages ({} bits taken)", String::from("User:").bold().yellow(), DEMO_BITLEN);
    for (i, (mi, mi_as)) in m.iter().zip(m_as.iter()).enumerate() {
        intro_text = format!("{}\nm_{}  = {}{:032b} ({})", intro_text, i, if *mi >= 0 {" "} else {"-"}, mi.abs(), mi_as);
    }
    intro_text = format!("{}\nx_1  = {}{:01b} ({})",  intro_text, if m_x1  >= 0 {" "} else {"-"}, m_x1.abs(),  m_x1 );
    intro_text = format!("{}\ny_1  = {}{:01b} ({})",  intro_text, if m_y1  >= 0 {" "} else {"-"}, m_y1.abs(),  m_y1 );
    intro_text = format!("{}\nx_4  = {}{:04b} ({})",  intro_text, if m_x4  >= 0 {" "} else {"-"}, m_x4.abs(),  m_x4 );
    intro_text = format!("{}\ny_4  = {}{:04b} ({})",  intro_text, if m_y4  >= 0 {" "} else {"-"}, m_y4.abs(),  m_y4 );
    intro_text = format!("{}\nx_8  = {}{:08b} ({})",  intro_text, if m_x8  >= 0 {" "} else {"-"}, m_x8.abs(),  m_x8 );
    intro_text = format!("{}\ny_8  = {}{:08b} ({})",  intro_text, if m_y8  >= 0 {" "} else {"-"}, m_y8.abs(),  m_y8 );
    intro_text = format!("{}\nx_16 = {}{:016b} ({})", intro_text, if m_x16 >= 0 {" "} else {"-"}, m_x16.abs(), m_x16);
    intro_text = format!("{}\ny_16 = {}{:016b} ({})", intro_text, if m_y16 >= 0 {" "} else {"-"}, m_y16.abs(), m_y16);
    intro_text = format!("{}\nx_17 = {}{:017b} ({})", intro_text, if m_x17 >= 0 {" "} else {"-"}, m_x17.abs(), m_x17);
    intro_text = format!("{}\ny_17 = {}{:017b} ({})", intro_text, if m_y17 >= 0 {" "} else {"-"}, m_y17.abs(), m_y17);
    intro_text = format!("{}\nx_32 = {}{:032b} ({})", intro_text, if m_x32 >= 0 {" "} else {"-"}, m_x32.abs(), m_x32);
    intro_text = format!("{}\ny_32 = {}{:032b} ({})", intro_text, if m_y32 >= 0 {" "} else {"-"}, m_y32.abs(), m_y32);
    println!("{}", intro_text);


    // =================================
    //  C: Evaluation

    let c_add  = pc.add(&c[0], &c[1])?;
    let c_sub  = pc.sub(&c[1], &c[0])?;
    //~ let c_sgn  = pc.sgn(&c[2]       )?;
    //~ let c_max  = pc.max(&c[1], &c[0])?;
    //~ let c_xy1  = pc.mul(&cx1,  &cy1 )?;
    //~ let c_xy4  = pc.mul(&cx4,  &cy4 )?;
    //~ let c_xy8  = pc.mul(&cx8,  &cy8 )?;
    //~ let c_xy16 = pc.mul(&cx16, &cy16)?;
    //~ let c_xy17 = pc.mul(&cx17, &cy17)?;
    //~ let c_xy32 = pc.mul(&cx32, &cy32)?;


    // =================================
    //  U: Decryption
    let m_add  = pu.decrypt(&c_add )?;
    let m_sub  = pu.decrypt(&c_sub )?;
    //~ let m_sgn  = pu.decrypt(&c_sgn )?;
    //~ let m_max  = pu.decrypt(&c_max )?;
    //~ let m_xy1  = pu.decrypt(&c_xy1 )?;
    //~ let m_xy4  = pu.decrypt(&c_xy4 )?;
    //~ let m_xy8  = pu.decrypt(&c_xy8 )?;
    //~ let m_xy16 = pu.decrypt(&c_xy16)?;
    //~ let m_xy17 = pu.decrypt(&c_xy17)?;
    //~ let m_xy32 = pu.decrypt(&c_xy32)?;

    let mut summary_text = format!("{} results", String::from("User:").bold().yellow());
    summary_text = format!("{}\nm_0 + m_1     = {:12} :: {} (exp. {} % {})", summary_text,
                            m_add,
                            if (m[0] + m[1] - m_add) % (1 << DEMO_BITLEN) == 0 {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            (m_as[0] + m_as[1]) % (1 << DEMO_BITLEN), 1 << DEMO_BITLEN
    );
    summary_text = format!("{}\nm_1 - m_0     = {:12} :: {} (exp. {} % {})", summary_text,
                            m_sub,
                            if (m[1] - m[0] - m_sub) % (1 << DEMO_BITLEN) == 0 {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            (m_as[1] - m_as[0]) % (1 << DEMO_BITLEN), 1 << DEMO_BITLEN
    );
    //~ summary_text = format!("{}\nsgn(m_2)      = {:12} :: {}", summary_text,
                            //~ m_sgn,
                            //~ if m_sgn == m[2].signum() {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
    //~ );
    //~ summary_text = format!("{}\nmax{{m_1, m_0}} = {:12} :: {} (exp. {} % {})", summary_text,
                            //~ m_max,
                            //~ if (std::cmp::max(m_as[1], m_as[0]) - m_max) % (1 << DEMO_BITLEN) == 0 {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            //~ std::cmp::max(m_as[1], m_as[0]), 1 << DEMO_BITLEN
    //~ );
    //~ summary_text = format!("{}\nx_1 × y_1     = {:12} :: {} (exp. {})", summary_text,
                            //~ m_xy1,
                            //~ if m_x1 * m_y1 == m_xy1 {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            //~ m_x1 * m_y1
    //~ );
    //~ summary_text = format!("{}\nx_4 × y_4     = {:12} :: {} (exp. {})", summary_text,
                            //~ m_xy4,
                            //~ if m_x4 * m_y4 == m_xy4 {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            //~ m_x4 * m_y4
    //~ );
    //~ summary_text = format!("{}\nx_8 × y_8     = {:12} :: {} (exp. {})", summary_text,
                            //~ m_xy8,
                            //~ if m_x8 * m_y8 == m_xy8 {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            //~ m_x8 * m_y8
    //~ );
    //~ summary_text = format!("{}\nx_16 × y_16   = {:12} :: {} (exp. {})", summary_text,
                            //~ m_xy16,
                            //~ if m_x16 * m_y16 == m_xy16 {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            //~ m_x16 * m_y16
    //~ );
    //~ summary_text = format!("{}\nx_17 × y_17   = {:12} :: {} (exp. {})", summary_text,
                            //~ m_xy17,
                            //~ if m_x17 * m_y17 == m_xy17 {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            //~ m_x17 * m_y17
    //~ );
    //~ summary_text = format!("{}\nx_32 × y_32   = {:24} :: {} (exp. {})", summary_text,
                            //~ m_xy32,
                            //~ if m_x32 * m_y32 == m_xy32 {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            //~ m_x32 * m_y32
    //~ );
    println!("{}", summary_text);


    // =================================
    println!("Demo END");
    // =================================

    Ok(())
}
