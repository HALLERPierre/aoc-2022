type ElveRange = (i32, i32);
type ElveGroupRange = (ElveRange, ElveRange);

pub fn puzzle1() {
    let elves_ranges = get_elves_ranges();
    let overlapings = elves_ranges
        .filter(|ranges| is_overlaping(*ranges))
        .collect::<Vec<ElveGroupRange>>();

    println!("1_ There is {:?} overlapings", overlapings.len());
}

pub fn puzzle2() {
    let elves_ranges = get_elves_ranges();
    let overlapings = elves_ranges
        .filter(|ranges| is_partial_overlaping(*ranges))
        .collect::<Vec<ElveGroupRange>>();

    println!("2_ There is {:?} partial overlapings", overlapings.len());
}

fn is_partial_overlaping(ranges: ElveGroupRange) -> bool {
    let (elve_1_min, elve_1_max) = ranges.0;
    let (elve_2_min, elve_2_max) = ranges.1;

    let is_elve_1_overlaping = elve_1_min >= elve_2_min && elve_1_min <= elve_2_max
        || elve_1_max >= elve_2_min && elve_1_max <= elve_2_max;
    let is_elve_2_overlaping = elve_2_min >= elve_1_min && elve_2_min <= elve_1_max
        || elve_2_max >= elve_1_min && elve_2_max <= elve_1_max;

    return is_elve_1_overlaping || is_elve_2_overlaping;
}

fn is_overlaping(ranges: ElveGroupRange) -> bool {
    let (elve_1_min, elve_1_max) = ranges.0;
    let (elve_2_min, elve_2_max) = ranges.1;
    return elve_1_min >= elve_2_min && elve_1_max <= elve_2_max
        || elve_2_min >= elve_1_min && elve_2_max <= elve_1_max;
}

fn parse_range(range: &str) -> (i32, i32) {
    let parsed_range: Vec<i32> = range .split('-') .map(|number| number.parse::<i32>().expect("should be integer")) .collect();
    let start_range = parsed_range.get(0).expect("should have start range");
    let end_range = parsed_range.get(1).expect("should have end range");

    return (*start_range, *end_range);
}

fn get_elves_ranges() -> impl Iterator<Item = ElveGroupRange> {
    let lines = include_str!("./input")
        .split("\n")
        .filter(|line| line.len() > 0);
    let elves_ranges = lines.map(|line| {
        let ranges: Vec<(i32, i32)> = line.split(',').map(parse_range).collect();
        let first_elve = ranges.get(0).expect("should have first elve");
        let second_elve = ranges.get(1).expect("should have second elve");
        return (*first_elve, *second_elve);
    });

    return elves_ranges;
}
