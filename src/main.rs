use colored::Colorize;



fn main() {

    parmesan::simple_duration!(
        ["bench {}", 42],
        [let _x = 0;]
    );

    //TODO  generate a set of random messages of different lengths (possibly directly from alphabet),
    //      benchmark addition, subtraction, signum, maximum, multiplication, (scalar multiplication, NN evaluation)

    let _p = parmesan::arith_demo();

    let _a = parmesan::nn_demo();
}
