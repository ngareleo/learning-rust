use std::{error::Error, fs, env};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(data) => data,
            None => return Err("Query not found"),

        };

        let path = match args.next() {
            Some(data) => data,
            None => return Err("File location not found")
        };

        let case_sensitive = env::var("IGNORE_CASE").is_ok();

        Ok(Self{
            query, 
            path,
            case_sensitive
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(&config.path)?;
    let result = if config.case_sensitive {
        search(&config.query, &content)
    }else {
        search_case_insensitive(&config.query, &content)
    };
    
    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &'a String, content: &'a String) -> Vec<&'a str> {
    content.lines().filter(|line| {line.contains(query)}).collect()
}


pub fn search_case_insensitive<'a>(query: &'a String, content: &'a String) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.to_uppercase().contains(&query.to_uppercase()))
        .collect()
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn search_string_with_string_present() {
        let query = "long live asap";
        let content = "/
            my name is asap, long live asap,
        ";
        assert_eq!(search(&query.to_string(), &content.to_string()), ["my name is asap, long live asap,"]);
    }

    
    #[test]
    fn search_string_with_missing_string() {
        let v: Vec<&str> = vec![];
        let query = "dudes are lost";
        let content = "/
            my name is asap, long live asap,
        ";
        assert_eq!(search(&query.to_string(), &content.to_string()), v);
    }
}