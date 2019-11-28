use rand::prelude::*;

#[macro_use]
extern crate log;

mod support;
use support::*;

const PLAYER_COUNT: usize = 7;
const ITERATIONS: usize = 1_000_000;

fn main() {
    pretty_env_logger::init();

    let mut players: Vec<Player> = vec![
        Player::new("Rob"),
        Player::new("Kirsten"),
        Player::new("Blair"),
        Player::new("Dave"),
        Player::new("Brett"),
        Player::new("Patrick"),
        Player::new("Jack"),
    ];
    let mut rng = rand::thread_rng();

    let mut length = 0;

    // GAME
    for i in 0..ITERATIONS {
        if i % (ITERATIONS / 100) == 0 {
            println!("At {}%", (i * 100) / ITERATIONS);
        }

        let mut caret = 0;
        let mut pot = 0;

        // Give everyone 3 points...
        for player in players.iter_mut() {
            player.score = 3;
        }

        while game_is_done(&players) == false {
            let player = caret;
            let left_player = get_left_player(caret);
            let right_player = get_right_player(caret);

            roll_die(
                player,
                left_player,
                right_player,
                &mut pot,
                &mut players,
                &mut rng,
            );
            caret = get_left_player(caret);

            length += 1;
        }

        let final_player = get_final_player(&mut players);
        players[final_player].victories += 1;
    }

    println!("Average length is {}", length / ITERATIONS);
    println!("Winners: {:#?}", players);
}

fn get_final_player(players: &mut [Player]) -> usize {
    for (i, player) in players.iter().enumerate() {
        if player.score != 0 {
            return i;
        }
    }

    panic!("We don't have a player to return! Did no one win somehow?");
}

fn game_is_done(players: &[Player]) -> bool {
    let mut ret = 2;

    for player in players {
        if player.score != 0 {
            ret -= 1;

            if ret == 0 {
                break;
            }
        }
    }

    if ret == 2 {
        error!("ERROR: No one has anything! This is wrong!");
    }

    ret == 1
}

fn get_left_player(caret: usize) -> usize {
    let next_player = caret + 1;
    if next_player == PLAYER_COUNT {
        0
    } else {
        next_player
    }
}

fn get_right_player(caret: usize) -> usize {
    if caret == 0 {
        PLAYER_COUNT - 1
    } else {
        caret - 1
    }
}

fn roll_die(
    current_player: usize,
    left_player: usize,
    right_player: usize,
    pot: &mut usize,
    players: &mut Vec<Player>,
    thread_rng: &mut ThreadRng,
) {
    let dice_amount = players[current_player].score.min(3);
    info!("Score is {}", players[current_player].score);

    for _ in 0..dice_amount {
        info!("Rolled!");
        let roll: Roll = thread_rng.gen_range(0, 6).into();

        players[current_player].score -= 1;

        match roll {
            Roll::Dot {} => players[current_player].score += 1,
            Roll::Left => players[left_player].score += 1,
            Roll::Right => players[right_player].score += 1,
            Roll::Star => *pot += 1,
        }
    }

    info!("---\n");
}
