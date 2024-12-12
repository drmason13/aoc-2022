use std::{
    collections::HashMap,
    sync::{mpsc, Arc},
};

use shared::{read_input, receive_answers, run_part_threaded};

type Answer = u32;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Clone, Debug)]
struct Game {
    id: u32,
    selections: Vec<HashMap<Color, u32>>,
}

impl Game {
    pub fn new(id: u32, selections: Vec<HashMap<Color, u32>>) -> Self {
        Game { id, selections }
    }
}

mod parsers {
    use std::collections::HashMap;

    use parsely::{switch, token, uint, ws, Lex, Parse, ParseResult};

    use crate::{Color, Game};

    /// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    pub fn game(input: &str) -> ParseResult<Game> {
        token("Game ")
            .skip_then(uint::<u32>())
            .then_skip(':')
            .then(selection_set().many(1..50).delimiter(';'.pad()))
            .map(|(id, selections)| Game::new(id, selections))
            .parse(input)
    }

    /// 3 blue, 4 red
    fn selection_set() -> impl Parse<Output = HashMap<Color, u32>> {
        selection()
            .many(1..=3)
            .delimiter(','.pad())
            .map(|color_counts| color_counts.into_iter().collect::<HashMap<Color, u32>>())
    }

    /// 3 blue
    fn selection() -> impl Parse<Output = (Color, u32)> {
        uint::<u32>().then_skip(ws()).then(color()).swap()
    }

    /// blue
    fn color() -> impl Parse<Output = Color> {
        switch([
            ("red", Color::Red),
            ("blue", Color::Blue),
            ("green", Color::Green),
        ])
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = read_input(2023, 2);
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn part1(input: &str) -> Answer {
    use Color::*;

    let games = input
        .lines()
        .map(|line| {
            let (game, _) = parsers::game(line).expect("parse input");
            game
        })
        .collect::<Vec<Game>>();

    let limits: HashMap<Color, u32> = {
        let mut hm = HashMap::new();
        hm.insert(Red, 12);
        hm.insert(Green, 13);
        hm.insert(Blue, 14);
        hm
    };

    games
        .iter()
        .filter(|g| {
            g.selections.iter().all(|colors| {
                [Red, Green, Blue].iter().all(|color| {
                    let Some(observed_count) = colors.get(color) else {
                        // none for a color is always possible
                        return true;
                    };
                    limits.get(color).unwrap() >= observed_count
                })
            })
        })
        .map(|g| g.id)
        .sum()
}

fn limit_of_game(game: &Game) -> HashMap<Color, u32> {
    use Color::*;

    let mut limit = {
        let mut hm = HashMap::new();
        hm.insert(Red, 0);
        hm.insert(Green, 0);
        hm.insert(Blue, 0);
        hm
    };

    for selection in game.selections.iter() {
        for color in [Red, Green, Blue] {
            match selection.get(&color) {
                Some(count) if count > limit.get(&color).unwrap() => {
                    *limit.get_mut(&color).unwrap() = *count
                }
                _ => {}
            }
        }
    }

    limit
}

fn power_of_game(game_limit: HashMap<Color, u32>) -> u32 {
    use Color::*;

    game_limit.get(&Red).unwrap() * game_limit.get(&Green).unwrap() * game_limit.get(&Blue).unwrap()
}

fn part2(input: &str) -> Answer {
    let games = input
        .lines()
        .map(|line| {
            let (game, _) = parsers::game(line).expect("parse input");
            game
        })
        .collect::<Vec<Game>>();

    games.iter().map(limit_of_game).map(power_of_game).sum()
}

#[cfg(test)]
mod test {
    use shared::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#};

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2286);
    }
}
