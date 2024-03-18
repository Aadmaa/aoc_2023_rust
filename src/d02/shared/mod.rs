// game id
// subgames -

#[derive(Debug)]
pub struct Subgame {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl Subgame {
    pub fn new(r: u32, g: u32, b: u32) -> Subgame {
        Subgame { r, g, b }
    }
}

impl PartialEq for Subgame {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.b == other.b && self.g == other.g
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub subgames: Vec<Subgame>,
}

impl Game {
    pub fn new(id: u32, subgames: Vec<Subgame>) -> Game {
        Game {
            id,
            subgames,
        }
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        if self.id != other.id {
            return false;
        }
        let len = self.subgames.len();
        if len != other.subgames.len() {
            return false;
        }

        for (i, value) in self.subgames.iter().enumerate() {
            let other_sub = &other.subgames[i];
            if other_sub != value {
                return false;
            }
        }
        
        true
    }
}

fn try_string_to_subgame(try_string: &str) -> Subgame {
    // try_string example: "3 blue, 4 red"
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    let color_entries = try_string.split(",");

    for entry in color_entries {
        let mut vals = entry.trim().split_whitespace();
        let color_count: u32 = vals.next().unwrap().trim().parse().unwrap();
        let color_name = vals.next().unwrap().trim();

        match color_name {
            "blue" => b += color_count,
            "green" => g += color_count,
            "red" => r += color_count,
            _ => panic!("Unknown color: {}", color_name)
        }
    }

    Subgame::new(r, g, b)
}

/// Get the Game from a line of text formatted like this:
/// "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
pub fn line_to_game(line: &str) -> Option<Game> {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let mut vals = line.split(":");
    let game_str = vals.next()?.trim(); // Game 1
    let tries_str = vals.next()?.trim(); // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

    let game_split = game_str.split(" ");
    let id: u32 = game_split.last()?.parse().unwrap();

    let try_strings = tries_str.split(";");

    let tries: Vec<_> = try_strings
        .map(|try_string| try_string_to_subgame(&try_string))
        .collect();

    let rez = Game::new(id, tries);

    // println!("{:?}", rez);
    Some(rez)
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_line_to_game() {
        let val = line_to_game("Game 12: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
            .expect("Expected Some Game, got None");

        let expect = Game {
            id: 12,
            subgames: vec![
                Subgame::new(4, 0, 3),
                Subgame::new(1, 2, 6),
                Subgame::new(0, 2, 0)
            ]
        };

        assert_eq!(val, expect);
        
    }
}