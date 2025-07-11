#!/usr/bin/env python3

import asyncio
import http.cookies
import os
import sys
import genshin


async def main():
    env_cookies = os.getenv("COOKIES")

    if not env_cookies:
        print("Environment variable COOKIES is not set.")
        sys.exit(1)

    try:
        cookies = http.cookies.SimpleCookie(env_cookies)
    except http.cookies.CookieError:
        print("Error parsing cookies.")
        sys.exit(1)

    required_cookies = ["account_id_v2", "cookie_token_v2"]

    for cookie in required_cookies:
        if cookie not in cookies:
            print(f"Missing {cookie} in the COOKIES environment variable.")
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


async def claim_daily_reward(game: genshin.Game, cookies: http.cookies.SimpleCookie):
    client = genshin.Client(game=game)

    await set_cookies(client, cookies)

    try:
        await client.claim_daily_reward(reward=False)
    except genshin.AlreadyClaimed:
        print(f"Already claimed the daily reward for {game.name} today.")
    except genshin.InvalidCookies:
        print("Either account_id_v2 or cookie_token_v2 is invalid.")
        sys.exit(1)


async def set_cookies(client: genshin.Client, cookies: http.cookies.SimpleCookie):
    account_id_v2 = cookies["account_id_v2"].value
    cookie_token_v2 = cookies["cookie_token_v2"].value

    client.set_cookies(account_id_v2=account_id_v2, cookie_token_v2=cookie_token_v2)


if __name__ == "__main__":
    asyncio.run(main())
