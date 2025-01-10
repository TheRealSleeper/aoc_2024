#[test]
fn test1() {
    const ANSWER1: isize = 140;
    let sample1 = std::fs::read_to_string("sample1.txt").expect("Unable to read file");
    let sample1a = std::fs::read_to_string("sample1a.txt").expect("Unable to read file");
    let sample1b = std::fs::read_to_string("sample1b.txt").expect("Unable to read file"); 
    assert_eq!(super::part1(&sample1), ANSWER1);
    assert_eq!(super::part1(&sample1a), 772);
    assert_eq!(super::part1(&sample1b), 1930); 
}

#[test]
fn test2() {
    const ANSWER2: isize = 80;
    let sample2 = std::fs::read_to_string("sample2.txt").expect("Unable to read file");
    let sample2a = std::fs::read_to_string("sample2a.txt").expect("Unable to read file");
    let sample2b = std::fs::read_to_string("sample2b.txt").expect("Unable to read file"); 
    assert_eq!(super::part2(&sample2), ANSWER2);
    assert_eq!(super::part2(&sample2a), 436);
    assert_eq!(super::part2(&sample2b), 236); 
}
