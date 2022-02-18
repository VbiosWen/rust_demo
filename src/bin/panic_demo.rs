use std::fs::{File, read};
use std::io::{ErrorKind, Read};
use std::{io, fs};

fn main() {
    let result = read_username_from_file();
    match result {
        Ok(st) => println!("{:?}", st),
        Err(e) => println!("{:?}", e),
    }
    let f = File::open("/Users/tengmu/test.cppo");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("/Users/tengmu/test.cppo") {
                Ok(file) => file,
                Err(error) => panic!("create error,{:?}", error)
            },
            other_error => panic!("open file error,{:?}", other_error),
        },
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("/Users/tengmu/test.cppo")?.read_to_string(&mut s)?;
    Ok(s)
}


fn read_username_from_file1() -> Result<String, io::Error> {
    fs::read_to_string("/Users/tengmu/test.cppo")
}