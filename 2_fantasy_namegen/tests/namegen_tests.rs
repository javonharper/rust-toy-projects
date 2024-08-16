use fantasy_namegen::Namegen;

#[test]
fn test_namegen_first_name_generation() {
    let mut namegen = Namegen::new(42);

    let first_name = namegen.first_name();

    assert!(!first_name.is_empty());
    assert!(first_name.is_ascii());
    assert!(first_name.chars().all(char::is_alphabetic));
}

#[test]
fn test_namegen_last_name_generation() {
    let mut namegen = Namegen::new(42);

    let last_name = namegen.last_name();

    assert!(!last_name.is_empty());
    assert!(last_name.is_ascii());
    assert!(last_name.chars().all(char::is_alphabetic));
}

#[test]
fn test_consistent_name_with_same_seed() {
    let mut namegen1 = Namegen::new(42);
    let mut namegen2 = Namegen::new(42);

    let first_name1 = namegen1.first_name();
    let last_name1 = namegen1.last_name();

    let first_name2 = namegen2.first_name();
    let last_name2 = namegen2.last_name();

    assert_eq!(first_name1, first_name2);
    assert_eq!(last_name1, last_name2);
}

#[test]
fn test_different_name_with_different_seed() {
    let mut namegen1 = Namegen::new(42);
    let mut namegen2 = Namegen::new(43);

    let first_name1 = namegen1.first_name();
    let last_name1 = namegen1.last_name();

    let first_name2 = namegen2.first_name();
    let last_name2 = namegen2.last_name();

    assert_ne!(first_name1, first_name2);
    assert_ne!(last_name1, last_name2);
}
