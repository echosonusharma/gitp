use core::fmt;
use std::{env, fs};

const GIT_CONF: &str = ".gitconfig";
const HELP: &str = r#"gitp is a simple tool to switch between your git profiles.

Commands:

-h            Check all the commands and usage.
{filename}    Pass the name of that you want to be swapped with your current .gitconfig
"#;

#[derive(Debug)]
enum AppErr {
    CommandLineArgs,
    FileCopyErr(std::io::Error)
}

impl fmt::Display for AppErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppErr::CommandLineArgs => write!(f, "Please provide a valid argument. Use -h to understand more."),
            AppErr::FileCopyErr(err) => write!(f, "Error copying file: {}", err)
        }
    }
}

impl std::error::Error for AppErr {}

fn handle_cmd(cmd: &String)  -> Result<String, AppErr> {
    match cmd.as_str().trim() {
        "-h" => {
            Ok(HELP.to_string())
        },
        c => {
            match fs::copy(c, GIT_CONF) {
                Ok(_) => {
                    Ok("done ^^;".to_string())
                },
                Err(e) => Err(AppErr::FileCopyErr(e))
            }
        }
    }
}

fn app() -> Result<String, AppErr>{
    let cmd = env::args().nth(1).ok_or(AppErr::CommandLineArgs)?;
    handle_cmd(&cmd)
}

fn main() {
    match app() {
        Ok(r) => println!("{}", r),
        Err(e) => println!("{}", e.to_string()),
    }
}
