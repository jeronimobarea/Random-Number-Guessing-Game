use rand::Rng;

#[path = "../settings/mod.rs"]
mod settings;

pub fn generate_number(level: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(0, settings::dificulty::select_dificulty(level));
    return number;
}
