use crate::cmd::{Cmd, ExecError, get_args_from_line};

#[cfg(target_os = "windows")]
fn cmd_sh_from_args(mut args: Vec<String>) -> Cmd {
    let cmd = "cmd".to_string();
    let prefix = "/c".to_string();
    args.insert(0, prefix);
    Cmd::new(cmd, args)
}

#[cfg(not(target_os = "windows"))]
fn cmd_sh_from_args(args: Vec<String>) -> Cmd {
    let cmd = "sh".to_string();
    let prefix = "-c".to_string();
    args.insert(0, prefix);
    Cmd::new(cmd, args)
}

pub fn sh_with_result<T: Into<String>>(args: Vec<T>) -> Result<String, ExecError> {
    cmd_sh_from_args(args.into_iter().map(|s| s.into()).collect()).run()
}

pub fn sh<T: Into<String>>(args: Vec<T>) -> String {
    sh_with_result(args).unwrap_or_else(|e| e.to_string())
}

#[macro_export]
macro_rules! sh {
    ($($arg:expr),*) => {
        sh(vec![$($arg),*])
    };
}

#[macro_export]
macro_rules! sh_with_result {
    ($($arg:expr),*) => {
        sh_with_result(vec![$($arg),*])
    };
}

pub fn shli_with_result(cmd_line_str: &str) -> Result<String, ExecError> {
    let args = get_args_from_line(cmd_line_str);
    let cmd = cmd_sh_from_args(args);

    cmd.run()
}

pub fn shli(cmd_line_str: &str) -> String {
    shli_with_result(cmd_line_str).unwrap_or_else(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sh_test() {
        let result = sh(vec!["echo", "hello"]);
        assert_eq!(result.trim_end(), "hello");
    }

    #[test]
    fn shli_test() {
        let result = shli("echo hello");
        assert_eq!(result.trim_end(), "hello");
    }
}
