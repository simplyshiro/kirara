#!/usr/bin/env python

import asyncio
import logging
import os
import sys
from types import CoroutineType
from typing import Any

import genshin


def get_environment() -> tuple[str | None, str | None]:
    return os.getenv("COOKIES"), os.getenv("GAMES")


async def claim_daily_reward(client: genshin.Client, game: genshin.Game) -> None:
    await client.claim_daily_reward(game=game, reward=False)


async def main() -> None:
    logging.basicConfig(
        format="[%(asctime)s %(levelname)s] %(message)s",
        datefmt="%Y-%m-%dT%H:%M:%S%z",
        level=logging.INFO,
    )

    env_cookies, env_games = get_environment()

    if not env_cookies:
        logging.error("Environment variable 'COOKIES' is not set")
        sys.exit(1)

    if not env_games:
        logging.error("Environment variable 'GAMES' is not set")
        sys.exit(1)

    games: list[str] = [
        env_game.strip().upper()
        for env_game in env_games.split(",")  # ty: ignore[possibly-missing-attribute]
        if env_game.strip()
    ]
    valid_games: list[genshin.Game] = []

    for game in games:
        try:
            valid_games.append(genshin.Game[game])
        except KeyError:
            logging.warning(f"'{game}' is not a valid game")

    if not valid_games:
        logging.error("Environment variable 'GAMES' contains no valid games")
        sys.exit(1)

    client: genshin.Client = genshin.Client(cookies=env_cookies)
    coroutines: list[CoroutineType[Any, Any, None]] = [
        claim_daily_reward(client, valid_game) for valid_game in valid_games
    ]
    results: list[Any | BaseException] = await asyncio.gather(
        *coroutines, return_exceptions=True
    )

    for valid_game, result in zip(valid_games, results):
        match result:
            case None:
                logging.info(
                    f"Successfully claimed the daily reward for '{valid_game.name}'"
                )
            case genshin.AlreadyClaimed():
                logging.info(
                    f"Already claimed the daily reward for '{valid_game.name}' today"
                )
            case genshin.InvalidCookies():
                logging.error("Invalid cookies set")
                sys.exit(1)
            case Exception() as e:
                logging.error(
                    f"Failed to claim daily reward for '{valid_game.name}': {e}"
                )


if __name__ == "__main__":
    asyncio.run(main())
