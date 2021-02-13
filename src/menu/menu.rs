use std::process;

use crate::menu;

#[path = "../utils/mod.rs"]
mod utils;

pub fn menu() {
    menu::questions::menu_questions();

    let selection: i32 = menu::input::menu_input();

    match selection {
        1 => {
            let hidden_number: i32 = utils::number_generator::generate_number(10);
            let user_number: i32 = menu::input::guess_number_input();
            let result: bool = utils::check_answer::check_answer(user_number, hidden_number);

            if result {
                println!("You win! \n");
            }
            println!("You lose the number was -> {}\n", hidden_number);
        }
        2 => println!("Input is equal to b"),
        _ => process::exit(1),
    }
}
