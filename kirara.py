#!/usr/bin/env python

import asyncio
import logging
import os
import sys
from collections.abc import Coroutine
from typing import Any

import genshin


logging.basicConfig(
    format="[%(asctime)s %(levelname)s] %(message)s",
    datefmt="%Y-%m-%dT%H:%M:%S%z",
    level=logging.INFO,
)


def get_configuration() -> tuple[str, list[genshin.Game], list[str]]:
    cookies: str = os.getenv("COOKIES")

    if not cookies:
        raise ValueError("Environment variable `COOKIES` is not set.")

    env_games: str = os.getenv("GAMES")

    if not env_games:
        raise ValueError(
            'Environment variable `GAMES` is not set (e.g., "GENSHIN,HONKAI,STARRAIL,ZZZ").'
        )

    games: list[str] = [
        game.strip().upper() for game in env_games.split(",") if game.strip()
    ]
    valid_games: list[genshin.Game] = []
    invalid_games: list[str] = []

    for game in games:
        try:
            valid_games.append(genshin.Game[game])
        except KeyError:
            invalid_games.append(game)

    if not valid_games:
        raise ValueError("Environment variable `GAMES` contains no valid games.")

    return cookies, valid_games, invalid_games


async def claim_daily_reward(
    client: genshin.Client, game: genshin.Game
) -> None:
    try:
        await client.claim_daily_reward(game=game, reward=False)
        logging.info(f"Successfully claimed the daily reward for `{game.name}`.")
    except genshin.AlreadyClaimed:
        logging.info(f"Already claimed the daily reward for `{game.name}` today.")


async def main() -> None:
    try:
        cookies, valid_games, invalid_games = get_configuration()

        for game in invalid_games:
            logging.warning(f"`{game}` is not a valid game.")
    except ValueError as e:
        logging.error(e)
        sys.exit(1)

    client: genshin.Client = genshin.Client(cookies=cookies)
    coroutines: list[Coroutine[Any, Any, None]] = [
        claim_daily_reward(client, game) for game in valid_games
    ]

    try:
        await asyncio.gather(*coroutines)
    except genshin.InvalidCookies:
        logging.error("Invalid cookies detected.")
        sys.exit(1)


if __name__ == "__main__":
    asyncio.run(main())
