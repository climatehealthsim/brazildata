use anyhow::{Result, bail};
use std::str::FromStr;


pub fn remove_coordinates(s: &str) -> Result<&str> {
    if let Some((coord, cap)) = s.split_once(' ') {
        if let Err(e) = u64::from_str(coord) {
            bail!("expected coordinates in {s:?}, but: {e}")
        }
        Ok(cap)
    } else {
        bail!("coordinates_* field does not have a space: {s:?}")
    }
}
