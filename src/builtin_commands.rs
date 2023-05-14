use std::env::set_current_dir;
use std::path::Path;
use std::io;

pub enum BUILTINS {
    NONE,
    CD,
    EXIT
}

fn change_working_dir(path_str: &str) -> io::Result<()> {
    let path = Path::new(path_str);
    set_current_dir(path)?;
    Ok(())
}

fn split_by_whitespace(input: &str) -> Vec<&str> {
    input.split_ascii_whitespace().collect()
}

// Check if the given input string is a built in command. Return true if the interpreter loop should be broken.
pub fn check_builtin(input: &str) -> io::Result<BUILTINS> {
    if input.eq("exit") {
        return Ok(BUILTINS::EXIT);
    }
    let parts = split_by_whitespace(input);
    if parts.len() >= 2 && parts[0].eq("cd") {
        if let Ok(_) = change_working_dir(parts[1]) {
            return Ok(BUILTINS::CD);
        }
    }
    Ok(BUILTINS::NONE)
}