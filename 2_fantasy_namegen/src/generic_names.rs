use rand::{rngs::StdRng, Rng};

pub fn first_name(rng: &mut StdRng) -> String {
    let prefixes = vec![
        "Al", "Ba", "Cor", "Dro", "El", "Fa", "Gar", "Hor", "Ir", "Ja",
    ];

    let middles = vec![
        "ban", "dor", "el", "gar", "hor", "ith", "jon", "karn", "lor", "mir",
    ];

    let suffixes = vec![
        "dor", "eth", "mir", "n", "ron", "th", "ur", "vor", "wen", "zir",
    ];

    let prefix = prefixes[rng.gen_range(0..prefixes.len())];
    let middle = middles[rng.gen_range(0..middles.len())];
    let suffix = suffixes[rng.gen_range(0..suffixes.len())];

    format!("{}{}{}", prefix, middle, suffix)
}

pub fn last_name(rng: &mut StdRng) -> String {
    let prefixes = vec![
        "Ash", "Bright", "Cold", "Dark", "Fair", "Grim", "Iron", "Storm", "Thorn", "Wind",
    ];

    let suffixes = vec![
        "wood", "stone", "blade", "brook", "field", "forge", "horn", "shield", "cliff", "fall",
    ];

    let prefix = prefixes[rng.gen_range(0..prefixes.len())];
    let suffix = suffixes[rng.gen_range(0..suffixes.len())];

    format!("{}{}", prefix, suffix)
}
