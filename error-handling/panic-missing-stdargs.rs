use std::env;

fn main() {
    let mut argv = env::args();

    // let arg: String = argv.nth(1).unwrap(); // error 1
    // let n: u32 = arg.parse().unwrap(); // error 2
    let arg = match argv.nth(1) {
        Option::Some(s) => s,
        Option::None => panic!("No argument given."),
    };


    let n: u32 = match arg.parse() {
        Ok(s) => s,
        Err(_) => panic!("No integer value given -> '{}'.", _),
    };


    let result = n * 2;
    println!("2 x {} = {}", n, result);
}
