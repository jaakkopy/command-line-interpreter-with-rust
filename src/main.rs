mod commands;
mod command_execution;
mod input;
mod history;
mod builtin_commands;
mod input_state_handler;
mod dirextory_prefix_tree;
use std::os::fd::RawFd;
use crate::input::Input;
use builtin_commands::BUILTINS;
use termios::*;

static STDIN_FILENO: RawFd  = 0;

fn set_termios_settings() -> std::io::Result<termios::Termios> {
    let old_terminal_settings = termios::Termios::from_fd(STDIN_FILENO)?;
    let mut new_settings = old_terminal_settings;
    // disable canonical mode. Also disable echo for better control of displayed input 
    new_settings.c_lflag &= !(ICANON | ECHO);
    // set new settings to stdin
    termios::tcsetattr(STDIN_FILENO, TCSANOW, &new_settings)?;
    Ok(old_terminal_settings)
}

pub fn split_by_whitespace(input_buf: &str) -> Vec<&str> {
    input_buf.split_whitespace().collect()
}

fn main_loop() -> std::io::Result<()> {
    let mut inp = Input::make()?;
    loop {
        if let Ok(input_str) = inp.read_input() {
            if let Ok(builtin) = builtin_commands::check_builtin(&input_str) {
                match builtin {
                    BUILTINS::CD => {inp.update_prefix_tree()?; continue;},
                    BUILTINS::EXIT => break,
                    _ => ()
                } 
            }
            
            command_execution::execute_commands(
                &mut commands::make_commands(
                    split_by_whitespace(&input_str)
                )
            );
        }
    }
    Ok(())
}

fn main() {
    if let Ok(old_terminal_settings) = set_termios_settings() {
        if let Err(_) = main_loop() {
            eprintln!("Failed to initialize");
        }
        // Restore old terminal settings.
        match termios::tcsetattr(STDIN_FILENO, TCSANOW, &old_terminal_settings) {
            Ok(_) => (),
            Err(_) => eprintln!("failed to restore terminal settings")
        }
    } else {
        eprintln!("Couldn't set up terminal settings");
    }
}
