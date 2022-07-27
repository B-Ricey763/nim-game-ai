use std::io;

pub enum Plr {
    Person,
    AI,
}

pub fn nim_sum<'a>(stacks: impl Iterator<Item = &'a u8>) -> u8 {
    stacks.fold(0, |s, acc| s ^ acc)
}

pub fn get_ai_input(stacks: Vec<u8>) -> (usize, u8) {
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

pub fn get_plr_input(stack_len: usize) -> Option<(usize, u8)> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nim_sum_358() {
        assert_eq!(14, nim_sum(vec![3, 5, 8].iter()))
    }

    #[test]
    fn ai_first_move_358() {
        assert_eq!((2, 2), get_ai_input(vec![3, 5, 8]))
    }
}
