mod api;
mod game;

use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::str::FromStr;
use tokio::task::JoinSet;
use tracing::Level;
use tracing::{error, info};

use crate::api::Client;
use crate::game::Game;

fn setup_logging() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
}

fn parse_games(input: &str) -> HashSet<Game> {
    input
        .split(',')
        .filter_map(|s| match Game::from_str(s) {
            Ok(game) => Some(game),
            Err(error) => {
                error!("{}", error);
                None
            }
        })
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_logging();

    let cookies = env::var("KIRARA_COOKIES")?;
    let client = Client::new(&cookies)?;
    let games = parse_games(&env::var("KIRARA_GAMES")?);

    if games.is_empty() {
        error!("no valid games found");
        return Ok(());
    }

    let mut tasks = JoinSet::new();

    for game in games {
        let c = client.clone();

        tasks.spawn(async move {
            let result = c.claim_daily_reward(game).await;
            (game, result)
        });
    }

    let results = tasks.join_all().await;

    for (game, result) in results {
        match result {
            Ok(response) => match response.retcode {
                0 => info!("claimed the daily reward for `{:?}`", game),
                -5003 => info!("already claimed the daily reward for `{:?}`", game),
                _ => error!(
                    "game `{:?}` returned code `{}` with message `{}`",
                    game, response.retcode, response.message
                ),
            },
            Err(error) => error!("`{:?}`: {}", game, error),
        }
    }

    Ok(())
}
