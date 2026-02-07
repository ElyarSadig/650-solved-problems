use std::io;

fn bool_to_int(v: bool) -> i32 {
    if v { 1 } else { 0 }
}

fn main() {
    let _ = io::stdin();

    let t = true;
    let f = false;

    println!("Logical AND &&");
    println!("{} && {}: {}", t, t, bool_to_int(t && t));
    println!("{} && {}: {}", t, f, bool_to_int(t && f));
    println!("{} && {}: {}", f, t, bool_to_int(f && t));
    println!("{} && {}: {}", f, f, bool_to_int(f && f));

    println!("Logical OR ||");
    println!("{} || {}: {}", t, t, bool_to_int(t || t));
    println!("{} || {}: {}", t, f, bool_to_int(t || f));
    println!("{} || {}: {}", f, t, bool_to_int(f || t));
    println!("{} || {}: {}", f, f, bool_to_int(f || f));

    println!("Logical NOT !");
    println!("!{}: {}", t, bool_to_int(!t));
    println!("!{}: {}", f, bool_to_int(!f));
}