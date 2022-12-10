const INPUT: &str = include_str!("./input");

pub fn puzzle1() {
    println!("1_ sequence begin at: {}", detect_sequence(INPUT, 4));
}

pub fn puzzle2() {
    println!("2_ message begin at: {}", detect_sequence(INPUT, 14));
}

fn detect_sequence(signal: &str, length_signal: usize) -> usize {
    let mut current_signal_index = length_signal;

    loop {
        let current_signal = &signal[current_signal_index - length_signal..current_signal_index];
        if is_signal_with_unique_chars(current_signal) {
            return current_signal_index;
        }
        current_signal_index += 1;
    }
}

fn is_signal_with_unique_chars(signal: &str) -> bool {
    let chars = signal.chars();
    for (index, current_char) in chars.clone().into_iter().enumerate() {
        let position = signal.find(current_char).expect("char should be in signal");
        if position != index {
            return false;
        }
    }
    return true;
}
