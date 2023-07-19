mod easycsv;
mod recent1;
mod recent2;
mod recent8;
mod util;

use anyhow::Result;
use recent2::RecentTable2;
use crate::{easycsv::optionfmt, recent1::RecentTable1, recent8::RecentTable8};


fn main() -> Result<()> {

    if false {
        let tbl = RecentTable1::get()?;
        dbg!(tbl);
    }
        
    {
        let tbl = RecentTable2::get()?;
        dbg!(tbl);
    }
        
    if false {
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
