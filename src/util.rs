use anyhow::{Result, bail};
use std::{str::FromStr, collections::HashSet, hash::Hash};

// -----------------------------------------------------------------------------
// Generic utils

#[allow(unused)]
pub fn compare_sets<'t, T: Eq + Hash>(
    s1: &'t HashSet<T>, s2: &'t HashSet<T>
) -> (HashSet<&'t T>, HashSet<&'t T>)
{
    (s1.difference(&s2).collect(),
     s2.difference(&s1).collect())
}

// -----------------------------------------------------------------------------
// Capitals as keys

pub trait CapitalDeNotificacao {
    fn capital_de_notificacao(&self) -> Result<&str>;
}

pub trait RecentTable<R> {
    fn desc(&self) -> &str;
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
        let cap = record.capital_de_notificacao()?;
        if !set.insert(cap) {
            bail!("duplicate key {cap:?} in table {}",
                  table.desc());
        }
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

// -----------------------------------------------------------------------------
