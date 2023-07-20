use anyhow::{Result, bail};
use kstring::KString;
use serde::Deserialize;

use crate::{easycsv::{CsvOption, self}, util::{RecentTable, CapitalDeNotificacao}};


#[derive(Debug, Deserialize)]
pub struct RecentRecord5 {
    capital: KString,
    pub ignorado: CsvOption<u64>,
    pub urbana: CsvOption<u64>,
    pub rural: CsvOption<u64>,
    pub urb_or_rural: CsvOption<u64>,
}

impl RecentRecord5 {
    fn check(&self) -> Result<()> {
        Ok(())
    }
    pub fn get_column(&self, col: usize) -> Option<u64> {
        match col {
            1 => &self.ignorado,
            2 => &self.urbana,
            3 => &self.rural,
            4 => &self.urb_or_rural,
            _ => panic!("invalid col number {col}")
        }.into()
    }
}

impl CapitalDeNotificacao for RecentRecord5 {
    fn capital_de_notificacao(&self) -> Result<&str> {
        // XXX is it correct to treat capital as capital_de_notificacao ?
        Ok(&self.capital)
    }
}    


// Capital de notificação	Ign/Ñ preenchido	Urbana	Rural	Urbana/Rural
// Fonte: Ministério da Saúde/SVS - Sistema de Informação de Agravos de Notificação - Sinan
// Notas:
// Excluídos casos não residentes no Brasil
// Períodos Disponíveis ou período - Correspondem aos anos de notificação dos casos.
const TSV : &'static str = "Capitais	ignorado	Urbana	Rural	urb/rural
São Paulo	27	672	4	2
Rio Branco	14	426	38	1
Recife	10	414	10	-
Curitiba	3	276	8	-
Belém	14	222	14	3
Salvador	2	230	-	-
Rio de Janeiro	10	176	1	-
Porto Alegre	1	150	7	5
Maceió	1	139	6	-
Fortaleza	-	119	12	-
Manaus	2	89	4	1
Macapá	1	83	6	-
Brasília	3	61	11	5
Florianópolis	2	47	6	4
Aracaju	-	56	1	-
Vitória	2	45	5	1
Belo Horizonte	3	40	5	2
São Luís	3	29	13	1
João Pessoa	-	22	-	1
Natal	1	7	1	-
Porto Velho	1	7	1	-
Campo Grande	-	7	1	-
Goiânia	-	3	1	-
Cuiabá	-	1	1	-
Teresina	1	-	-	-
Palmas	-	1	-	-
Boa Vista	-	-	1	-
";

const TOTAL : &[Option<u64>] = &[ None, Some(101), Some(3322), Some(157), Some(26) ];



#[derive(Debug)]
pub struct RecentTable5 {
    desc: String,
    records: Vec<RecentRecord5>,
}
    
impl RecentTable5 {
    fn check(&self) -> Result<()> {
        let mut totals = [None, Some(0), Some(0), Some(0), Some(0)];
        for record in &self.records {
            record.check()?;
            for i in 1..5 {
                // HACK? again like in recent4
                if let Some(n) = record.get_column(i) {
                    *(totals[i].as_mut().unwrap()) += n;
                }
            }
        }
        if totals != TOTAL {
            bail!("totals do not match: got {totals:?}, expected {TOTAL:?}")
        } 
        Ok(())
    }
    
    pub fn get() -> Result<RecentTable5> {
        let records = easycsv::parse_tsv::<RecentRecord5>(TSV.as_bytes())?;
        let tabl = RecentTable5 {
            desc: String::from(file!()),
            records
        };
        tabl.check()?;
        Ok(tabl)
    }
}

impl RecentTable<RecentRecord5> for RecentTable5 {
    fn desc(&self) -> &str { &self.desc }
    fn records(&self) -> &Vec<RecentRecord5> { &self.records }
}
