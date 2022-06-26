mod core;

fn main() {
    /*
     * four arithmetic operations
     */
    // add
    println!(
        "expr=\"(+ 10 20)\"\n result=\"{:?}\"\n",
        core::eval::eval("(+ 10 20)")
    );

    // sub
    println!(
        "expr=\"(- 10 20)\"\n result=\"{:?}\"\n",
        core::eval::eval("(- 10 20)")
    );

    // sub (signed)
    println!(
        "expr=\"(- 20 10)\"\n result=\"{:?}\"\n",
        core::eval::eval("(- 20 10)")
    );

    // mul
    println!(
        "expr=\"(* 20 10)\"\n result=\"{:?}\"\n",
        core::eval::eval("(* 20 10)")
    );

    // div int
    println!(
        "expr=\"(/ 20 10)\"\n result=\"{:?}\"\n",
        core::eval::eval("(/ 20 10)")
    );

    // div (float)
    println!(
        "expr=\"(/ 10 20)\"\n result=\"{:?}\"\n",
        core::eval::eval("(/ 10 20)")
    );

    /*
     * list operations
     */
    println!(
        "expr=\"(cdr '(10 20 30))\" \nresult=\"{:?}\"\n",
        core::eval::eval("(cdr '(10 20 30))")
    );
    println!(
        "expr=\"(car '(10 20 30))\" \nresult=\"{:?}\"\n",
        core::eval::eval("(car '(10 20 30))")
    );
    println!(
        "expr=\"(car (cdr '(10 20 30)))\" \nresult=\"{:?}\"\n",
        core::eval::eval("(car (cdr '(10 20 30)))")
    );
}
