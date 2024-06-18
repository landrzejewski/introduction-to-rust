use std::env;

const SEPARATOR: &str = " ";

pub fn run() {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    println!("{}", arguments.join(SEPARATOR));
}
