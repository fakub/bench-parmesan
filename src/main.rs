use std::error::Error;
use colored::Colorize;

use parmesan::params;
use parmesan::ciphertexts::ParmCiphertext;

use parmesan::userovo::*;
use parmesan::ParmesanUserovo;

use parmesan::cloudovo::*;
use parmesan::ParmesanCloudovo;

fn main() {



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
    let mp: Vec<i32> = vec![1,0,1,-1,-1,0,-1,1,1,-1,1,1,1,-1,-1,0,0,1,1,0,0,0,0,-1,0,0,0,0,0,-1,0,0,];
    let mn: Vec<i32> = vec![-1,0,0,-1,1,1,-1,1,-1,0,0,1,0,1,1,0,0,0,-1,0,0,1,0,0,-1,0,-1,-1,-1,1,1,0,];
    //~ let mut mp: Vec<i32> = vec![1,0,0,1,1,0,1,1,1,0,1,1,]; mp.reverse();   // [0,1,1,1,1,1,1,0,1,1,0,0,1,0,0,1,0,0,1,1,1,0,0,1,1,0,1,1,1,0,1,1,];
    //~ let mut mn: Vec<i32> = vec![1,1,1,1,0,0,1,0,0,0,0,0,]; mn.reverse();   // [0,0,1,1,0,0,1,0,0,0,1,1,1,1,1,0,0,1,1,0,1,1,1,1,0,0,1,0,0,0,0,0,];
    let mp_val = encryption::convert(&mp).expect("encryption::convert failed.");
    let mn_val = encryption::convert(&mn).expect("encryption::convert failed.");
    println!("mp_in = {:12}",   mp_val);
    println!("mn_in = {:12}\n", mn_val);
    let cp = pu.encrypt_vec(&mp).expect("encrypt_vec failed.");
    let cn = pu.encrypt_vec(&mn).expect("encrypt_vec failed.");

    //~ //DBG
    //~ let cp = pu.encrypt(2491, 12).expect("encrypt_vec failed.");
    //~ let cn = pu.encrypt(3872, 12).expect("encrypt_vec failed.");
    //~ let mp_d = pu.decrypt(&cp).expect("decrypt failed.");
    //~ let mn_d = pu.decrypt(&cn).expect("decrypt failed.");
    //~ println!("mp_dc = {}", mp_d);
    //~ println!("mn_dc = {}\n", mn_d);


    // =========================================================================
    //  Addition

    // first-level addition/subtraction:   a + b   ,   c + d
    parmesan::simple_duration!(
        ["1st level addition"],
        [
            let c_a1 = pc.add(&cp, &cn).expect("add failed.");
        ]
    );
    parmesan::simple_duration!(
        ["1st level subtraction"],
        [
            let c_s1 = pc.sub(&cp, &cn).expect("sub failed.");
        ]
    );

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
    let mut summary_text = format!("\n{}\n", String::from("Results:").bold().yellow());
    summary_text = format!("{}\nmp + mn     = {:12} :: {} (exp. {} % {})", summary_text,
                            m_a1,
                            if (mp_val + mn_val - m_a1) % (1 << 32) == 0 {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            (mp_val + mn_val) % (1 << 32), 1u64 << 32
    );
    summary_text = format!("{}\nmp - mn     = {:12} :: {} (exp. {} % {})", summary_text,
                            m_s1,
                            if (mp_val - mn_val - m_s1) % (1 << 32) == 0 {String::from("PASS").bold().green()} else {String::from("FAIL").bold().red()},
                            (mp_val - mn_val) % (1 << 32), 1u64 << 32
    );   //TODO something like DEMO_BITLEN
    println!("{}\n", summary_text);

}
