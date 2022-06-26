mod core;

fn main() {
    println!(
        "expr=\"(car (cdr '(10 20 30)))\" \nresult=\"{:?}\"\n",
        core::eval::eval("(car (cdr '(10 20 30)))")
    );
    println!(
        "expr=\"(+ 10 20)\"\n result=\"{:?}\"\n",
        core::eval::eval("(+ 10 20)")
    );
    println!(
        "expr=\"(- 10 20)\"\n result=\"{:?}\"\n",
        core::eval::eval("(- 10 20)")
    );
    println!(
        "expr=\"(- 20 10)\"\n result=\"{:?}\"\n",
        core::eval::eval("(- 20 10)")
    );
    println!(
        "expr=\"(* 20 10)\"\n result=\"{:?}\"\n",
        core::eval::eval("(* 20 10)")
    );
    println!(
        "expr=\"(/ 20 10)\"\n result=\"{:?}\"\n",
        core::eval::eval("(/ 20 10)")
    );
    println!(
        "expr=\"(/ 10 20)\"\n result=\"{:?}\"\n",
        core::eval::eval("(/ 10 20)")
    );
}
