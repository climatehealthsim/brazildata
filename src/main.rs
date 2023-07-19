mod easycsv;
mod recent1;
mod recent8;

use anyhow::Result;
use crate::{easycsv::optionfmt, recent1::RecentTable1, recent8::RecentTable8};


fn main() -> Result<()> {

    {
        let rt1 = RecentTable1::get();
        dbg!(rt1)?;
    }
        
    {
        let rt8 = RecentTable8::get()?;
        let records = &rt8.records;
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
