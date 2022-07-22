use std::fmt;
use std::io;
use std::{thread, time};

enum Plr {
    Person(u8), // number identifier for player
    AI,
}

impl fmt::Display for Plr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Plr::Person(id) => format!("Player {}", id),
                Plr::AI => "AI Player".to_owned(),
            }
        )
    }
}

fn main() {
    play_game(vec![3, 5, 8], vec![Plr::Person(1), Plr::AI]);
}

fn play_game(mut stacks: Vec<u8>, plrs: Vec<Plr>) {
    assert!(plrs.len() == 2, "Players Vector must have 2 elements!");

    let mut plr_iter = plrs.iter().cycle();
    let mut plr_turn = plr_iter.next().unwrap();
    let dividing_bar = "-".repeat(40); // some visual string I don't want to recreate again
    let winner = loop {
        println!("{}", &dividing_bar);
        println!("Nim Sum: {}", nim_sum(stacks.iter()));
        println!("{}", get_str_stacks(&stacks));
        println!("{}: Choose a stack and number to remove.", plr_turn);

        // We can essentially force the player to re input values
        // without panicking
        let (index, num) = match plr_turn {
            Plr::Person(_) => match get_plr_input(stacks.len()) {
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
            break plr_turn;
        }
        // Its safe to unwrap since its cyclic
        plr_turn = plr_iter.next().unwrap();
    };
    println!("{} wins!", winner);
}

fn get_str_stacks(stacks: &Vec<u8>) -> String {
    stacks
        .iter()
        .enumerate()
        .map(|(i, num_in_stack)| format!("{}: {}", i + 1, "🪙".repeat((*num_in_stack).into())))
        .collect::<Vec<String>>()
        .join("\n")
}

fn nim_sum<'a>(stacks: impl Iterator<Item = &'a u8>) -> u8 {
    stacks.fold(0, |s, acc| s ^ acc)
}

fn get_ai_input(stacks: Vec<u8>) -> (usize, u8) {
    // This brute forces trying to find a combo that adds up to zero,
    // but since we can only remove some number from one stack at a time,
    // its not too bad
    let mut lowest_sum = u8::MAX;
    let mut plr_move = (0, 0);
    for (i, &stack_size) in stacks.iter().enumerate() {
        let mut potential_stacks = stacks.clone();
        for j in 0..stack_size {
            potential_stacks[i] = j;
            let ns = nim_sum(potential_stacks.iter());
            // we try and mimize the sum, 0 is our goal but sometimes
            // we can't get their, but the next best thing can work against
            // an unskilled player
            if ns < lowest_sum {
                plr_move = (i, stack_size - j);
                lowest_sum = ns;
            }
        }
    }
    plr_move
}

fn get_plr_input(stack_len: usize) -> Option<(usize, u8)> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let mut input_iter = input.trim().split(' ');
    let index = input_iter
        .next()
        .and_then(|i| i.parse::<usize>().ok())
        .filter(|&i| i <= stack_len && i > 0)
        .and_then(|i| Some(i - 1)); // Since its 1 indexed for less confusion
    let num = input_iter
        .next()
        .and_then(|i| i.parse::<u8>().ok())
        .filter(|&i| i > 0);
    index.zip(num)
}
