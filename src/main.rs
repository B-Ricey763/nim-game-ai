use game::start;

mod game;

fn main() {
    loop {
        let stacks = start::get_starting_stacks().expect("Invalid Response");
        let plrs = start::get_starting_players();
        game::play_game(stacks, plrs);
    }
}
