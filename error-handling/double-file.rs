use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> i32 {
    let mut file = File::open(file_path).unwrap(); // error 1

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap(); // error 2

    let n = contents.trim().parse::<i32>().unwrap(); // error 3
    2 * n
}

fn main() {
    let doubled = file_double("file_to_double");
    println!("Result is {}", doubled);
}
