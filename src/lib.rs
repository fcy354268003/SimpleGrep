use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub fn run() {
    let grep_cmd = GrepCmd::init(&env::args().collect()).unwrap_or_else(|error_info| {
        println!("parase parameter wrong:{}", error_info);
        process::exit(1);
    });
    if let Err(e) = read_target_file(&grep_cmd.file_path) {
        println!("there was an error when read the target file");
        process::exit(1);
    }
}

#[derive(Debug)]
pub struct GrepCmd {
    pub query: String,
    pub file_path: String,
}

impl GrepCmd {
    pub fn init(cmd: &Vec<String>) -> Result<GrepCmd, &'static str> {
        if cmd.len() < 3 {
            return Err("not enough parameters");
        }
        let res = GrepCmd {
            query: String::from(&cmd[1]),
            file_path: String::from(&cmd[2]),
        };
        Ok(res)
    }
}

fn read_target_file(path: &String) -> Result<String, Box<dyn Error>> {
    let res = fs::read_to_string(path)?;
    println!("读取到的文件内容:\n{}",res);
    Ok(res)
}