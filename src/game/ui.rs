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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_display_358() {
        assert_eq!(
            "1: 🪙🪙🪙\n\
            2: 🪙🪙🪙🪙🪙\n\
            3: 🪙🪙🪙🪙🪙🪙🪙🪙",  
          get_str_stacks(&vec![3, 5, 8])
        )
    }
}
