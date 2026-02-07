use std::io;
use chrono::{Datelike, Local};

fn read_int(prompt: &str) -> i32 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<i32>().expect("Not a valid number")
}

fn main() {
    let byy = read_int("Enter birth date(year):");
    let bmm = read_int("Enter birth date(month):");
    let bdd = read_int("Enter birth date(day):");

    let now = Local::now();
    let mut cyy: i32 = now.year();
    let mut cmm: i32 = now.month() as i32;
    let mut cdd: i32 = now.day() as i32;

    if cdd < bdd {
        cmm -= 1;
        cdd += 30;
    }
    let day = cdd - bdd;

    if cmm < bmm {
        cyy -= 1;
        cmm += 12;
    }
    let month = cmm - bmm;
    let year = cyy - byy;

    let days = day + month * 30 + year * 365;
    let hh = days * 24;
    let mm = hh * 60;
    let ss = mm * 60;

    println!("Old is: {}/{}/{}", year, month, day);
    print!("Hour is (hh:mm:ss) ({}:{}:{})", hh, mm, ss);
}