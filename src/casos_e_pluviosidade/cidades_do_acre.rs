use std::convert::identity;

use anyhow::{Result, bail};
use serde::Deserialize;

use crate::{easycsv::{CsvMonth, self, CsvOption}, util::RecentTable, staticdatabase::MunicipalityName};

const SOURCE : &str = "(Periodo de 2005 a 2014) tab 7 of 'CASOS CONFIRMADOS CAPITAL REGIAO E PLUVIOSIDADE.xlsx'";

#[derive(Debug, Deserialize)]
pub struct CidadesDoAcreRecord {
    pub month: CsvMonth,
    cases_by_city: [CsvOption<u64>; 5], // Cruzieiro do Sul	Tarauacá	Sena Madureira	Rio Branco	Brasileia
    pub total: u64, // careful, ignores missing data !
}

impl CidadesDoAcreRecord {
    fn check(&self) -> Result<()> {
        let total: u64 = self.cases_by_city.iter().map(|v| **v).filter_map(identity).sum();
        if total != self.total {
            bail!("total != self.total: {total} != {}", self.total)
        }
        Ok(())
    }
    pub fn get_column(&self, col: usize) -> Option<u64> {
        if col == 6 {
            Some(self.total)
        } else {
            *self.cases_by_city[col - 1]
        }
    }
    pub fn cases_by_municipalityname(&self, mun: MunicipalityName) -> Result<Option<u64>> {
        // future optimization: introduce integer IDs for matching
        let i =
            match mun.0 {
                "Cruzieiro do Sul" => 0,
	        "Tarauacá"=> 1,
                "Sena Madureira"=> 2,
	        "Rio Branco"=> 3,
	        "Brasileia"=> 4,
                _ => bail!("unknown {mun:?}")
            };
        Ok(*self.cases_by_city[i])
    }
}

const TSV: &str = "	Cruzieiro do Sul	Tarauacá	Sena Madureira	Rio Branco	Brasileia	Total
Marco	13	11	3	655	3	685
Abril	15	7	4	311	2	339
Fevereiro	43	8	1	310	-	362
Maio	30	7	1	185	1	224
Janeiro	20	5	1	199	2	227
Junho	27	1	-	119	1	148
Julho	38	7	-	98	2	145
Dezembro	9	3	-	137	1	150
Outubro	8	1	-	122	-	131
Novembro	14	10	1	91	2	118
Agosto	16	2	-	106	2	126
Setembro	2	5	-	78	1	86
";
const TOTAL: &[Option<u64>] = &[None, Some(235), Some(67), Some(11), Some(2411), Some(17), Some(2741)];

#[derive(Debug)]
pub struct CidadesDoAcreTable {
    desc: String,
    records: Vec<CidadesDoAcreRecord>,
}
    
impl CidadesDoAcreTable {
    fn check(&self) -> Result<()> {
        let mut totals = [None, Some(0), Some(0), Some(0), Some(0), Some(0), Some(0)];
        for record in &self.records {
            record.check()?;
            for i in 1..7 {
                // again ignoring missing values!
                if let Some(v) = record.get_column(i) {
                    *(totals[i].as_mut().unwrap()) += v;
                }
            }
        }
        if totals != TOTAL {
            bail!("totals do not match: got {totals:?}, expected {TOTAL:?}")
        } 
        Ok(())
    }
    
    pub fn get() -> Result<CidadesDoAcreTable> {
        let records = easycsv::parse_tsv::<CidadesDoAcreRecord>(TSV.as_bytes())?;
        let tabl = CidadesDoAcreTable {
            desc: String::from(file!()) + " (" + SOURCE + ")",
            records
        };
        tabl.check()?;
        Ok(tabl)
    }
}

impl RecentTable<CidadesDoAcreRecord> for CidadesDoAcreTable {
    fn desc(&self) -> &str { &self.desc }
    fn records(&self) -> &Vec<CidadesDoAcreRecord> { &self.records }
}
