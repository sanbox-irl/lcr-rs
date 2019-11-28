use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub score: usize,
    pub name: String,
    pub victories: usize,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            score: 0,
            victories: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Roll {
    Dot,
    Star,
    Left,
    Right,
}

impl From<usize> for Roll {
    fn from(o: usize) -> Roll {
        match o {
            0 | 1 | 2 => Roll::Dot,
            3 => Roll::Star,
            4 => Roll::Left,
            5 => Roll::Right,
            _ => panic!("We rolled a non-dice number! We rolled {}", o),
        }
    }
}

#[allow(dead_code)]
pub fn load_players() -> Result<Vec<Player>, failure::Error> {
    let text = std::fs::read_to_string("out/out.yaml")?;

    Ok(serde_yaml::from_str(&text)?)
}

#[allow(dead_code)]
pub fn save_players(players: &Vec<Player>) -> Result<(), failure::Error> {
    let text = serde_yaml::to_string(players)?;

    Ok(std::fs::write("out/out.yaml", text)?)
}
