use std::process::Command;

fn main() {
    let cmd = Command::new("calc.exe").spawn();

    match cmd {
        Ok(mut child) => match child.wait() {
            Ok(status) => {
                if status.success() {
                    println!("Calculator executed successfully");
                } else {
                    println!("Calculator exited with status: {}", status);
                }
            }
            Err(e) => {
                eprintln!("Error waiting for process: {}", e);
            }
        },
        Err(e) => {
            eprintln!("Error starting calc.exe: {}", e);
        }
    }
}
