use chrono::{offset::LocalResult, DateTime as ChronoDateTime, SecondsFormat, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::ops::Deref;
use std::str;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct DateTime(pub ChronoDateTime<Utc>);

impl Default for DateTime {
    fn default() -> Self {
        Self(Utc::now())
    }
}

impl From<ChronoDateTime<Utc>> for DateTime {
    fn from(v: ChronoDateTime<Utc>) -> Self {
        Self(v)
    }
}

impl From<DateTime> for ChronoDateTime<Utc> {
    fn from(x: DateTime) -> Self {
        x.0
    }
}

impl FromStr for DateTime {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl TryFrom<String> for DateTime {
    type Error = ();
    fn try_from(v: String) -> Result<Self, Self::Error> {
        Self::try_from(v.as_str())
    }
}

impl TryFrom<&str> for DateTime {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Ok(v) =
            chrono::DateTime::parse_from_rfc3339(s).map(|dt| Self(dt.with_timezone(&Utc)))
        {
            Ok(v)
        } else if let Ok(v) = chrono::DateTime::parse_from_str(s, "%Y/%m/%d %H:%M:%S")
            .map(|dt| Self(dt.with_timezone(&Utc)))
        {
            Ok(v)
        } else if let Ok(v) = chrono::DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
            .map(|dt| Self(dt.with_timezone(&Utc)))
        {
            Ok(v)
        } else if let Ok(v) =
            chrono::DateTime::parse_from_str(s, "%Y-%m-%d").map(|dt| Self(dt.with_timezone(&Utc)))
        {
            Ok(v)
        } else {
            Err(())
        }
    }
}

impl TryFrom<(i64, u32)> for DateTime {
    type Error = ();
    fn try_from(v: (i64, u32)) -> Result<Self, Self::Error> {
        match Utc.timestamp_opt(v.0, v.1) {
            LocalResult::Single(v) => Ok(Self(v)),
            _ => Err(()),
        }
    }
}

impl Deref for DateTime {
    type Target = ChronoDateTime<Utc>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DateTime {
    /// Convert the Datetime to a raw String
    pub fn to_raw(&self) -> String {
        self.0.to_rfc3339_opts(SecondsFormat::AutoSi, true)
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.to_raw(), f)
    }
}
