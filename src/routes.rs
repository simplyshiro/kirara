use std::fmt;
use std::fmt::{Display, Formatter};
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
            Self::GenshinImpact => "hk4e",
            Self::HonkaiImpact3rd => "bh3",
            Self::HonkaiStarRail => "hkrpg",
            Self::TearsOfThemis => "nxx",
            Self::ZenlessZoneZero => "zzz",
        }
    }

    pub fn reward_url(self) -> &'static str {
        match self {
            Self::GenshinImpact => {
                "https://sg-hk4e-api.hoyolab.com/event/sol/sign?act_id=e202102251931481"
            }

            Self::HonkaiImpact3rd => {
                "https://sg-public-api.hoyolab.com/event/mani/sign?act_id=e202110291205111"
            }

            Self::HonkaiStarRail => {
                "https://sg-public-api.hoyolab.com/event/luna/hkrpg/os/sign?act_id=e202303301540311"
            }

            Self::TearsOfThemis => {
                "https://sg-public-api.hoyolab.com/event/luna/nxx/os/sign?act_id=e202202281857121"
            }

            Self::ZenlessZoneZero => {
                "https://sg-public-api.hoyolab.com/event/luna/zzz/os/sign?act_id=e202406031448091"
            }
        }
    }
}

#[derive(Debug)]
pub struct ParseGameError(String);

impl Display for ParseGameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "`{}` is not a valid game", self.0)
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        match s.trim().to_uppercase().as_str() {
            "GI" => Ok(Self::GenshinImpact),
            "HI3" => Ok(Self::HonkaiImpact3rd),
            "HSR" => Ok(Self::HonkaiStarRail),
            "TOT" => Ok(Self::TearsOfThemis),
            "ZZZ" => Ok(Self::ZenlessZoneZero),
            _ => Err(ParseGameError(s.to_string())),
        }
    }
}
