# kirara

Automatically claim daily rewards in [HoYoLab](https://www.hoyolab.com/).

This project was inspired by [collei](https://github.com/c4em/collei), which is no longer available.

## Setup

Clone this repository and install the dependencies:
```sh
git clone https://github.com/simplyshiro/kirara.git
cd kirara
python -m pip install -r requirements.txt
```

Login to [HoYoLab](https://www.hoyolab.com/), open your browser's developer tools, go to the **Application** or **Storage** tab, go to the HoYoLab domain on the **Cookies** section, and copy the values of `account_id_v2` and `cookie_id_v2`.

Set the `COOKIES` environment variable with your `account_id_v2` and `cookie_id_v2` in the following format below.

`account_id_v2=123xxxx; cookie_id_v2=v2_abcxxxx...`

Set the `GAMES` environment variable to a comma-separated list of the games you want claim daily rewards for.

### Supported Games

- `GENSHIN`
- `HONKAI`
- `STARRAIL`
- `ZZZ`

## Usage

Run the script:
```sh
python kirara.py
```

## Disclaimer

**kirara** is not affiliated with or endorsed by HoYoverse. Using third-party tools could potentially lead to your account being suspended or banned.

By using this script, you acknowledge and agree that you are doing so at your own risk. The developer of this project is not responsible for any consequences, including but not limited to account suspensions, bans, or loss of game data. Please proceed with caution.
