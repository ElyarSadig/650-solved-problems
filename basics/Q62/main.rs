use std::path::Path;

fn main() {
    let p = Path::new("/users/system1/student1/homework-1.go");
    let base = p
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    println!("{}", base);
}