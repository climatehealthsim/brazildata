use std::{io::Read, marker::PhantomData, str::FromStr, ops::Deref, any::type_name, fmt::Display};
use serde::{Deserialize, Deserializer, de::Visitor};
use anyhow::Result;

#[derive(Debug)]
pub struct CsvOption<T>(pub Option<T>);

impl<T> Deref for CsvOption<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// ever useless (implicit conversion this not is; and have to give
// type, stupid). use deref instead.
impl<T> From<CsvOption<T>> for Option<T> {
    fn from(v: CsvOption<T>) -> Self {
        v.0
    }
}

// actually useful when having a & but need to return an option
impl<T: Clone> From<&CsvOption<T>> for Option<T> {
    fn from(v: &CsvOption<T>) -> Self {
        v.0.clone()
    }
}


struct CSVOptionVisitor<T> {
    _marker: PhantomData<T>,
}
impl<T> CSVOptionVisitor<T> {
    fn new() -> CSVOptionVisitor<T> {
        CSVOptionVisitor { _marker: PhantomData::default() }
    }
}

impl<'v, T: FromStr> Visitor<'v> for CSVOptionVisitor<T>
{
    type Value = CsvOption<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a ")?;
        formatter.write_str(type_name::<T>())
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // println!("OK got a str: {v}");
        match T::from_str(v) {
            Ok(n) => Ok(CsvOption(Some(n))),
            Err(_e) =>
                if v == "-" {
                    Ok(CsvOption(None))
                } else {
                    // Err(_e)
                    Err(serde::de::Error::invalid_type(serde::de::Unexpected::Str(v), &self))
                }
        }
    }
}

impl<'d, T: FromStr> Deserialize<'d> for CsvOption<T> {
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error>
    {
        // println!("deserialize");
        deserializer.deserialize_str(CSVOptionVisitor::new())
    }
}

pub fn parse_tsv_<T: for<'de> serde::Deserialize<'de>>(input: impl Read, header: bool) -> Result<Vec<T>> {
    let mut readerbuilder = csv::ReaderBuilder::new();
    readerbuilder.delimiter(b'\t');
    readerbuilder.has_headers(false);
    let mut reader = readerbuilder.from_reader(input);
    let mut records : Vec<T> = Vec::new();
    let mut iter = reader.deserialize().into_iter();
    if header {
        iter.next();
    }
    for rowresult in iter {
        let rowresult = rowresult?;
        let record : T = rowresult;
        records.push(record);
    }
    Ok(records)
}

#[allow(dead_code)]
pub fn parse_tsv_noheader<T: for<'de> serde::Deserialize<'de>>(input: impl Read) -> Result<Vec<T>> {
    parse_tsv_(input, false)
}

pub fn parse_tsv<T: for<'de> serde::Deserialize<'de>>(input: impl Read) -> Result<Vec<T>> {
    parse_tsv_(input, true)
}

pub fn optionfmt<T: Display>(v: Option<T>) -> String {
    if let Some(v) = v {
        format!("{v}")
    } else {
        String::from("-")
    }
}
