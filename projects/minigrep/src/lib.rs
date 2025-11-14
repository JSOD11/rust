use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file_path)?;
    let matching_lines = if config.ignore_case {
        case_insensitive_search(&config.query, &file_contents)
    } else {
        search(&config.query, &file_contents)
    };
    println!("{:#?}", matching_lines);
    Ok(())
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>,
    {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("2 arguments required: <search_term>, <path_to_file>"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("2 arguments required: <search_term>, <path_to_file>"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lowercase_query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&lowercase_query))
        .collect()
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

    #[test]
    fn case_insensitive_the() {
        let file_contents = fs::read_to_string("poem.txt").unwrap();
        let matching_lines = case_insensitive_search("tHe", &file_contents);
        assert_eq!(
            vec![
                "Then there's a pair of us - don't tell!",
                "They'd banish us, you know.",
                "To tell your name the livelong day"
            ],
            matching_lines
        );
    }
}
