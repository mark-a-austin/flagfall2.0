
use futures_util::StreamExt;
use log::{debug, error, info, warn};

use crate::{user::{ChallengeSchema, self}, gametype::VsHuman};

use super::{LICHESS_HOST, LICHESS_TOKEN};

use reqwest::Client;
use reqwest::Response;
use shakmaty::Move;

use std::io::Write;
use shakmaty::{san::San, uci::Uci, Position};

use shakmaty::Chess;

pub async fn send_move_to_game(client: &Client, game_id: &str, mv: &Move) -> Response {
    client
        .post(format!(
            "{LICHESS_HOST}/api/board/game/{game_id}/move/{}",
            mv.to_uci(shakmaty::CastlingMode::Standard)
        ))
        .bearer_auth(LICHESS_TOKEN)
        .send()
        .await
        .unwrap()
}

pub async fn send_challenge(client: &Client, username: &str, schema: &ChallengeSchema) -> Response {
    info!("sending challenge to {username}");
    let body = serde_urlencoded::to_string(schema).unwrap();
    debug!("challenge schema: {body}");
    client
        .post(format!("{LICHESS_HOST}/api/challenge/{username}"))
        .bearer_auth(LICHESS_TOKEN)
        .body(body)
        .send()
        .await
        .unwrap()
}

pub async fn create_new_game(client: &Client) -> Option<String> {
    let (username, schema) = user::get_challenge_schema::<VsHuman>();
    let response = send_challenge(client, &username, &schema).await;
    debug!("challenge sent, response: {response:?}");
    let mut stream = response.bytes_stream();
    let mut game_id = None;
    while let Some(Ok(bytes)) = stream.next().await {
        let s = String::from_utf8_lossy(&bytes);
        debug!("received {s}");
        if s.contains("gameId") {
            let v: serde_json::Value = serde_json::from_str(&s).unwrap();
            game_id = v["gameId"].as_str().map(ToString::to_string);
            break;
        }
    }
    game_id.map_or_else(
        || {
            warn!("no game ID received, exiting.");
            None
        },
        |game_id| {
            info!("game ID: {game_id}");
            Some(game_id)
        },
    )
}

pub async fn join_game(
    client: &Client,
    n_current_games: usize,
    default_game: &serde_json::Value,
) -> Option<String> {
    println!("create a new game or join an existing one? [C|J] (you have {n_current_games} ongoing game{})", if n_current_games == 1 { "" } else { "s" });
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    let user_input = user_input.trim().to_lowercase();
    if user_input == "c" {
        loop {
            let game_id = create_new_game(client).await;
            if let Some(game_id) = game_id {
                return Some(game_id);
            }
            println!("error creating game, try again? [Y|N]");
            let mut user_input = String::new();
            std::io::stdin().read_line(&mut user_input).unwrap();
            let user_input = user_input.trim().to_lowercase();
            if user_input != "y" {
                return None;
            }
        }
    } else if user_input == "j" {
        if n_current_games == 0 {
            error!("no ongoing games, exiting.");
            return None;
        } else if n_current_games > 1 {
            warn!("more than one current game, selecting the first one.");
        }
        // get the game id
        let Some(game_id) = default_game.get("gameId").map(|game_id| game_id.as_str().unwrap()) else {
            error!("no 'gameId' field in json string ({json}), exiting.", json = default_game);
            return None;
        };
        return Some(game_id.to_string());
    } else {
        error!("invalid input, exiting.");
        return None;
    };
}


#[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
pub async fn main() {
    info!("creating reqwest client");
    let client = Client::builder()
        .user_agent("flagfall-lichess-api")
        .build()
        .unwrap();

    info!("getting ongoing games");
    let ongoing_games = client
        .get(format!("{LICHESS_HOST}/api/account/playing"))
        .bearer_auth(LICHESS_TOKEN)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // parse the json string
    let v: serde_json::Value = serde_json::from_str(&ongoing_games).unwrap();
    // info!("{v:?}");

    let current_games = &v["nowPlaying"];
    info!("current games: {current_games:?}");

    let n_current_games = current_games.as_array().unwrap().len();
    info!("number of currently active games: {n_current_games}");
    let game_id = join_game(&client, n_current_games, &current_games[0])
        .await
        .unwrap();

    info!("current game ID: {game_id}");

    // stream the game
    info!("streaming game {game_id}");
    let mut stream = client
        .get(format!("{LICHESS_HOST}/api/board/game/stream/{game_id}"))
        .bearer_auth(LICHESS_TOKEN)
        .send()
        .await
        .unwrap()
        .bytes_stream();

    let game = stream.next().await.unwrap().unwrap();
    let game: serde_json::Value = serde_json::from_slice(&game).unwrap();

    // get the color
    let colour = game["color"].as_str().unwrap();
    let is_our_turn = game["isMyTurn"].as_bool().unwrap();
    info!("is our turn: {is_our_turn}");
    let colour = if is_our_turn {
        colour
    } else {
        match colour {
            "white" => "black",
            "black" => "white",
            _ => panic!("invalid colour"),
        }
    };
    info!("colour: {colour}");

    info!("entering game stream loop");
    let mut stm = colour;
    while let Some(line) = stream.next().await {
        let line = line.unwrap();

        // skip newlines
        let line_ref = std::str::from_utf8(line.as_ref()).unwrap();
        if line_ref.trim().is_empty() {
            continue;
        }

        debug!("line: {line:?}");

        // parse the json string
        let v: serde_json::Value = serde_json::from_slice(&line).unwrap();
        debug!("serdejson value: {v:?}\n");

        if v.get("type").map(|t| t.as_str().unwrap()) == Some("chatLine") {
            info!("chat line: {} says {}", v["username"], v["text"]);
            continue;
        }

        // continue if we're not to move:
        if stm != colour {
            stm = match stm {
                "white" => "black",
                "black" => "white",
                _ => panic!("invalid stm"),
            };
            continue;
        }

        // get the fen
        let Some(moves) = v.get("state")
            .map_or_else(
                || v.get("moves").and_then(serde_json::Value::as_str), 
                |state| state.get("moves").and_then(serde_json::Value::as_str)
        ) else {
            warn!("no 'moves' or 'state' field in json string");
            return;
        };

        info!("moves made so far: {moves}");
        let mut board = Chess::default();
        let moves = moves.split_whitespace().collect::<Vec<_>>();
        for mv in &moves {
            let mv: Uci = mv.parse().unwrap();
            let mv = mv.to_move(&board).unwrap();
            board = board.play(&mv).unwrap();
        }

        println!("opponent's move: {}", moves.last().unwrap());

        // print the legal moves:
        let legal_moves = board.legal_moves();

        print!("legal moves: ");
        for m in &legal_moves {
            print!("{}, ", San::from_move(&board, m));
        }
        println!();

        let mut user_input = String::new();
        print!("enter move: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut user_input).unwrap();
        let user_input = user_input.trim();

        let user_move = user_input.parse::<San>().unwrap().to_move(&board).unwrap();

        // post the move in the form of a json string
        // like this: https://lichess.org/api/board/game/{gameId}/move/{move}

        let res = send_move_to_game(&client, &game_id, &user_move).await;

        let body = res.text().await.unwrap();

        info!("{body}");

        stm = match stm {
            "white" => "black",
            "black" => "white",
            _ => panic!("invalid stm"),
        };
    }
}