mod core;

fn main() {
    println!("{:?}", core::eval::eval("(car (cdr '(10 20 30)))"));
    println!("{:?}", core::eval::eval("(+ 10 20)"));
}
