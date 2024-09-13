use core::fmt;
use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
};

const GIT_CONF: &str = ".gitconfig";
const HELP: &str = r#"gitp is a simple tool to switch between your git profiles.

Commands:

-h            Check all the commands and usage.
{filename}    Pass the name of that you want to be swapped with your current .gitconfig
"#;

#[derive(Debug)]
enum AppErr {
    CommandLineArgs,
    FileCopyErr(std::io::Error),
    FileOpenErr(std::io::Error),
    FileLineReadErr(std::io::Error),
    FileNameOrEmailMissing,
}

impl fmt::Display for AppErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppErr::CommandLineArgs => write!(
                f,
                "Please provide a valid argument. Use -h to understand more."
            ),
            AppErr::FileCopyErr(err) => write!(f, "Error copying file: {}", err),
            AppErr::FileOpenErr(err) => write!(f, "Error opening file: {}", err),
            AppErr::FileLineReadErr(err) => write!(f, "Error reading file: {}", err),
            AppErr::FileNameOrEmailMissing => write!(f, "Name or email is missing."),
        }
    }
}

impl std::error::Error for AppErr {}

struct Profile {
    name: String,
    email: String,
}

impl Profile {
    fn new(name: String, email: String) -> Self {
        Self { name, email }
    }

    fn detail(&self) -> String {
        format!(
            "\x1b[33musing {} with name {} & email {}\x1b[0m",
            GIT_CONF, self.name, self.email
        )
    }
}

fn read_file(path: &str) -> Result<Profile, AppErr> {
    let mut name: Option<String> = None;
    let mut email: Option<String> = None;

    match File::open(path) {
        Ok(file) => {
            let buf_reader = BufReader::new(file);

            for line in buf_reader.lines() {
                if name.is_some() && email.is_some() {
                    break;
                }

                match line {
                    Ok(l) => {
                        if l.contains("name") {
                            let (_, v) = l.split_once("=").unwrap_or(("", ""));
                            name = Some(v.trim().to_string())
                        }
                        if l.contains("email") {
                            let (_, v) = l.split_once("=").unwrap_or(("", ""));
                            email = Some(v.trim().to_string())
                        }
                    }
                    Err(e) => return Err(AppErr::FileLineReadErr(e)),
                }
            }

            if let (Some(n), Some(e)) = (name, email) {
                Ok(Profile::new(n, e))
            } else {
                Err(AppErr::FileNameOrEmailMissing)
            }
        }
        Err(e) => Err(AppErr::FileOpenErr(e)),
    }
}

fn handle_cmd(cmd: &String) -> Result<String, AppErr> {
    match cmd.as_str().trim() {
        "-h" => Ok(HELP.to_string()),
        c => {
            match fs::copy(c, GIT_CONF) {
                Ok(_) => {
                    let read_config = read_file(c)?;
                    Ok(read_config.detail())
                },
                Err(e) => Err(AppErr::FileCopyErr(e)),
            }
        }
    }
}

fn app() -> Result<String, AppErr> {
    let cmd = env::args().nth(1).ok_or(AppErr::CommandLineArgs)?;
    handle_cmd(&cmd)
}

fn main() {
    match app() {
        Ok(r) => println!("{}", r),
        Err(e) => println!("\x1b[31m{}\x1b[0m", e.to_string()),
    }
}
