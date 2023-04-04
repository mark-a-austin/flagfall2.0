
use std::str::FromStr;

use log::error;
use serde::Serialize;

use crate::gametype::GameType;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub enum ChallengeColour {
    #[serde(rename = "white")]
    White,
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "random")]
    Random,
}

impl FromStr for ChallengeColour {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "white" => Ok(Self::White),
            "black" => Ok(Self::Black),
            "random" => Ok(Self::Random),
            _ => Err(()),
        }
    }
}

#[derive(Serialize)]
pub struct ChallengeSchema {
    pub rated: bool,
    #[serde(rename = "clock.limit")]
    pub clock_limit: u32,
    #[serde(rename = "clock.increment")]
    pub clock_increment: u32,
    pub color: ChallengeColour,
    pub variant: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fen: Option<String>,
    #[serde(rename = "keepAliveStream")]
    pub keep_alive_stream: bool,
}

pub fn get_challenge_schema<T: GameType>() -> (String, ChallengeSchema) {
    let mut user_input = String::new();
    if T::IS_VS_HUMAN {
        println!("enter the username to challenge:");
    } else {
        println!("do you want to challenge Viridithas or Maia? [V|M]");
    };
    std::io::stdin().read_line(&mut user_input).unwrap();
    let username = user_input.trim().to_lowercase();
    let time_control = if T::IS_VS_HUMAN {
        println!("enter the time control in the form of 'min+inc' (e.g. '5+2' for 5 minutes + 2 seconds increment):");
        user_input.clear();
        std::io::stdin().read_line(&mut user_input).unwrap();
        user_input.trim().to_lowercase()
    } else {
        "15+10".to_string()
    };
    let rated = if T::IS_VS_HUMAN {
        println!("should the game be rated? [Y|N]");
        user_input.clear();
        std::io::stdin().read_line(&mut user_input).unwrap();
        match user_input.trim().to_lowercase().as_str() {
            "y" => true,
            "n" => false,
            _ => panic!("invalid input"),
        }
    } else {
        false
    };
    println!("enter the challenge colour [white|black|random]:");
    user_input.clear();
    std::io::stdin().read_line(&mut user_input).unwrap();
    let colour = user_input.trim().to_lowercase();
    let Ok(colour) = colour.parse::<ChallengeColour>() else {
        error!("invalid colour: {colour}");
        panic!("invalid colour");
    };
    let clock_limit = time_control
        .split('+')
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap()
        * 60;
    let clock_increment = time_control
        .split('+')
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let keep_alive_stream = true;
    (
        username,
        ChallengeSchema {
            rated,
            clock_limit,
            clock_increment,
            color: colour,
            variant: "standard".to_string(),
            fen: None,
            keep_alive_stream,
        },
    )
}