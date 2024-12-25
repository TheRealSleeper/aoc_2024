#[test]
fn test1() {
    const ANSWER1: isize = 41;
    let sample1 = std::fs::read_to_string("sample1.txt").expect("Unable to read file");
    assert_eq!(super::part1(&sample1), ANSWER1);
}

#[test]
fn test2() {
    const ANSWER2: isize = 6;
    let sample2 = std::fs::read_to_string("sample2.txt").expect("Unable to read file");
    assert_eq!(super::part2(&sample2), ANSWER2);
}

#[test]
fn test2_additional() {
    let additional = std::fs::read_to_string("part2_addition.txt").unwrap();
    assert_eq!(super::part2(&additional), 1);
}
