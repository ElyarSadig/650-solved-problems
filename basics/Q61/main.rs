use std::io;

fn main() {
    let host_name = match std::env::var("HOSTNAME") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => match std::env::var("COMPUTERNAME") {
            Ok(v) if !v.trim().is_empty() => v,
            _ => match std::env::current_dir() {
                Ok(_) => String::from("Unknown"),
                Err(e) => {
                    println!("Error is {}", e);
                    return;
                }
            },
        },
    };

    let _ = io::stdout().flush();

    println!("HostName: {}", host_name);
}