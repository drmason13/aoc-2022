pub struct Monkey {
    /// current items
    pub items: Vec<u64>,
    /// apply this function to each item to get a new item
    pub operation: Box<dyn Fn(u64) -> u64>,
    pub test: u64,
    pub if_true: usize,
    pub if_false: usize,
    pub inspection_count: u64,
}

pub fn parse_monkey(input: &str) -> Monkey {
    let lines: Vec<&str> = input.lines().collect();
    let items = lines[1].trim();
    let items: String = items.chars().skip_while(|ch| !ch.is_numeric()).collect();
    let items: Vec<u64> = items
        .split(", ")
        .map(|item| item.parse().expect("valid input"))
        .collect();

    let operation = lines[2].trim();
    let split_at_index = operation
        .find(|ch| matches!(ch, '+' | '*'))
        .expect("valid input");
    let (_, operation) = operation.split_at(split_at_index);
    let operation: Box<dyn Fn(u64) -> u64> = if operation.starts_with('+') {
        let digits: String = operation.chars().skip(2).collect();
        match digits.as_str() {
            "old" => Box::new(|value: u64| value + value),
            val => {
                let val = val.parse::<u64>().expect("valid input");
                Box::new(move |value: u64| value + val)
            }
        }
    } else {
        let digits: String = operation.chars().skip(2).collect();
        match digits.as_str() {
            "old" => Box::new(|value: u64| value * value),
            val => {
                let val = val.parse::<u64>().expect("valid input");
                Box::new(move |value: u64| value * val)
            }
        }
    };

    let test = lines[3].trim();
    let digits: String = test.chars().skip_while(|ch| !ch.is_numeric()).collect();
    let test = digits.parse::<u64>().expect("valid input");

    let if_true = lines[4].trim();
    let digits: String = if_true.chars().skip_while(|ch| !ch.is_numeric()).collect();
    let if_true = digits.parse::<usize>().expect("valid input");

    let if_false = lines[5].trim();
    let digits: String = if_false.chars().skip_while(|ch| !ch.is_numeric()).collect();
    let if_false = digits.parse::<usize>().expect("valid input");

    Monkey {
        items,
        test,
        operation,
        if_true,
        if_false,
        inspection_count: 0,
    }
}
