use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;


#[allow(dead_code)]
pub fn error_main(){
    let f = File::open("Hello");
    let _f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound  => {
            match File::create("hello") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("File Create Error {:?}", e)
                }
            }
        },
        Err(error) => {
            panic!("error {:?}", error)
        }
    };
    
    match read_from_file() {
        Ok(s) => println!("Read {}", s),
        Err(e) => println!("Error {:?}", e),
    }
    match read_from_fileq() {
        Ok(s) => println!("Read {}", s),
        Err(e) => println!("Error {:?}", e),
    }
    /*
    if let Ok(s) = r {
        println!("Ok {}", s);
    } else {
        println!("error");
    }*/
} 

fn read_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => return Ok(s),
        Err(e) => return Err(e),
    }
}

fn read_from_fileq() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_from_file_chain() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}