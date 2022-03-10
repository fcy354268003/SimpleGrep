use std::env;
use std::error::Error;
use std::fs;
use std::process;

#[cfg(test)]
mod tests;

pub fn run() {
    let grep_cmd = GrepCmd::new(&env::args().collect()).unwrap_or_else(|error_info| {
        println!("parase parameter wrong:{}", error_info);
        process::exit(1);
    });
    match read_target_file(&grep_cmd.file_path) {
        Err(_e) => {
            println!("there was an error when read the target file");
            process::exit(1);
        }
        Ok(res) => {
            let target_lines = if grep_cmd.case_sensitive {
                search(&grep_cmd.query, &res)
            } else {
                search_insensitive(&grep_cmd.query, &res)
            };
            for target in target_lines {
                println!("{}", target);
            }
        }
    }
}

#[derive(Debug)]
pub struct GrepCmd {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl GrepCmd {
    pub fn new(cmd: &Vec<String>) -> Result<GrepCmd, &'static str> {
        if cmd.len() < 3 {
            return Err("not enough parameters");
        }
        let is_case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        let res = GrepCmd {
            query: String::from(&cmd[1]),
            file_path: String::from(&cmd[2]),
            case_sensitive: is_case_sensitive,
        };
        Ok(res)
    }
}

fn read_target_file(path: &String) -> Result<String, Box<dyn Error>> {
    let res = fs::read_to_string(path)?;
    // println!("读取到的文件内容:\n{}", res);
    Ok(res)
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
}
