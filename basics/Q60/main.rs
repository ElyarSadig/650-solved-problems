use std::mem::size_of;

struct User {
    id: i32,
    first_name: String,
    last_name: String,
}

fn main() {
    let user = User {
        id: 1,
        first_name: "Elyar".to_string(),
        last_name: "Sadig".to_string(),
    };
    let a1: i32 = 10;
    let a2: i64 = 10;
    let s: String = "Hello World".to_string();
    let slice_of_bytes: Vec<u8> = vec![0u8; 1024];
    let r: char = 'A';
    let array: [i32; 10] = [0; 10];

    println!("Memory Size of a1 is {} bytes", size_of::<i32>());
    println!("Memory Size of a2 is {} bytes", size_of::<i64>());
    println!("Memory Size of s is {} bytes", size_of::<String>());
    println!("Memory Size of user is {} bytes", size_of::<User>());
    println!(
        "Memory Size of sliceOfBytes is {} bytes",
        size_of::<Vec<u8>>()
    );
    println!("Memory Size of r is {} bytes", size_of::<char>());
    println!("Memory Size of array is {} bytes", size_of::<[i32; 10]>());

    let _ = (user, a1, a2, s, slice_of_bytes, r, array);
}