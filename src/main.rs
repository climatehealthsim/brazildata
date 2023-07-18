use std::{io::Read, marker::PhantomData, str::FromStr, ops::Deref, any::type_name};
use serde::{Deserialize, Deserializer, de::Visitor};
use anyhow::Result;

#[derive(Debug)]
struct CsvOption<T>(Option<T>);

impl<T> Deref for CsvOption<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
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
        println!("deserialize");
        deserializer.deserialize_str(CSVOptionVisitor::new())
    }
}

fn parse_tsv_without_headers<T: for<'de> serde::Deserialize<'de>>(input: impl Read) -> Result<Vec<T>> {
    let mut readerbuilder = csv::ReaderBuilder::new();
    readerbuilder.delimiter(b'\t');
    readerbuilder.has_headers(false);
    let mut reader = readerbuilder.from_reader(input);
    let mut records : Vec<T> = Vec::new();
    let mut iter = reader.deserialize().into_iter();
    for rowresult in iter {
        let rowresult = rowresult?;
        let record : T = rowresult;
        records.push(record);
    }
    Ok(records)
}

// Notification capital	Ign/White	Illiterate	1st to 4th incomplete grade of FS	4th complete grade of FS	5th to 8th incomplete grade of FS	Complete elementary school	Incomplete high school	Complete higher education	Incomplete higher education	Complete higher education	Does not apply
const RECENT_TABLES_TSV : &'static str = "
355030 São Paulo	1042	18	134	93	297	197	104	148	21	26	14
120040 Rio Branco	195	69	367	122	398	79	226	317	49	36	52
261160 Recife	691	29	106	37	207	36	52	65	6	4	35
410690 Curitiba	557	10	83	48	149	55	75	84	8	23	11
292740 Salvador	443	3	44	66	108	40	23	27	2	2	17
431490 Porto Alegre	174	3	36	35	155	83	31	65	12	20	5
150140 Belém	241	1	51	43	98	52	43	42	2	3	4
330455 Rio de Janeiro	392	2	21	14	56	22	20	29	4	7	10
270430 Maceió	40	64	117	30	127	26	25	50	1	6	9
160030 Macapá	88	12	83	34	81	34	34	39	4	8	58
130260 Manaus	265	4	18	26	42	21	22	30	3	4	6
280030 Aracaju	77	25	121	15	96	20	11	33	-	3	3
230440 Fortaleza	211	6	50	13	35	17	15	17	3	3	11
420540 Florianópolis	122	6	16	17	59	32	18	22	3	12	3
320530 Vitória	96	3	14	9	30	10	21	30	5	7	14
211130 São Luís	46	1	45	17	45	16	19	37	1	4	5
110020 Porto Velho	74	7	41	13	46	13	13	18	3	1	7
530010 Brasília	115	3	16	13	23	8	14	12	2	1	1
310620 Belo Horizonte	150	-	3	9	14	10	2	2	-	7	3
240810 Natal	41	2	5	4	5	10	5	8	-	4	3
250750 João Pessoa	44	5	9	4	8	5	6	5	-	-	-
520870 Goiânia	15	-	3	-	9	-	-	5	1	1	6
500270 Campo Grande	15	1	1	1	3	2	-	2	3	2	2
221100 Teresina	6	3	2	-	8	2	5	2	-	-	1
140010 Boa Vista	7	-	1	1	2	-	2	3	1	1	-
172100 Palmas	5	1	-	1	-	1	-	1	1	-	-
510340 Cuiabá	-	-	-	2	-	2	1	1	-	-	-
";

#[derive(Debug, Deserialize)]
struct RecentEntry {
    capital: String,
    ign_per_white: CsvOption<u64>,
    illiterate: CsvOption<u64>,
    _1st_to_4th_incomplete_grade_of_fs: CsvOption<u64>,
    _4th_complete_grade_of_fs: CsvOption<u64>,
    _5th_to_8th_incomplete_grade_of_fs: CsvOption<u64>,
    complete_elementary_school: CsvOption<u64>,
    incomplete_medium_education: CsvOption<u64>,
    complete_medium_education: CsvOption<u64>,
    incomplete_higher_education: CsvOption<u64>,
    complete_higher_education: CsvOption<u64>,
    does_not_apply: CsvOption<u64>,
}

fn main() -> Result<()> {
    let records = parse_tsv_without_headers::<RecentEntry>(RECENT_TABLES_TSV.as_bytes())?;
    println!("{records:?}");

    for record in records {
        if let Some(ed) = *record.incomplete_medium_education {
            println!("ed = {ed}");
        } else {
            println!("ed = -");
        }
    }
    
    Ok(())
}
