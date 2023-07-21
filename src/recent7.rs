use anyhow::{Result, bail};
use kstring::KString;
use serde::Deserialize;

use crate::{easycsv::{CsvOption, self}, util::{CapitalDeNotificacao, RecentTable}};

#[derive(Debug, Deserialize)]
pub struct RecentRecord7 {
    capital: KString,
    pub ig_nao_preench: CsvOption<u64>,
    pub nenhum_ano_concluido: CsvOption<u64>,
    pub _1_a_3_anos_concl: CsvOption<u64>,
    pub _4_a_7_anos_concl: CsvOption<u64>,
    pub _8_a_11_concl: CsvOption<u64>,
    pub _de_12_e_plus_anos_concl: CsvOption<u64>,
    pub nao_se_aplica: CsvOption<u64>,
}

impl RecentRecord7 {
    fn check(&self) -> Result<()> {
        Ok(())
    }
    pub fn get_column(&self, col: usize) -> Option<u64> {
        match col {
            1 => &self.ig_nao_preench,
            2 => &self.nenhum_ano_concluido,
            3 => &self._1_a_3_anos_concl,
            4 => &self._4_a_7_anos_concl,
            5 => &self._8_a_11_concl,
            6 => &self._de_12_e_plus_anos_concl,
            7 => &self.nao_se_aplica,
            _ => panic!("invalid col number {col}")
        }.into()
    }
}

impl CapitalDeNotificacao for RecentRecord7 {
    fn capital_de_notificacao(&self) -> Result<&str> {
        Ok(&self.capital)
    }
}    


// Notas:
// Excluídos casos não residentes no Brasil
// Períodos Disponíveis ou período - Correspondem aos anos de notificação dos casos.
const TSV: &str = "Capital	ig/nao preench	nenhum ano concluído	1 a 3 anos concl.	4 a 7 anos concl.	8 a 11 concl.	de 12 e + anos concl.	Nao se aplica
Porto Velho	1	-	1	3	4	-	-
Rio Branco	119	22	123	120	59	4	32
Manaus	27	2	10	28	21	4	4
Boa Vista	1	-	-	-	-	-	-
Belém	125	2	74	20	20	3	9
Macapá	10	4	9	28	27	5	7
Palmas	-	-	-	-	1	-	-
São Luís	17	-	5	9	8	5	2
Teresina	-	-	1	-	-	-	-
Fortaleza	63	7	23	13	16	2	7
Natal	7	1	-	1	-	-	-
João Pessoa	4	4	3	9	3	-	-
Recife	205	12	36	102	47	8	24
Maceió	10	21	27	65	22	1	-
Aracaju	14	8	13	13	7	2	-
Salvador	140	3	15	51	11	5	7
Belo Horizonte	39	-	2	2	5	1	1
Vitória	17	2	6	19	5	1	3
Rio de Janeiro	106	1	9	42	22	3	4
São Paulo	298	27	69	178	99	20	14
Curitiba	175	3	14	47	31	10	7
Florianópolis	25	-	1	13	13	4	3
Porto Alegre	38	1	13	53	38	14	6
Campo Grande	5	-	-	1	-	2	-
Cuiabá	1	-	1	-	-	-	-
Goiânia	-	-	-	3	-	1	-
Brasília	34	3	5	18	13	6	1
";
const TOTAL: &[Option<u64>] = &[None, Some(1481), Some(123), Some(460), Some(838), Some(472), Some(101), Some(131)];

#[derive(Debug)]
pub struct RecentTable7 {
    desc: String,
    records: Vec<RecentRecord7>,
}
    
impl RecentTable7 {
    fn check(&self) -> Result<()> {
        let mut totals = [None, Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0)];
        for record in &self.records {
            record.check()?;
            for i in 1..8 {
                // HACK? again like in recent{4,5,6}
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
    
    pub fn get() -> Result<RecentTable7> {
        let records = easycsv::parse_tsv::<RecentRecord7>(TSV.as_bytes())?;
        let tabl = RecentTable7 {
            desc: String::from(file!()),
            records
        };
        tabl.check()?;
        Ok(tabl)
    }
}

impl RecentTable<RecentRecord7> for RecentTable7 {
    fn desc(&self) -> &str { &self.desc }
    fn records(&self) -> &Vec<RecentRecord7> { &self.records }
}
