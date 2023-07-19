mod easycsv;
mod recent1;
mod recent2;
mod recent3;
mod recent8;
mod util;

use anyhow::Result;
use recent2::RecentTable2;
use crate::{easycsv::optionfmt, recent1::RecentTable1, recent8::RecentTable8, recent3::RecentTable3};




fn main() -> Result<()> {

    let rt1 = RecentTable1::get()?;
    if false {
        dbg!(rt1);
    }
        
    let rt2 = RecentTable2::get()?;
    if false {
        dbg!(rt2);
    }

    let rt3 = RecentTable3::get()?;
    if false {
        dbg!(rt3);
    }
        
    let rt8 = RecentTable8::get()?;
    if false {
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
