use anyhow::{Result, bail};
use serde::Deserialize;

use crate::{easycsv::{CsvOption, self}, util::{CapitalDeNotificacao, RecentTable, remove_coordinates}};

#[derive(Debug, Deserialize)]
pub struct RecentRecord6 {
    capital_de_notificacao: String,
    pub ign_branco: CsvOption<u64>,
    pub urbana: CsvOption<u64>,
    pub rural: CsvOption<u64>,
    pub periurbana: CsvOption<u64>,
    pub total: u64, // This is the sum of all fields of all records below this one and this one, or so (the top matches, the beginning doesn't; aha, it appears that rows were sorted at some point.)
}

impl RecentRecord6 {
    fn check(&self) -> Result<()> {
        Ok(())
    }
    pub fn get_column(&self, col: usize) -> Option<u64> {
        let total = CsvOption(Some(self.total));
        match col {
            1 => &self.ign_branco,
            2 => &self.urbana,
            3 => &self.rural,
            4 => &self.periurbana,
            5 => &total,
            _ => panic!("invalid col number {col}")
        }.into()
    }
}

impl CapitalDeNotificacao for RecentRecord6 {
    fn capital_de_notificacao(&self) -> Result<&str> {
        remove_coordinates(&self.capital_de_notificacao)
    }
}    

// Notas:
// Excluídos casos não residentes no Brasil.
// Períodos Disponíveis ou período - Correspondem aos anos de notificação dos casos.
// Excluídas duplicidades de dados de 2007.
// Para tabular dados epidemiológicos de um determinado ano selecione na linha a variável de interesse, na Coluna Ano dos 1ºs sintomas; em Períodos Disponíveis assinale o ano inicial da série e todos os posteriores até o ano atual (p/ incluir casos notificados com atraso) e em Seleções Disponíveis assinale os anos dos 1ºs sintomas (ex: nº de casos com início de sintomas 2007: selecione na linha UF de residência, na Coluna Ano de 1ºs sintomas, em Períodos disponíveis 2007 até o ano atual e em Seleções assinale Ano de 1ºs sintomas 2007).
// Dados de 2008 atualizados em 26/03/2010.
// Dados de 2009 atualizados em 01/08/2011.
// Dados de 2010 atualizados em 18/12/2012.
// Dados de 2011 atualizados em 30/01/2014.
// Dados de 2012 atualizados em 22/06/2015.
// Dados de 2013 atualizados em 02/10/2015, sujeitos à revisão.
// Dados de 2014 atualizados em 02/10/2015, sujeitos à revisão.
// Dados de 2015 atualizados em 02/10/2015, sujeitos à revisão.
const TSV: &'static str = "Capital de notificação	Ign/Branco	Urbana	Rural	Periurbana	
355030 São Paulo	81	1984	11	18	12859
120040 Rio Branco	38	1559	271	42	2094
261160 Recife	61	1173	26	8	1910
410690 Curitiba	23	1054	18	8	1268
292740 Salvador	31	736	7	1	1103
431490 Porto Alegre	24	550	27	18	775
150140 Belém	8	551	19	2	619
330455 Rio de Janeiro	24	546	2	5	580
270430 Maceió	3	451	36	5	577
160030 Macapá	-	429	46	-	495
130260 Manaus	9	410	19	3	475
280030 Aracaju	11	340	31	22	441
230440 Fortaleza	12	314	55	-	404
420540 Florianópolis	43	232	24	11	381
320530 Vitória	4	223	9	3	310
211130 São Luís	10	168	43	15	239
110020 Porto Velho	4	123	107	2	236
530010 Brasília	10	158	30	10	236
310620 Belo Horizonte	13	180	6	1	208
240810 Natal	1	76	9	1	200
250750 João Pessoa	1	82	3	-	87
520870 Goiânia	1	34	5	-	86
500270 Campo Grande	-	30	2	-	40
221100 Teresina	1	18	9	1	32
140010 Boa Vista	-	16	2	-	29
172100 Palmas	-	8	2	-	18
510340 Cuiabá	-	4	2	-	10
";
const TOTAL: &[Option<u64>] = &[None, Some(413), Some(11449), Some(821), Some(176)];

#[derive(Debug)]
pub struct RecentTable6 {
    name: String,
    records: Vec<RecentRecord6>,
}
    
impl RecentTable6 {
    fn check(&self) -> Result<()> {
        let mut totals = [None, Some(0), Some(0), Some(0), Some(0)];
        for record in &self.records {
            record.check()?;
            for i in 1..5 {
                // HACK? again like in recent{4,5}
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
    
    pub fn get() -> Result<RecentTable6> {
        let records = easycsv::parse_tsv::<RecentRecord6>(TSV.as_bytes())?;
        let tabl = RecentTable6 {
            name: String::from(file!()),
            records
        };
        tabl.check()?;
        Ok(tabl)
    }
}

impl RecentTable<RecentRecord6> for RecentTable6 {
    fn name(&self) -> &str { &self.name }
    fn records(&self) -> &Vec<RecentRecord6> { &self.records }
}
