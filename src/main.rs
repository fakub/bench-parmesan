use colored::Colorize;

fn main() {

    parmesan::simple_duration!(
        ["bench {}", 42],
        [let _x = 0;]
    );

    //TODO  generate a set of random messages of different lengths (possibly directly from alphabet),
    //      benchmark addition, subtraction, signum, maximum, multiplication, (scalar multiplication, NN evaluation)


    // =========================================================================
    //  Generate & Encrypt inputs

    // encrypt 4 random 32-word sequences of {-1,0,1}


    // =========================================================================
    //  Addition

    // first-level addition:   a + b   ,   c + d

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

}
