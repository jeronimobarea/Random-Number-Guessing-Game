use std::io;

pub fn menu_input() -> i32 {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(__) => {}
        Err(error) => println!("error: {}", error),
    }
    let parse_input = input.trim().parse::<i32>().unwrap();
    return parse_input;
}

pub fn guess_number_input() -> i32 {
    println!("Say a number between 0 and 10");
    let user_number = menu_input();
    return user_number;
}
