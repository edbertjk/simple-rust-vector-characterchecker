struct User {
    name: String,
    age: i32,
}

fn main() {
    let string1: &str = "Edbert julian karsten";
    let mut vector: Vec<&User> = Vec::new();
    let userData: User = User {
        name: String::from("Edbert"),
        age: 21,
    };
    for i in 0..string1.len() {
        let bytes_char: char = string1.as_bytes()[i] as char;
        println!("{}", i);
        if bytes_char == 'n' {
            vector.push(&userData);
        }
    }
    for user in vector {
        println!("here: {}", user.name);
    }
}
