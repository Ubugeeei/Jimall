mod core;
use crate::core::evaluation::eval::eval;

fn main() {
    /*
     * four arithmetic operations
     */
    // add
    println!("expr = (+ 10 20)\nresult = {:?}\n", eval("(+ 10 20)"));

    // sub
    println!("expr = (- 10 20)\nresult = {:?}\n", eval("(- 10 20)"));

    // sub (signed)
    println!("expr = (- 20 10)\nresult = {:?}\n", eval("(- 20 10)"));

    // mul
    println!("expr = (* 20 10)\nresult = {:?}\n", eval("(* 20 10)"));

    // div int
    println!("expr = (/ 20 10)\nresult = {:?}\n", eval("(/ 20 10)"));

    // div (float)
    println!("expr = (/ 10 20)\nresult = {:?}\n", eval("(/ 10 20)"));

    /*
     * list operations
     */
    println!(
        "expr = (cdr '(10 20 30)) \nresult = {:?}\n",
        eval("(cdr '(10 20 30))")
    );
    println!(
        "expr = (car '(10 20 30)) \nresult = {:?}\n",
        eval("(car '(10 20 30))")
    );
    println!(
        "expr = (car (cdr '(10 20 30))) \nresult = {:?}\n",
        eval("(car (cdr '(10 20 30)))")
    );
}
