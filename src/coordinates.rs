use anyhow::{Result, bail};
use lazy_static::lazy_static;
use nav_types::{self, WGS84};
use regex::Regex;

#[derive(Copy, Clone, Debug)]
enum CoordinateDirection {
    Latitude, // north–south
    Longitude // east–west
}

#[derive(Copy, Clone, Debug)]
struct Coordinate {
    #[allow(unused)]
    direction: CoordinateDirection,
    is_negative: bool,
    deg: u8,
    min: u8,
    sec: u8
}

impl Coordinate {
    fn try_new(
        direction: CoordinateDirection,
        is_negative: bool,
        deg: u8,
        min: u8,
        sec: u8
    ) -> Result<Coordinate> {
        let max_deg = match direction {
            CoordinateDirection::Latitude => 90,
            CoordinateDirection::Longitude => 180,
        };
        if deg > max_deg {
            bail!("invalid degree value {deg} for {direction:?}")
        }
        if min > 59 {
            bail!("invalid minute value {min}")
        }
        if sec > 59 {
            bail!("invalid sec value {sec}")
        }
        if deg == max_deg {
            if min != 0 || sec != 0 {
                bail!("degree value is at the max {deg}, but min {min} or sec {sec} are not 0");
            }
        }
        Ok(Coordinate { direction, is_negative, deg, min, sec })
    }
}

impl From<Coordinate> for f64 {
    fn from(c: Coordinate) -> Self {
        let v = c.deg as f64 + c.min as f64 / 60. + c.sec as f64 / 3600.;
        if c.is_negative { -v } else { v }
    }
}

fn parse_direction_latitude(s: &str) -> Result<bool> {
    match s {
        "N" => Ok(false),
        "S" => Ok(true),
        _ => bail!("invalid latitude direction {s:?}")
    }
}

fn parse_direction_longitude(s: &str) -> Result<bool> {
    match s {
        "E" => Ok(false),
        "W" => Ok(true),
        "O" => Ok(true), // portuguese
        _ => bail!("invalid latitude direction {s:?}")
    }
}

pub fn parse_coordinates(s: &str, altitude_meters: f64) -> Result<WGS84<f64>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r#"^\s*(\d+)°\s*(\d+)['′]\s*(\d+)(?:″|"|'')\s*([NS])\s+(\d+)°\s*(\d+)['′]\s*(\d+)(?:″|"|'')\s*([EWO])\s*$"#
        ).unwrap();
    }
    if let Some(cap) = RE.captures(s) {
        let latitude_deg: u8 = cap[1].parse()?;
        let latitude_min: u8 = cap[2].parse()?;
        let latitude_sec: u8 = cap[3].parse()?;
        let latitude_direction: &str = &cap[4];

        let longitude_deg: u8 = cap[5].parse()?;
        let longitude_min: u8 = cap[6].parse()?;
        let longitude_sec: u8 = cap[7].parse()?;
        let longitude_direction: &str = &cap[8];

        let lat = Coordinate::try_new(
            CoordinateDirection::Latitude,
            parse_direction_latitude(latitude_direction)?,
            latitude_deg,
            latitude_min,
            latitude_sec
        )?;
        let lon = Coordinate::try_new(
            CoordinateDirection::Longitude,
            parse_direction_longitude(longitude_direction)?,
            longitude_deg,
            longitude_min,
            longitude_sec
        )?;
        Ok(WGS84::from_degrees_and_meters(lat.into(), lon.into(), altitude_meters))
    } else {
        bail!("string does not match coordinates format: {s:?}")
    }
}
