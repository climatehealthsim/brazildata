use anyhow::{Result, bail};
use std::{str::FromStr, collections::HashSet};

pub trait CapitalDeNotificacao {
    fn capital_de_notificacao(&self) -> Result<&str>;
}

pub trait RecentTable<R> {
    fn records(&self) -> &Vec<R>;
}

pub fn capitals_from_table<'t,
                           R: CapitalDeNotificacao + 't,
                           T: RecentTable<R>>(
    table: &'t T
) -> Result<HashSet<&'t str>>
{
    let mut set = HashSet::new();
    for record in table.records() {
        set.insert(record.capital_de_notificacao()?);
    }
    Ok(set)
}


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
