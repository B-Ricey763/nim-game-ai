use super::players::Plr;
use std::fmt;

impl fmt::Display for Plr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Plr::Person => "Human",
                Plr::AI => "AI",
            }
        )
    }
}

pub fn get_str_stacks(stacks: &Vec<u8>) -> String {
    stacks
        .iter()
        .enumerate()
        .map(|(i, num_in_stack)| format!("{}: {}", i + 1, "🪙".repeat((*num_in_stack).into())))
        .collect::<Vec<String>>()
        .join("\n")
}
