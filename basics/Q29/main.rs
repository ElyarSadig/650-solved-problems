use std::process::Command;

fn main() {
    let output = Command::new("go").args(["list", "all"]).output();

    let output = match output {
        Ok(o) => o,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    if !output.status.success() {
        let err_text = String::from_utf8_lossy(&output.stderr);
        let msg = err_text.trim();
        if msg.is_empty() {
            println!("Error: command failed");
        } else {
            println!("Error: {}", msg);
        }
        return;
    }

    let out_text = String::from_utf8_lossy(&output.stdout);
    for line in out_text.split('\n') {
        if !line.is_empty() {
            println!("{}", line);
        }
    }
}