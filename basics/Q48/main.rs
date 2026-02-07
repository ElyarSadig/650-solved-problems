use std::process::Command;

fn main() {
    let output = Command::new("whoami").output();
    match output {
        Ok(out) => {
            if !out.status.success() {
                eprintln!("Error: failed to get current user");
                return;
            }
            let username = String::from_utf8(out.stdout);
            match username {
                Ok(s) => {
                    let u = s.trim();
                    if u.is_empty() {
                        eprintln!("Error: failed to get current user");
                        return;
                    }
                    println!("Username is: {}", u);
                }
                Err(_) => {
                    eprintln!("Error: invalid username output");
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}