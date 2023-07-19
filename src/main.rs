mod easycsv;

use std::str::FromStr;
use serde::Deserialize;
use anyhow::{Result, bail};
use crate::easycsv::{CsvOption, optionfmt};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct RecentRecord1 {
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

const RECENT_TABLES_1_TSV : &'static str = "capital de Notificaçao 	Ign/branco	domiciliar	trabalho	lazer	outro	total
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
const RECENT_TABLES_1_TOTAL : &[Option<u64>] = &[None, Some(5273), Some(7272), Some(2347), Some(673), Some(780), Some(3606)];

#[derive(Debug)]
pub struct RecentTable1 {
    records: Vec<RecentRecord1>,
}
    
impl RecentTable1 {
    fn new(records: Vec<RecentRecord1>) -> Result<RecentTable1> {
        let tabl = RecentTable1 { records };
        tabl.check()?;
        Ok(tabl)
    }
    fn check(&self) -> Result<()> {
        let mut totals = [None, Some(0), Some(0), Some(0), Some(0), Some(0), Some(0)];
        for record in &self.records {
            record.check()?;
            for i in 1..7 {
                *(totals[i].as_mut().unwrap()) += record.get_column(i);
            }
        }
        if totals != RECENT_TABLES_1_TOTAL {
            bail!("totals do not match: got {totals:?}, expected {RECENT_TABLES_1_TOTAL:?}")
        } 
        Ok(())
    }
}



// Notification capital	Ign/White	Illiterate	1st to 4th incomplete grade of FS	4th complete grade of FS	5th to 8th incomplete grade of FS	Complete elementary school	Incomplete high school	Complete higher education	Incomplete higher education	Complete higher education	Does not apply
const RECENT_TABLES_TSV : &'static str = "Capital de notificação	Ign/Branco	Analfabeto	1ª a 4ª série incompleta do EF	4ª série completa do EF	5ª a 8ª série incompleta do EF	Ensino fundamental completo	Ensino médio incompleto	Ensino médio completo	Educação superior incompleta	Educação superior completa	Não se aplica
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
#[allow(dead_code)]
struct RecentRecord {
    coordinates_and_capital_de_notificacao: String,
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

impl RecentRecord {
    fn capital_de_notificacao(&self) -> Result<&str> {
        let s : &str = &self.coordinates_and_capital_de_notificacao;
        if let Some((coord, cap)) = s.split_once(' ') {
            if let Err(e) = u64::from_str(coord) {
                bail!("expected coordinates in {s:?}, but: {e}")
            }
            Ok(cap)
        } else {
            bail!("expecting coordinates_capital field to have a space")
        }
    }
}


fn main() -> Result<()> {

    {
        let rt1 = RecentTable1::new(easycsv::parse_tsv::<RecentRecord1>(RECENT_TABLES_1_TSV.as_bytes())?)?;
        dbg!(rt1);
    }
        
    {
        let records = easycsv::parse_tsv::<RecentRecord>(RECENT_TABLES_TSV.as_bytes())?;
        println!("{records:?}");

        for record in records {
            let cap = record.capital_de_notificacao()?;
            let ill = optionfmt(*record.illiterate);
            let ed = optionfmt(*record.incomplete_medium_education);
            println!("{cap} {ill} {ed}");
        }
    }
    
    Ok(())
}
