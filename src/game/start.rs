use super::players::Plr;
use std::io;

fn get_yes_ans() -> Option<bool> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok()?;
    match buffer.trim().to_lowercase().as_str() {
        "y" => Some(true),
        "n" => Some(false),
        _ => None,
    }
}

pub fn get_starting_stacks() -> Option<Vec<u8>> {
    println!("Write out a list stacks, with a comma seperating each (i.e. 3, 5, 8)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    // Get rid of all whitespace and other bad chars
    let refinded_input = input.trim().replace(" ", "");
    refinded_input
        .split(',')
        .map(|n| n.parse::<u8>().ok())
        .collect()
}

pub fn get_starting_players() -> Vec<Plr> {
    let mut plr_vec: Vec<Plr> = vec![Plr::Person, Plr::Person];
    println!("Singleplayer with AI? [Y/n]");
    let is_single = get_yes_ans().expect("Invalid response");
    if is_single {
        plr_vec[0] = Plr::AI; // by default ai is second
        println!("Do you want Player 1? [Y/n]");
        if get_yes_ans().expect("Invalid response") {
            plr_vec.reverse()
        }
    }
    plr_vec
}
