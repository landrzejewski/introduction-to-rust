use regex::Regex;
use walkdir::{DirEntry, WalkDir};

use crate::exercises::utils::{assert, get_arguments, is_not_empty, min_length};

const ELEMENT_TYPE_SEPARATOR: &str = ",";

fn show_help() {
    println!("Usage:");
    println!("find regexp t1,t2,t3 path1 path2 ...");
    println!("Args:");
    println!("  regexp - match/regular expression");
    println!("  types - one or many types separated by comma. Types: dir,file,link");
}

enum ElementType {
    Dir,
    File,
    Link,
}

impl ElementType {
    fn from(text: &str) -> Option<Self> {
        match text {
            "dir" => Some(ElementType::Dir),
            "file" => Some(ElementType::File),
            "link" => Some(ElementType::Link),
            _ => None,
        }
    }
}

fn is_type_of(entry: &DirEntry, element_type: &ElementType) -> bool {
    match element_type {
        ElementType::Dir => entry.file_type().is_dir(),
        ElementType::File => entry.file_type().is_file(),
        ElementType::Link => entry.file_type().is_symlink(),
    }
}

fn find(file_name_regex: &Regex, element_types: &Vec<ElementType>, paths: &Vec<String>) -> Vec<String> {
    let element_type_filter = |entry: &DirEntry| {
        element_types
            .iter()
            .any(|element_type| is_type_of(entry, element_type))
    };

    let name_filter = |entry: &DirEntry| {
        let Some(file_name) = entry.file_name().to_str() else {
            return false;
        };
        file_name_regex.is_match(file_name)
    };

    let find_on_path = |path: &String| {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|result| result.ok())
            .filter(element_type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
    };

    println!("Searching...");

    let mut result = Vec::new();

    paths.iter().fold(&mut result, |result, path| {
        result.extend(find_on_path(path));
        result
    });

    return result;
}

pub fn run() {
    let arguments = get_arguments();
    assert(&arguments, min_length(3), show_help);

    let regex = Regex::new(&arguments[0]).expect("Invalid regexp expression");

    let element_types = arguments[1]
        .split(ELEMENT_TYPE_SEPARATOR)
        .map(|element| element.trim())
        .filter_map(|element| ElementType::from(element))
        .collect::<Vec<_>>();
    assert(&element_types, is_not_empty(), show_help);
    let paths = arguments.iter().skip(2).cloned().collect::<Vec<_>>();
    assert(&paths, is_not_empty(), show_help);

    find(&regex, &element_types, &paths)
        .iter()
        .for_each(|path| println!("{path}"));
}
