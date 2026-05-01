use std::fmt::{self, Display};
use std::result;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Game {
    GenshinImpact,
    HonkaiImpact3rd,
    HonkaiStarRail,
    TearsOfThemis,
    ZenlessZoneZero,
}

impl Game {
    pub fn id(self) -> &'static str {
        match self {
            Game::GenshinImpact => "hk4e",
            Game::HonkaiImpact3rd => "bh3",
            Game::HonkaiStarRail => "hkrpg",
            Game::TearsOfThemis => "nxx",
            Game::ZenlessZoneZero => "zzz",
        }
    }

    pub fn reward_url(self) -> &'static str {
        match self {
            Game::GenshinImpact => {
                "https://sg-hk4e-api.hoyolab.com/event/sol/sign?act_id=e202102251931481"
            }

            Game::HonkaiImpact3rd => {
                "https://sg-public-api.hoyolab.com/event/mani/sign?act_id=e202110291205111"
            }

            Game::HonkaiStarRail => {
                "https://sg-public-api.hoyolab.com/event/luna/hkrpg/os/sign?act_id=e202303301540311"
            }

            Game::TearsOfThemis => {
                "https://sg-public-api.hoyolab.com/event/luna/nxx/os/sign?act_id=e202202281857121"
            }

            Game::ZenlessZoneZero => {
                "https://sg-public-api.hoyolab.com/event/luna/zzz/os/sign?act_id=e202406031448091"
            }
        }
    }
}

#[derive(Debug)]
pub struct ParseGameError(String);

impl Display for ParseGameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}` is not a valid game", self.0)
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        match s.trim().to_uppercase().as_str() {
            "GI" => Ok(Game::GenshinImpact),
            "HI3" => Ok(Self::HonkaiImpact3rd),
            "HSR" => Ok(Game::HonkaiStarRail),
            "TOT" => Ok(Self::TearsOfThemis),
            "ZZZ" => Ok(Self::ZenlessZoneZero),
            _ => Err(ParseGameError(s.to_string())),
        }
    }
}
