use std::env;

fn main() {
    let exe_path = match env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let abs_path = match exe_path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    println!("Current File Name {}", abs_path.display());
}