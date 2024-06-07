use std::process::{Command, Output, Stdio};

pub struct ShellUtils;

/// Utility functions for executing shell commands.
impl ShellUtils {
    /// Executes a shell command and returns the output as a `Result<Output>`.
    ///
    /// # Arguments
    ///
    /// * `program` - The shell command to execute.
    /// * `args` - The arguments to pass to the shell command.
    ///
    /// # Example
    ///
    /// ```
    /// use adb_core::utils::ShellUtils;
    ///
    /// let output = ShellUtils::shell("ls", vec!["-l"]);
    /// match output {
    ///     Ok(result) => {
    ///         println!("Command executed successfully");
    ///         println!("stdout: {}", String::from_utf8_lossy(&result.stdout));
    ///         println!("stderr: {}", String::from_utf8_lossy(&result.stderr));
    ///     }
    ///     Err(error) => {
    ///         println!("Command execution failed: {}", error);
    ///     }
    /// }
    /// ```
    pub fn shell<T: AsRef<str>>(program: T, args: Vec<T>) -> std::io::Result<Output> {
        let program = program.as_ref();
        let args = args.iter().map(|x| x.as_ref()).collect::<Vec<&str>>();

        println!("shell: {} {}", &program, &args.join(" ")); // Log command

        Command::new(program)
            .args(args)
            .stdout(Stdio::piped()) // Redirect output
            .stderr(Stdio::piped())
            .output()
    }

    /// Executes a shell command and returns the output as a `String`.
    ///
    /// # Arguments
    ///
    /// * `program` - The shell command to execute.
    /// * `args` - The arguments to pass to the shell command.
    ///
    /// # Example
    ///
    /// ```
    /// use adb_core::utils::ShellUtils;
    ///
    /// let output = ShellUtils::shell_to_string("ls", vec!["-l"]);
    /// println!("Command output: {}", output);
    /// ```
    pub fn shell_to_string<T: AsRef<str>>(program: T, args: Vec<T>) -> String {
        match Self::shell(program, args) {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                stderr + &stdout // Combine stdout and stderr
            }
            Err(err) => err.to_string(),
        }
    }

    /// Spawns a shell command and returns the child process.
    ///
    /// # Arguments
    ///
    /// * `program` - The shell command to execute.
    /// * `args` - The arguments to pass to the shell command.
    ///
    /// # Example
    ///
    /// ```
    /// use adb_core::utils::ShellUtils;
    ///
    /// let child = ShellUtils::shell_spawn("ls", vec!["-l"]);
    /// // Additional operations on the child process...
    /// ```
    pub fn shell_spawn<T: AsRef<str>>(program: T, args: Vec<T>) -> std::process::Child {
        let program = program.as_ref();
        let args = args.iter().map(|x| x.as_ref()).collect::<Vec<&str>>();

        println!("shell_spawn: {} {}", &program, &args.join(" ")); // Log command

        Command::new(program)
            .args(args)
            .stdout(Stdio::piped()) // Redirect output
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to execute command")
    }
}
