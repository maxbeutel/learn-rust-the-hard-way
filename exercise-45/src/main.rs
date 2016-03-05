use std::io;
use std::io::prelude::*;

//use std::net::{IoResult, File, TcpStream};
use std::net::TcpStream;

//use url::Url;

// fn download_file(url: &Url, file: &Path) {

// }

fn download_file() {

}

fn main() {
    let mut stream = match TcpStream::connect("127.0.0.1:80") {
        Ok(s) => s,
        Err(_) => panic!("Connection failed"),
    };

    // let stdin = io::stdin();

    println!("OK");


    // for line in stdin.lock().lines() {
    //     println!("|{}|", line.unwrap());
    // }
}
