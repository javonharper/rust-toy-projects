use crate::settings::DEBUG_ROLLS;
use rand::Rng;

// Function roll
// Takes a dnd-like dice roll string and returns the result
// Example: "2d6+3" -> 2d6 + 3 = 5 + 3 = 8
// Example: "1d20" -> 1d20 = 15
// Example: "3d4-1" -> 3d4 - 1 = 7 - 1 = 6
// Example: "1d100" -> 1d100 = 42
// Example: "1d100+10" -> 1d100 + 10 = 42 + 10 = 52
pub fn roll(dice: &str, debug_str: &str) -> i32 {
    let mut rng = rand::thread_rng();
    let mut result = 0;
    let mut parts = dice.split(|c| c == 'd' || c == '+' || c == '-');
    let num_dice = parts.next().unwrap().parse::<i32>().unwrap();
    let num_sides = parts.next().unwrap().parse::<i32>().unwrap();
    let modifier = parts.next().map_or(0, |m| m.parse::<i32>().unwrap());

    let rolls = &mut Vec::new();
    for _ in 0..num_dice {
        // result += rng.gen_range(1..die + 1);
        let roll = rng.gen_range(1..num_sides + 1);
        rolls.push(roll);
        result += roll;
    }
    result += modifier;

    if DEBUG_ROLLS {
        print!(
            "{}: {}d{} = {} = {}  +{} = {}\n",
            debug_str,
            num_dice,
            num_sides,
            rolls
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("+"),
            result - modifier,
            modifier,
            result
        );
    }
    result
}
