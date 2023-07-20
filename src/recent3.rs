use anyhow::{Result, bail};
use kstring::KString;
use serde::Deserialize;

use crate::{util::{remove_coordinates, CapitalDeNotificacao, RecentTable}, easycsv};



#[derive(Debug, Deserialize)]
pub struct RecentRecord3 {
    coordinates_and_capitais: KString, // capitais = capitals
    pub masculino: u64, // cases
    pub feminino: u64, // cases
}

impl RecentRecord3 {
    fn check(&self) -> Result<()> {
        Ok(())
    }
    pub fn get_column(&self, col: usize) -> u64 {
        match col {
            1 => self.masculino,
            2 => self.feminino,
            _ => panic!("invalid col number {col}")
        }
    }
}

impl CapitalDeNotificacao for RecentRecord3 {
    fn capital_de_notificacao(&self) -> Result<&str> {
        // XXX is it correct to treat capital as capital_de_notificacao ?
        remove_coordinates(&self.coordinates_and_capitais)
    }
}    

const TSV : &'static str = "Capitais	Masculino	Feminino
355030 São Paulo	2294	505
120040 Rio Branco	1400	936
261160 Recife	1233	359
410690 Curitiba	1125	265
150140 Belém	647	186
292740 Salvador	841	166
330455 Rio de Janeiro	632	132
431490 Porto Alegre	666	116
270430 Maceió	517	124
230440 Fortaleza	303	109
130260 Manaus	436	101
160030 Macapá	361	204
530010 Brasília	242	46
420540 Florianópolis	303	66
280030 Aracaju	385	76
320530 Vitória	186	106
310620 Belo Horizonte	203	47
211130 São Luís	221	61
250750 João Pessoa	91	18
240810 Natal	74	22
110020 Porto Velho	166	79
500270 Campo Grande	34	6
520870 Goiânia	30	14
510340 Cuiabá	8	0
221100 Teresina	24	6
172100 Palmas	6	4
140010 Boa Vista	10	9
";
const TOTAL: &[Option<u64>] = &[None, Some(12438), Some(3763)];


#[derive(Debug)]
pub struct RecentTable3 {
    name: KString,
    records: Vec<RecentRecord3>,
}
    
impl RecentTable3 {
    fn check(&self) -> Result<()> {
        let mut totals = [None, Some(0), Some(0)];
        for record in &self.records {
            record.check()?;
            for i in 1..3 {
                *(totals[i].as_mut().unwrap()) += record.get_column(i);
            }
        }
        if totals != TOTAL {
            bail!("totals do not match: got {totals:?}, expected {TOTAL:?}")
        } 
        Ok(())
    }
    
    pub fn get() -> Result<RecentTable3> {
        let records = easycsv::parse_tsv::<RecentRecord3>(TSV.as_bytes())?;
        let tabl = RecentTable3 {
            name: KString::from(file!()),
            records
        };
        tabl.check()?;
        Ok(tabl)
    }
}

impl RecentTable<RecentRecord3> for RecentTable3 {
    fn name(&self) -> &str { &self.name }
    fn records(&self) -> &Vec<RecentRecord3> { &self.records }
}
