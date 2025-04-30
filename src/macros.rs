/// Run command with given arguments.
#[macro_export]
macro_rules! exes {
    ($($arg:expr),*) => {
        $crate::cmd::exes(vec![$($arg),*])
    };
}

/// Run command with given arguments and returns result.
#[macro_export]
macro_rules! exes_with_result {
    ($($arg:expr),*) => {
        $crate::cmd::exes_with_result(vec![$($arg),*])
    };
}

/// Generate cmd with given arguments.
#[macro_export]
macro_rules! cmd{
    ($($arg:expr),*) => {
        $crate::cmd::Cmd::from_str_args(vec![$($arg),*]).map(|c|c.cmd())
    };
}

/// exec command in shell
#[cfg(feature = "shell")]
#[macro_export]
macro_rules! sh {
    ($($arg:expr),*) => {
        $crate::shell::sh(vec![$($arg),*])
    };
}

/// exec command in shell with result
#[cfg(feature = "shell")]
#[macro_export]
macro_rules! sh_with_result {
    ($($arg:expr),*) => {
        $crate::shell::sh_with_result(vec![$($arg),*])
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn cmd_macros_test() {
        let result = exes!["git", "--version"];
        assert!(result.starts_with("git version"));
        let result2 = exes_with_result!("git", "--version");
        assert!(result2.is_ok());
        assert!(result2.unwrap().starts_with("git version"));

        let cmd = cmd!["git", "--version"];
        assert!(cmd.is_ok());
        let mut cmd = cmd.unwrap();
        assert_eq!(cmd.get_program(), "git");
        assert_eq!(cmd.get_args().collect::<Vec<_>>(), vec!["--version"]);
        let output = cmd.output().unwrap();
        assert!(output.status.success());
        assert!(output.stdout.starts_with(b"git version"));
    }

    #[test]
    fn sh_test() {
        let result = sh!("echo", "hello");
        assert_eq!(result.trim_end(), "hello");

        let result = sh_with_result!("echo", "hello");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim_end(), "hello");
    }
}
