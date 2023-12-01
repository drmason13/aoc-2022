pub fn concat_digits(a: u32, b: u32) -> u32 {
    debug_assert!((0..=9).contains(&a));
    debug_assert!((0..=9).contains(&b));
    a * 10 + b
}

pub fn first_digit(digits: &str) -> u32 {
    digits.chars().next().unwrap().to_digit(10).unwrap()
}

pub fn last_digit(digits: &str) -> u32 {
    digits.chars().last().unwrap().to_digit(10).unwrap()
}

/// replaces a unique subset of english words with their corresponding digit 1..=9 as follows:
/// * "one" => "o1e"
/// * "two" => "t20"
/// * "three" => "th3ee"
/// * "four" => "fo4r"
/// * "five" => "fi5e"
/// * "six" => "si6"
/// * "seven" => "s7en"
/// * "eight" => "ei8ht"
/// * "nine" => "n9ne"
///
/// This is done in order to support matching consecutive digit words with overlaps.
///
/// # Example
/// ```
/// use aoc_2023_day01::replace_number_words_with_digits;
///
/// // note there is only 1 'e' and it is needed by both "one" and "eight"
/// let tricky = "oneight";
///
/// let raw_replacement = replace_number_words_with_digits(tricky);
/// assert_eq!(raw_replacement, "o1ei8ht");
///
/// // Typical usage will see all none-digit chars removed after using this function.
/// let digits_only = raw_replacement
///     .chars()
///     .filter(|c| c.is_ascii_digit())
///     .collect::<String>();
///
/// assert_eq!(digits_only, "18");
/// ```
pub fn replace_number_words_with_digits(s: &str) -> String {
    s.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "th3ee")
        .replace("four", "fo4r")
        .replace("five", "fi5e")
        .replace("six", "si6")
        .replace("seven", "se7en")
        .replace("eight", "ei8ht")
        .replace("nine", "n9ne")
}
