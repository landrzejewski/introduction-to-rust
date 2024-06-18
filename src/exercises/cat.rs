use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const WITH_LINE_NUMBERS_PARAM: &str = "-n";
const WITH_LINE_NUMBERS_EXCLUDING_EMPTY_LINES_PARAM: &str = "-nb";

#[derive(PartialEq)]
enum Mode {
    WithLineNumbers(bool),
    WithoutLineNumbers,
}

fn get_mode(arguments: &Vec<String>) -> Mode {
    match arguments[0].as_str() {
        WITH_LINE_NUMBERS_PARAM => Mode::WithLineNumbers(true),
        WITH_LINE_NUMBERS_EXCLUDING_EMPTY_LINES_PARAM => Mode::WithLineNumbers(false),
        _ => Mode::WithoutLineNumbers,
    }
}

fn get_file_names(mode: &Mode, arguments: &Vec<String>) -> Vec<String> {
    if *mode == Mode::WithoutLineNumbers {
        arguments.clone()
    } else {
        arguments.iter().skip(1).cloned().collect()
    }
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

fn print_line_with_line_numbers(line_number: &mut i32, line: &String) {
    *line_number = *line_number + 1;
    println!("{:6}:\t{}", line_number, line);
}

fn print_line_with_line_numbers_without_empty_lines(line_number: &mut i32, line: &String) {
    if !line.is_empty() {
        *line_number = *line_number + 1;
        println!("{:6}:\t{}", line_number, line);
    } else {
        println!();
    }
}

fn print_line(_line_number: &mut i32, line: &String) {
    println!("{}", line)
}

fn print_file_lines(mode: &Mode, file: &File) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(file);

    let print = match mode {
        Mode::WithLineNumbers(true) => print_line_with_line_numbers,
        Mode::WithLineNumbers(false) => print_line_with_line_numbers_without_empty_lines,
        Mode::WithoutLineNumbers => print_line,
    };

    let mut line_number = 0;
    for (_, line) in reader.lines().enumerate() {
        print(&mut line_number, &line?);
    }
    Ok(())
}

fn cat(mode: &Mode, file_names: &Vec<String>) {
    for file_name in file_names {
        println!("File: {file_name}");
        match File::open(file_name) {
            Ok(file) => {
                if let Err(error) = print_file_lines(&mode, &file) {
                    eprintln!("Failed read from file {file_name} ({error})")
                }
            }
            Err(error) => eprintln!("Failed to open file {file_name} ({error})"),
        }
        println!();
    }
}

pub fn run() {
    let arguments = get_arguments();
    if arguments.is_empty() {
        show_help();
        return;
    }
    let mode = get_mode(&arguments);
    let file_names = get_file_names(&mode, &arguments);
    cat(&mode, &file_names);
}
