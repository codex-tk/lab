use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config<'a>{
    pub query: &'a str,
    pub filename: &'a str,
}

impl<'a> Config<'a>{
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("invalid arg")
        }
        let query = &args[1];
        let filename = &args[2];
        Ok(Config{ query,filename,})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let mut f = File::open(config.filename)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    for line in search(config.query, &content) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut v = Vec::new();
    for line in contents.lines(){
        if line.contains(query) {
            v.push(line);
        }
    }
    v
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
