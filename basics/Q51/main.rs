use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

fn main() {
    let code = "import sys\nprint(sys.copyright)\n";

    let mut path = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    path.push(format!("script{}.py", nanos));

    let res: Result<(), Box<dyn std::error::Error>> = (|| {
        let mut file = File::create(&path)?;
        file.write_all(code.as_bytes())?;
        file.sync_all()?;

        let output = Command::new("python").arg(&path).output()?;

        if !output.status.success() {
            eprintln!(
                "Error executing Python script: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            return Ok(());
        }

        print!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    })();

    let _ = fs::remove_file(&path);

    if let Err(e) = res {
        eprintln!("Error: {}", e);
    }
}