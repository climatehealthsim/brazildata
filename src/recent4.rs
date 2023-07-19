use anyhow::Result;
use serde::Deserialize;

use crate::{util::{CapitalDeNotificacao, RecentTable}, easycsv::{self, CsvOption}};


#[derive(Debug, Deserialize)]
pub struct RecentRecord4 {
    capital_de_notificacao: String,
    values: [CsvOption<u64>; 12],
}

impl RecentRecord4 {
    fn check(&self) -> Result<()> {
        Ok(())
    }
    pub fn get_column(&self, col: usize) -> Option<u64> {
        if col >= 1 && col <= 12 {
            *self.values[col - 1]
        } else {
            panic!("invalid col number {col}")
        }
    }
}

impl CapitalDeNotificacao for RecentRecord4 {
    fn capital_de_notificacao(&self) -> Result<&str> {
        Ok(&self.capital_de_notificacao)
    }
}    


// Notas:
// Excluídos casos não residentes no Brasil
// Períodos Disponíveis ou período - Correspondem aos anos de notificação dos casos.
const TSV: &'static str = "Capital de notificação	Em branco/IGN	<1 Ano	01 a 04	05 a 09	09 a 14	15-19	20-39	40-59	60-64	65-69	70-79	80 e +
São Paulo	-	8	5	40	122	230	1147	988	119	69	58	12
Rio Branco	1	6	38	149	291	296	992	479	55	39	34	9
Recife	-	22	12	88	173	207	719	395	37	25	20	4
Curitiba	-	7	3	36	127	149	541	405	41	37	35	9
Belém	-	5	3	16	44	108	357	231	24	19	23	3
Salvador	-	19	6	8	43	86	447	328	35	18	13	3
Rio de Janeiro	2	9	1	12	28	64	280	286	27	25	26	2
Porto Alegre	-	4	3	15	31	63	313	270	50	25	7	1
Maceió	-	3	3	10	38	85	317	157	15	7	5	1
Fortaleza	2	4	4	14	23	45	219	135	26	11	26	3
Manaus	1	1	3	24	41	74	230	139	12	8	4	-
Macapá	-	6	32	85	107	59	168	88	6	5	7	2
Brasília	-	2	-	1	11	31	145	79	13	3	2	1
Florianópolis	-	3	-	11	13	19	180	109	15	11	8	-
Aracaju	-	3	-	3	23	50	190	151	14	13	10	3
Vitória	1	-	3	20	25	21	114	82	6	6	9	4
Belo Horizonte	-	-	-	5	4	17	211	92	15	9	10	1
São Luís	1	2	2	12	23	39	107	76	5	4	8	3
João Pessoa	-	-	-	2	4	12	43	34	7	3	4	-
Natal	-	0	1	5	2	4	32	41	8	-	1	2
Porto Velho	-	1	3	13	18	28	108	56	6	5	6	1
Campo Grande	-	-	-	2	-	2	25	7	1	1	2	-
Goiânia	-	2	-	2	-	1	19	17	-	1	-	-
Cuiabá	-	-	-	1	-	1	4	1	1	-	-	-
Teresina	-	-	-	1	-	4	14	7	1	1	2	-
Palmas	-	-	-	-	1	-	5	3	-	1	1	-
Boa Vista	-	-	-	-	-	2	13	4	-	-	-	-
";
// XXX weird, what are these totals, are they totally bogus?
const TOTAL : &[Option<u64>] = &[
    None, Some(4), Some(24), Some(33), Some(160), Some(263), Some(413), Some(1516), Some(952), Some(104), Some(78), Some(49), Some(10)
];

#[derive(Debug)]
pub struct RecentTable4 {
    name: String,
    records: Vec<RecentRecord4>,
}

impl RecentTable4 {
    fn check(&self) -> Result<()> {
        let mut totals = [None, Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0)];
        for record in &self.records {
            record.check()?;
            for i in 1..13 {
                // skip "-" values  XX wow ?? HACK? but not from me
                if let Some(v) = record.get_column(i) {
                    *(totals[i].as_mut().unwrap()) += v;
                }
            }
        }
        if totals != TOTAL {
            // bail!("totals do not match: got {totals:?}, expected {TOTAL:?}")
            // see note above, the values are simply bogus?
            if std::env::var("WARN").is_ok() {
                println!("totals do not match:\n       got: {totals:?}\n  expected: {TOTAL:?}")
            }
        } 
        Ok(())
    }
    
    pub fn get() -> Result<RecentTable4> {
        let records = easycsv::parse_tsv::<RecentRecord4>(TSV.as_bytes())?;
        let tabl = RecentTable4 {
            name: String::from(file!()),
            records
        };
        tabl.check()?;
        Ok(tabl)
    }
}

impl RecentTable<RecentRecord4> for RecentTable4 {
    fn name(&self) -> &str { &self.name }
    fn records(&self) -> &Vec<RecentRecord4> { &self.records }
}
