#[test]
fn test1() {
    const ANSWER1: isize = 36;
    let sample1 = std::fs::read_to_string("sample1.txt").expect("Unable to read file");
    assert_eq!(super::part1(&sample1), ANSWER1);
}

#[test]
fn test2() {
    const ANSWER2: isize = 81;
    let sample2 = std::fs::read_to_string("sample2.txt").expect("Unable to read file");
    assert_eq!(super::part2(&sample2), ANSWER2);
}
