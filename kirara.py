#!/usr/bin/env python3

import asyncio
import http.cookies
import os
import sys
import genshin


async def main():
    cookies = os.getenv("COOKIES")

    if not cookies:
        print("Environment variable COOKIES is not set.")
        sys.exit(1)

    env_games = os.getenv("GAMES")
    games_to_claim_daily_reward = []

    if env_games:
        games_to_claim_daily_reward = [
            game.strip().upper() for game in env_games.split(",") if game.strip()
        ]

    if not games_to_claim_daily_reward:
        print(
            'No specified games to claim daily reward, set the GAMES environment variable (e.g., GAMES="GENSHIN,HONKAI,STARRAIL,ZZZ").'
        )
        sys.exit(0)

    for game in games_to_claim_daily_reward:
        try:
            await claim_daily_reward(game=genshin.Game[game], cookies=cookies)
        except KeyError:
            print(f"{game} is not a valid game. Skipping.")


async def claim_daily_reward(game: genshin.Game, cookies: str):
    client = genshin.Client(game=game, cookies=cookies)

    try:
        await client.claim_daily_reward(reward=False)
    except genshin.AlreadyClaimed:
        print(f"Already claimed the daily reward for {game.name} today.")
    except genshin.InvalidCookies:
        print("Invalid cookies detected.")
        sys.exit(1)


if __name__ == "__main__":
    asyncio.run(main())
