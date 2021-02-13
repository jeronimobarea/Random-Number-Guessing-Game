pub fn check_answer(user_number: i32, hidden_number: i32) -> bool {
    if user_number == hidden_number {
        return true;
    }
    return false;
}
