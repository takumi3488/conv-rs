use std::{
    fs::File,
    io::{Read, Write},
};

use clap::Parser;
use regex::Regex;
pub mod csv;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // Source File path
    path: String,

    // Target extension
    target: String,
}

pub struct FileName {
    stem: String,
    extension: String,
}

impl FileName {
    // Split into extensions and others.
    fn new(path: &str) -> Self {
        let re = Regex::new(r"^(.*)\.([^\.]+)$").unwrap();
        match re.captures(path) {
            Some(caps) => Self {
                stem: caps[1].to_string(),
                extension: caps[2].to_string(),
            },
            None => panic!("extension not identified"),
        }
    }

    // Return full filename.
    fn full_path(&self) -> String {
        format!("{}.{}", self.stem, self.extension)
    }

    // read file content
    fn read(&self) -> String {
        let mut f = File::open(self.full_path()).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        contents
    }
}

pub fn main() {
    let args = Args::parse();
    let file = FileName::new(&args.path);
    let res = run(&file, &args.target);
    let target_path: &str = &format!("{}.{}", &file.stem, &args.target);

    // create target file
    let mut new_file = match File::create(target_path) {
        Err(why) => panic!("{}", why),
        Ok(f) => f,
    };
    if let Err(why) = new_file.write_all(res.as_bytes()) {
        panic!("{}", why);
    }

    println!("Successfully created: {}", target_path);
}

/// Convert the contents of file to target format.
///
/// # Examples
/// 
/// ```
/// let res: &str  = &run("test.csv", "html");
/// assert_eq!(res, "<table><thead><tr><th>a</th><th>total</th></tr></thead><tbody>\
///       <tr><td>3</td><td>B</td></tr><tr><td>9</td><td>E</td></tr></tbody></table>");
/// ```
pub fn run(file: &FileName, target: &str) -> String {
    let extension: &str = &file.extension;
    let res = match extension {
        "csv" => csv::convert(&file.read(), target),
        _ => panic!("{}", format!("{} is not supported", file.extension)),
    };
    match res {
        Err(why) => panic!("{}", why),
        Ok(s) => s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filename_struct_works() {
        let f1 = FileName::new("test.csv");
        assert_eq!(f1.full_path(), "test.csv");
        assert_eq!(f1.stem, "test");
        assert_eq!(f1.extension, "csv");

        let f2 = FileName::new("test.csv.html");
        assert_eq!(f2.full_path(), "test.csv.html");
        assert_eq!(f2.stem, "test.csv");
        assert_eq!(f2.extension, "html");
    }

    #[test]
    #[should_panic]
    fn filename_without_extension() {
        FileName::new("test");
    }

    #[test]
    #[should_panic]
    fn unsupported_target_extension() {
        run(&FileName::new("test.cc"), "rb");
    }
}
