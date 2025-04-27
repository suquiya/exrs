use std::process::Command;

#[derive(Debug, Clone)]
pub struct Cmd {
    command: String,
    args: Vec<String>,
}

impl<T: Into<String>> From<Vec<T>> for Cmd {
    fn from(args: Vec<T>) -> Self {
        let args = args.into_iter().map(|s| s.into()).collect();
        Cmd::from_args(args).unwrap()
    }
}

pub fn get_args_from_line(cmd_line_str: &str) -> Vec<String> {
    let args = cmd_line_str
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    args
}

#[derive(Debug)]
pub enum Error {
    Parse(ParseError),
    Exec(ExecError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parse(e) => write!(f, "{}", e),
            Error::Exec(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParseError {
    EmptyCommand,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::EmptyCommand => write!(f, "empty command"),
        }
    }
}

#[derive(Debug)]
pub enum ExecError {
    IO(std::io::Error),
    Cmd(std::process::ExitStatus, String),
}

impl std::fmt::Display for ExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecError::IO(e) => write!(f, "error occured: {}", e),
            ExecError::Cmd(status, e) => {
                write!(f, "error occured: {} \n exit with status: {}", e, status)
            }
        }
    }
}

impl From<&str> for Cmd {
    fn from(cmd_str: &str) -> Self {
        Cmd::from_line_str(cmd_str).unwrap()
    }
}

impl Cmd {
    pub fn new(command: String, args: Vec<String>) -> Cmd {
        Cmd { command, args }
    }

    pub fn cmd(&self) -> Command {
        let mut cmd = Command::new(&self.command);
        cmd.args(&self.args);
        cmd
    }

    pub fn output(&self) -> std::io::Result<std::process::Output> {
        self.cmd().output()
    }

    pub fn get_line_str(&self) -> String {
        format!("{} {}", self.command, self.args.join(" "))
    }

    pub fn get_args(&self) -> Vec<String> {
        self.args.clone()
    }

    pub fn get_command(&self) -> String {
        self.command.clone()
    }

    pub fn run(&self) -> Result<String, ExecError> {
        match self.output() {
            Ok(output) => {
                if !output.status.success() {
                    return Err(ExecError::Cmd(
                        output.status,
                        String::from_utf8_lossy(&output.stderr).to_string(),
                    ));
                }
                let output_str = String::from_utf8_lossy(&output.stdout).to_string();
                Ok(output_str)
            }
            Err(e) => Err(ExecError::IO(e)),
        }
    }

    pub fn from_line_str(cmd_line_str: &str) -> Result<Cmd, ParseError> {
        let args = get_args_from_line(cmd_line_str);
        Cmd::from_args(args)
    }

    pub fn from_args(args: Vec<String>) -> Result<Cmd, ParseError> {
        let mut iter = args.into_iter();
        if iter.len() < 1 {
            return Err(ParseError::EmptyCommand);
        }
        let command: String = iter.next().unwrap();
        let args: Vec<String> = iter.collect();
        Ok(Cmd::new(command, args))
    }

    pub fn from_str_args(args: Vec<&str>) -> Result<Cmd, ParseError> {
        let mut iter = args.into_iter();
        if iter.len() < 1 {
            return Err(ParseError::EmptyCommand);
        }
        let command: String = iter.next().unwrap().to_string();
        let args: Vec<String> = iter.map(|s| s.to_string()).collect();
        Ok(Cmd::new(command, args))
    }
}

pub fn exes_with_result<T: Into<String>>(args: Vec<T>) -> Result<String, Error> {
    Cmd::from_args(args.into_iter().map(|s| s.into()).collect())
        .map_err(Error::Parse)?
        .run()
        .map_err(Error::Exec)
}

pub fn exes<T: Into<String>>(args: Vec<T>) -> String {
    exes_with_result(args).unwrap_or_else(|e| e.to_string())
}

#[macro_export]
macro_rules! exes {
    ($($arg:expr),*) => {
        exes(vec![$($arg),*])
    };
}

#[macro_export]
macro_rules! exes_with_result {
    ($($arg:expr),*) => {
        exes_with_result(vec![$($arg),*])
    };
}

#[macro_export]
macro_rules! cmd{
    ($($arg:expr),*) => {
        Cmd::from_str_args(vec![$($arg),*])
    };
}

pub fn exli_with_result(cmd_line_str: &str) -> Result<String, Error> {
    Cmd::from_line_str(cmd_line_str)
        .map_err(Error::Parse)?
        .run()
        .map_err(Error::Exec)
}

pub fn exli(cmd_line_str: &str) -> String {
    exli_with_result(cmd_line_str).unwrap_or_else(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cmd_from_str() {
        let cmd = Cmd::from("git --version");
        assert_eq!(cmd.command, "git");
        assert_eq!(cmd.args, vec!["--version"]);
    }

    #[test]
    fn test_exli_with_result() {
        let result = exli_with_result("git --version");
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("git version"));
    }

    #[test]
    fn exes_test() {
        let result = exes(vec!["git", "--version"]);
        assert!(result.starts_with("git version"));
    }

    #[test]
    fn macros_test() {
        let result = exes!["git", "--version"];
        assert!(result.starts_with("git version"));
        let result2 = exes_with_result!("git", "--version");
        assert!(result2.is_ok());
        assert!(result2.unwrap().starts_with("git version"));

        let cmd = cmd!["git", "--version"];
        assert!(cmd.is_ok());
        assert_eq!(cmd.clone().unwrap().command, "git");
        assert_eq!(cmd.unwrap().args, vec!["--version"]);
    }
}
