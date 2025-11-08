use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file_path)?;
    let matching_lines = search(&config.query, &file_contents);
    println!("{:#?}", matching_lines);
    Ok(())
}

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("2 arguments required: <search_term>, <path_to_file>");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Self { query, file_path })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matching_lines = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            matching_lines.push(line);
        }
    }
    matching_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let query = "frog";
        let contents = "\
How dreary to be somebody!
How public, like a frog
To tell your name the livelong day";
        assert_eq!(vec!["How public, like a frog"], search(query, contents));
    }

    #[test]
    fn nobody() {
        let file_contents = fs::read_to_string("poem.txt").unwrap();
        let matching_lines = search("nobody", &file_contents);
        assert_eq!(
            vec!["I'm nobody! Who are you?", "Are you nobody, too?"],
            matching_lines
        );
    }

    #[test]
    fn the() {
        let file_contents = fs::read_to_string("poem.txt").unwrap();
        let matching_lines = search("the", &file_contents);
        assert_eq!(
            vec![
                "Then there's a pair of us - don't tell!",
                "To tell your name the livelong day"
            ],
            matching_lines
        );
    }

    #[test]
    fn a() {
        let file_contents = fs::read_to_string("poem.txt").unwrap();
        let matching_lines = search("a", &file_contents);
        assert_eq!(
            vec![
                "I'm nobody! Who are you?",
                "Then there's a pair of us - don't tell!",
                "They'd banish us, you know.",
                "How dreary to be somebody!",
                "How public, like a frog",
                "To tell your name the livelong day",
                "To an admiring bog!",
            ],
            matching_lines
        );
    }
}
