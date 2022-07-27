use players::*;
use std::{thread, time};
use ui::get_str_stacks;

mod players;
pub mod start;
mod ui;

pub fn play_game(mut stacks: Vec<u8>, plrs: Vec<Plr>) {
    let mut plr_iter = plrs.iter().enumerate().cycle();
    let (mut i, mut plr_turn) = plr_iter.next().unwrap();
    let dividing_bar = "-".repeat(40); // some visual string I don't want to recreate again
    loop {
        println!("{}", &dividing_bar);
        println!("Nim Sum: {}", nim_sum(stacks.iter()));
        println!("{}", get_str_stacks(&stacks));
        println!(
            "Player {} ({}): Choose a stack and number to remove.",
            i + 1,
            plr_turn
        );

        // We can essentially force the player to re input values
        // without panicking
        let (index, num) = match plr_turn {
            Plr::Person => match get_plr_input(stacks.len()) {
                Some(x) => x,
                None => continue,
            },
            Plr::AI => {
                let input @ (i, n) = get_ai_input(stacks.clone());
                // Give the AI player a little more humanity,
                // taking time to think and actually playing
                println!("{} {}", i + 1, n);
                thread::sleep(time::Duration::from_millis(500));
                input
            }
        };

        // we have to add one here to automatically get rid of the
        // stack when 0, we don't want a stack with exactly 0
        if let Some(remaining_stack) = stacks[index].checked_sub(num + 1) {
            stacks[index] = remaining_stack + 1;
        } else {
            stacks.remove(index);
        }

        // The last player made the last move, winning the game
        if stacks.is_empty() {
            break;
        }
        // Its safe to unwrap since its cyclic
        (i, plr_turn) = plr_iter.next().unwrap();
    }
    println!("Player {} ({}) wins!", i, plr_turn);
}
