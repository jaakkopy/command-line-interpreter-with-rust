use std::process::{Command, Stdio};
use std::fs::File;
use std::io::{Error, ErrorKind};

use crate::commands::*;

// take the previous child process struct and the next command in the sequence. Create a new child process struct based on these
fn advance_pipe_sequence(previous: std::process::Child, current: &SingleCommand) -> std::io::Result<std::process::Child> {
    let mut proc = Command::new(&current.name);
    proc.args(&current.args);
    if let Some(in_red) = &current.in_redirect {
        // replace stdin with file
        let file = File::open(in_red)?;
        proc.stdin(file);
    } else if let Some(out) = previous.stdout {
        proc.stdin(out);
    } 
    if let Some(out_red) = &current.out_redirect {
        // replace stdout with file
        let file = File::create(out_red)?;
        proc.stdout(file);
    } else {
        proc.stdout(Stdio::piped());
    }

    proc.spawn()
}

// pipe the commands in a single pipe group
fn execute_group(group: CommandGroup) -> std::io::Result<std::process::Child> {
    if let Some(first) = group.first() {
        let mut cmd = Command::new(&first.name);
        cmd.args(&first.args);

        if let Some(in_red) = &first.in_redirect {
            let file = File::open(in_red)?;
            cmd.stdin(file);
        }

        if let Some(out_red) = &first.out_redirect {
            let file = File::create(out_red)?;
            cmd.stdout(file);
        } else {
            cmd.stdout(Stdio::piped());
        }

        let mut proc = cmd.spawn()?;

        for cmd in group.x_to_last(1) {
            proc = advance_pipe_sequence(proc, cmd)?;
        }
        return Ok(proc);
    }
    Err(Error::new(ErrorKind::Other, "Nothing to do"))
}

pub fn execute_commands(commands: &mut Commands) {
    let mut processes: Vec<std::process::Child> = Vec::new();

    while commands.has_more() {
        if let Some(group) = commands.get_last() {
            if let Ok(proc) = execute_group(group) {
                processes.push(proc);
            }
        }
    }

    // gather the outputs
    for p in processes {
        if let Ok(out) = p.wait_with_output() {
            if let Ok(result) = std::str::from_utf8(&out.stdout) {
                print!("{}", result);
            }
        }
    }
}

