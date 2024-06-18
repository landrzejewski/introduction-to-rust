use std::env;

#[derive(PartialEq)]
enum Mode {
    WithLineNumbers(bool),
    WithoutLineNumbers,
}

fn show_help() {
    println!("Usage:");
    println!("cat [args] file1 file2 ...");
    println!("Args:");
    println!("  -n - show line numbers");
    println!("  -nb - show line numbers excluding blank lines");
}

fn get_arguments() -> Vec<String> {
    env::args().skip(1).collect::<Vec<_>>()
}

fn cat(mode: &Mode, file_names: &Vec<String>) {

}

pub fn run() {
    let arguments = get_arguments();
    if arguments.is_empty() {
        show_help();
        return;
    }
    // cat(&mode, $file_name)
}
