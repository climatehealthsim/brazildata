use anyhow::{Result, bail};
use serde::Deserialize;

use crate::{easycsv, util::remove_coordinates};

#[derive(Debug, Deserialize)]
pub struct RecentRecord2 {
    coordinates_and_capitais: String, // capitais = capitals
    pub ign_branco: u64,
    pub cura: u64,
    pub obito: u64,
}

impl RecentRecord2 {
    fn check(&self) -> Result<()> {
        Ok(())
    }
    #[allow(dead_code)]
    pub fn capital(&self) -> Result<&str> {
        remove_coordinates(&self.coordinates_and_capitais)
    }
    pub fn get_column(&self, col: usize) -> u64 {
        match col {
            1 => self.ign_branco,
            2 => self.cura,
            3 => self.obito,
            _ => panic!("invalid col number {col}")
        }
    }
}

const TSV : &'static str = "Capitais	Ign/branco	cura 	óbito
355030 São Paulo	342	2021	419
120040 Rio Branco	35	2315	38
261160 Recife	139	1381	175
410690 Curitiba	56	1128	197
150140 Belém	457	632	125
292740 Salvador	75	794	132
330455 Rio de Janeiro	94	514	154
431490 Porto Alegre	17	687	74
270430 Maceió	60	519	44
230440 Fortaleza	28	414	62
130260 Manaus	17	466	53
160030 Macapá	53	501	11
530010 Brasília	36	221	29
420540 Florianópolis	18	325	24
280030 Aracaju	13	324	121
320530 Vitória	47	213	32
310620 Belo Horizonte	20	193	34
211130 São Luís	18	209	50
250750 João Pessoa	4	78	26
240810 Natal	16	68	11
110020 Porto Velho	7	221	10
500270 Campo Grande	3	34	2
520870 Goiânia	2	35	7
510340 Cuiabá	2	4	2
221100 Teresina	12	18	0
172100 Palmas	2	8	1
140010 Boa Vista	8	9	2
";
const TOTAL : &[Option<u64>] = &[None, Some(1581), Some(13332), Some(1835)];

#[derive(Debug)]
pub struct RecentTable2 {
    pub records: Vec<RecentRecord2>,
}
    
impl RecentTable2 {
    fn check(&self) -> Result<()> {
        let mut totals = [None, Some(0), Some(0), Some(0)];
        for record in &self.records {
            record.check()?;
            for i in 1..4 {
                *(totals[i].as_mut().unwrap()) += record.get_column(i);
            }
        }
        if totals != TOTAL {
            bail!("totals do not match: got {totals:?}, expected {TOTAL:?}")
        } 
        Ok(())
    }
    
    pub fn get() -> Result<RecentTable2> {
        let records = easycsv::parse_tsv::<RecentRecord2>(TSV.as_bytes())?;
        let tabl = RecentTable2 { records };
        tabl.check()?;
        Ok(tabl)
    }
    
}
