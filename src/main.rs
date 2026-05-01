use crate::api::Client;
use crate::routes::Game;

use std::env;
use std::str::FromStr;
use tokio::task::JoinSet;
use tracing::{Level, error, info, subscriber::SetGlobalDefaultError};
use tracing_subscriber::FmtSubscriber;

mod api;
mod routes;

fn setup_logging() -> Result<(), SetGlobalDefaultError> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging()?;

    let cookies = env::var("KIRARA_COOKIES")?;
    let client = Client::new(&cookies)?;

    let mut games = Vec::new();

    for s in env::var("KIRARA_GAMES")?.split(',') {
        match Game::from_str(s) {
            Ok(game) => games.push(game),
            Err(error) => error!("{}", error),
        }
    }

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
