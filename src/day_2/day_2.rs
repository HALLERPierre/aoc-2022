pub fn puzzle1() {
    let all_rounds_results = get_all_rounds_results_1();
    let total_points: i32 = all_rounds_results.iter().sum();

    println!("1_: Total points: {:?}", total_points);
}

pub fn puzzle2() {
    let all_rounds_results = get_all_rounds_results_2();
    let total_points: i32 = all_rounds_results.iter().sum();

    println!("2_: Total points: {:?}", total_points);
}

fn get_all_rounds_results_1() -> Vec<i32> {
    return include_str!("./input")
        .split("\n")
        .filter(|choices_str| choices_str.len() > 0)
        .map(|choices| {
            let choices_vec: Vec<&str> = choices.split_ascii_whitespace().collect();

            let opponent_choice = *choices_vec.get(0).expect("Should have opponent choice");
            let my_choice = *choices_vec.get(1).expect("Should have opponent choice");

            let opponent_points = get_opponent_points(opponent_choice);
            let my_points = get_my_points(my_choice);

            let round_result = get_round_point(opponent_points, my_points);

            let points = round_result + my_points;

            return points;
        })
        .collect::<Vec<i32>>();
}

fn get_all_rounds_results_2() -> Vec<i32> {
    return include_str!("./input")
        .split("\n")
        .filter(|choices_str| choices_str.len() > 0)
        .map(|choices| {
            let choices_vec: Vec<&str> = choices.split_ascii_whitespace().collect();

            let opponent_choice = *choices_vec.get(0).expect("Should have opponent choice");
            let expected_outcome = *choices_vec.get(1).expect("Should have opponent choice");

            let opponent_points = get_opponent_points(opponent_choice);
            let my_points = get_my_choice(opponent_points, expected_outcome);

            let round_result = get_round_point(opponent_points, my_points);

            let points = round_result + my_points;

            return points;
        })
        .collect::<Vec<i32>>();
}

fn get_opponent_points(opponent: &str) -> i32 {
    match opponent {
        "A" => 1, // rock
        "B" => 2, // paper
        "C" => 3, // scissors
        _ => panic!("Unknown opponent choice {:?}", opponent),
    }
}

fn get_my_points(choice: &str) -> i32 {
    match choice {
        "X" => 1, // rock
        "Y" => 2, // paper
        "Z" => 3, // scissors
        _ => panic!("Unknown opponent choice"),
    }
}

fn get_my_choice(opponent_choice: i32, expected_outcome: &str) -> i32 {
    match opponent_choice {
        1 => {
            return match expected_outcome {
                "X" => 3,
                "Y" => 1,
                "Z" => 2,
                _ => panic!("unknown outcome"),
            }
        }
        2 => {
            return match expected_outcome {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => panic!("unknown outcome"),
            }
        }
        3 => {
            return match expected_outcome {
                "X" => 2,
                "Y" => 3,
                "Z" => 1,
                _ => panic!("unknown outcome"),
            }
        }
        _ => panic!("unknown opponent choice"),
    }
}

fn get_round_point(opponent: i32, choice: i32) -> i32 {
    if opponent == choice {
        return 3;
    }
    if opponent == 1 && choice == 3 || opponent == 2 && choice == 1 || opponent == 3 && choice == 2
    {
        return 0;
    }
    return 6;
}
