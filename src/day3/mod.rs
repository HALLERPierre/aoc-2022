fn get_rucksacks<'a>() -> impl Iterator<Item = &'a str> {
    include_str!("./input")
        .split("\n")
        .filter(|rucksack| rucksack.len() > 0)
}

pub fn puzzle1() {
    let rucksacks = get_rucksacks();
    let errors_sum_priority: i32 = rucksacks
        .map(|rucksack| get_compartments(rucksack))
        .map(|compartments| find_error(compartments))
        .map(|error| get_priority(error))
        .sum();

    println!("1_ priority error sum: {:?}", errors_sum_priority);
}

pub fn puzzle2() {
    let rucksacks_by_team = get_rucksacks_by_team(get_rucksacks());
    let badge_sum: i32 = rucksacks_by_team
        .iter()
        .map(|team_rucksacks| get_badge_in_rucksacks(team_rucksacks.clone()))
        .map(|badge| get_priority(badge))
        .sum();

    println!("2_ badge sum: {:?}", badge_sum);
}

fn get_rucksacks_by_team<'a>(rucksacks: impl Iterator<Item = &'a str>) -> Vec<Vec<&'a str>> {
    let rucksacks_vec: Vec<&str> = rucksacks.collect();
    let rucksacks_size = rucksacks_vec.len();

    let mut rucksack_index = 1;
    let mut rucksacks_by_team: Vec<Vec<&str>> = Vec::new();

    while rucksack_index + 1 < rucksacks_size {
        let team_rucksacks = &rucksacks_vec[rucksack_index - 1..=rucksack_index + 1];
        rucksacks_by_team.push(team_rucksacks.to_vec());

        rucksack_index += 3;
    }

    rucksacks_by_team
}

fn get_badge_in_rucksacks(rucksacks: Vec<&str>) -> char {
    let rucksack = *rucksacks.get(0).expect("should have at least one item");
    let mut chars = rucksack.chars();
    chars
        .find(|char| rucksacks.iter().all(|rucksack| rucksack.contains(*char)))
        .expect("should have a common item in rucksack")
}

fn get_compartments(rucksack: &str) -> (&str, &str) {
    if rucksack.len() % 2 != 0 {
        panic!("rucksack len should be pair")
    }
    let compartment_size = rucksack.len() / 2;

    let compartment1 = &rucksack[..compartment_size];
    let compartment2 = &rucksack[compartment_size..];

    (compartment1, compartment2)
}

fn find_error(compartments: (&str, &str)) -> char {
    let (compartment1, compartment2) = compartments;

    let error = compartment1
        .chars()
        .find(|char1| compartment2.contains(*char1))
        .expect("No error found");

    error
}

fn get_priority(error: char) -> i32 {
    let ascii = error as i32;
    let ascii_value_a = 'a' as i32;
    let ascii_value_a_uppercase = 'A' as i32;
    if ascii >= 97 {
        return ascii - ascii_value_a + 1;
    }
    ascii - ascii_value_a_uppercase + 27
}
