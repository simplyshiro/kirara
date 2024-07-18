#!/usr/bin/env python3

import asyncio
import http.cookies
import os
import genshin


async def main():
    cookies = http.cookies.SimpleCookie(os.getenv("COOKIES"))

    if (os.getenv("GENSHIN")):
        await claim_daily_reward(game=genshin.Game.GENSHIN, cookies=cookies)

    if (os.getenv("HONKAI")):
        await claim_daily_reward(game=genshin.Game.HONKAI, cookies=cookies)

    if (os.getenv("STARRAIL")):
        await claim_daily_reward(game=genshin.Game.STARRAIL, cookies=cookies)

    if (os.getenv("ZZZ")):
        await claim_daily_reward(game=genshin.Game.ZZZ, cookies=cookies)


async def claim_daily_reward(game: genshin.Game,
                             cookies: http.cookies.SimpleCookie):
    client = genshin.Client(game=game)

    await set_cookies(client, cookies)

    try:
        await client.claim_daily_reward(reward=False)
    except genshin.AlreadyClaimed:
        print("Already claimed the daily reward today.")


async def set_cookies(client: genshin.Client,
                      cookies: http.cookies.SimpleCookie):
    account_id_v2 = cookies["account_id_v2"].value
    cookie_token_v2 = cookies["cookie_token_v2"].value

    client.set_cookies(account_id_v2=account_id_v2,
                       cookie_token_v2=cookie_token_v2)


if __name__ == '__main__':
    asyncio.run(main())
