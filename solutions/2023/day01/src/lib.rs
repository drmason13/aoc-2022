use parsely::Parse;

pub fn concat_digits(a: u32, b: u32) -> u32 {
    debug_assert!((0..=9).contains(&a));
    debug_assert!((0..=9).contains(&b));
    a * 10 + b
}

pub fn digit_parser() -> impl Parse<Output = u32> {
    parsely::combinator::crawl(parsely::switch([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]))
}

pub fn digit_word_parser() -> impl Parse<Output = u32> {
    parsely::combinator::crawl(parsely::switch([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]))
}
