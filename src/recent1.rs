use anyhow::{Result, bail};
use serde::Deserialize;

use crate::easycsv;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RecentRecord1 {
    capital_de_notificacao: String,
    ign_branco: u64,
    domiciliar: u64,
    trabalho: u64,
    lazer: u64,
    outro: u64,
    total: u64,
}

impl RecentRecord1 {
    fn get_column(&self, col: usize) -> u64 {
        match col {
            1 => self.ign_branco,
            2 => self.domiciliar,
            3 => self.trabalho,
            4 => self.lazer,
            5 => self.outro,
            6 => self.total,
            _ => panic!("invalid col number {col}")
        }
    }
    #[allow(dead_code)]
    fn capital_de_notificacao(&self) -> &str {
        &self.capital_de_notificacao
    }
    // fn total(&self) -> Result<u64> {
    //     let tot = self.ign_branco + self.domiciliar + self.trabalho + self.lazer + self.outro;
    //     if tot != self.total {
    //         bail!("total failure: calculated {tot} but total field says {}", self.total)
    //     }
    //     Ok(tot)
    // }
    // ^ that's *not* it.
    fn check(&self) -> Result<()> {
        // self.total()?;
        Ok(())
    }
}

const TSV : &'static str = "capital de Notificaçao 	Ign/branco	domiciliar	trabalho	lazer	outro	total
São Paulo	697	1427	393	126	59	705
Rio Branco	379	1737	224	22	27	479
Recife	973	464	167	39	66	434
Curitiba	325	572	238	179	76	287
Belém	364	325	115	10	19	253
Salvador	341	206	85	14	247	232
Rio de Janeiro	367	337	89	16	32	187
Porto Alegre	280	192	181	83	46	163
Maceió	94	271	171	39	66	146
Fortaleza	351	95	47	16	9	131
Manaus	124	317	69	15	12	96
Macapá	43	479	23	8	12	90
Brasília	75	101	79	25	8	80
Florianópolis	71	145	111	28	14	59
Aracaju	90	200	127	16	28	57
Vitória	177	82	20	5	8	53
Belo Horizonte	160	50	21	7	12	50
São Luís	135	93	29	11	14	46
João Pessoa	46	42	15	2	5	23
Natal	50	23	18	2	4	9
Porto Velho	54	83	99	2	7	9
Campo Grande	15	8	9	4	4	8
Goiânia	29	8	5	1	0	4
Cuiabá	2	1	3	1	1	2
Teresina	16	4	7	1	2	1
Palmas	4	4	0	1	2	1
Boa Vista	11	6	2	0	0	1
";
const TOTAL : &[Option<u64>] = &[None, Some(5273), Some(7272), Some(2347), Some(673), Some(780), Some(3606)];

#[derive(Debug)]
pub struct RecentTable1 {
    pub records: Vec<RecentRecord1>,
}
    
impl RecentTable1 {
    fn check(&self) -> Result<()> {
        let mut totals = [None, Some(0), Some(0), Some(0), Some(0), Some(0), Some(0)];
        for record in &self.records {
            record.check()?;
            for i in 1..7 {
                *(totals[i].as_mut().unwrap()) += record.get_column(i);
            }
        }
        if totals != TOTAL {
            bail!("totals do not match: got {totals:?}, expected {TOTAL:?}")
        } 
        Ok(())
    }
    pub fn get() -> Result<RecentTable1> {
        let records = easycsv::parse_tsv::<RecentRecord1>(TSV.as_bytes())?;
        let tabl = RecentTable1 { records };
        tabl.check()?;
        Ok(tabl)
    }
}


