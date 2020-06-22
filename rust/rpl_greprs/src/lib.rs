use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config{
    pub query: String,
    pub filename: String,
}

impl Config{
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(v) => v,
            None => return Err("invalid args"),
        };
        let filename = match args.next() {
            Some(v) => v,
            None => return Err("invalid args"),
        };
        Ok(Config{ query,filename,})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let mut f = File::open(config.filename)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    for line in search(&config.query, &content) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
    /*
    let mut v = Vec::new();

    for line in contents.lines(){
        if line.contains(query) {
            v.push(line);
        }
    }
    v*/
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }
}
