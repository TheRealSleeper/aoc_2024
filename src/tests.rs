#[test]
fn test1() {
    const ANSWER1A: isize = 2028;
    const ANSWER1B: isize = 10092;
    let sample1a = std::fs::read_to_string("sample1a.txt").expect("Unable to read file");
    let sample1b = std::fs::read_to_string("sample1b.txt").expect("Unable to read file");
    assert_eq!(super::part1(&sample1a), ANSWER1A);
    assert_eq!(super::part1(&sample1b), ANSWER1B);
}

#[test]
fn test2() {
    const ANSWER2: isize = 9021;
    let sample2 = std::fs::read_to_string("sample2.txt").expect("Unable to read file");
    assert_eq!(super::part2(&sample2), ANSWER2);
}
