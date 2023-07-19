mod easycsv;
mod recent1;
mod recent2;
mod recent3;
mod recent4;
mod recent5;
mod recent6;
mod recent8;
mod util;

use anyhow::Result;
use recent2::RecentTable2;
use util::capitals_from_table;
use crate::recent4::RecentTable4;
use crate::recent5::RecentTable5;
use crate::recent6::RecentTable6;
use crate::{easycsv::optionfmt, recent1::RecentTable1, recent8::RecentTable8, recent3::RecentTable3};
use crate::util::{CapitalDeNotificacao, RecentTable};


fn main() -> Result<()> {

    let rt1 = RecentTable1::get()?;
    if false {
        dbg!(&rt1);
    }
    let c1 = capitals_from_table(&rt1)?;
        
    let rt2 = RecentTable2::get()?;
    if false {
        dbg!(&rt2);
    }
    let c2 = capitals_from_table(&rt2)?;

    let rt3 = RecentTable3::get()?;
    if false {
        dbg!(&rt3);
    }
    let c3 = capitals_from_table(&rt3)?;

    let rt4 = RecentTable4::get()?;
    if false {
        dbg!(&rt4);
    }
    let c4 = capitals_from_table(&rt4)?;
    
    let rt5 = RecentTable5::get()?;
    if false {
        dbg!(&rt5);
    }
    let c5 = capitals_from_table(&rt5)?;

    let rt6 = RecentTable6::get()?;
    if false {
        dbg!(&rt6);
    }
    let c6 = capitals_from_table(&rt6)?;
    
    let rt8 = RecentTable8::get()?;
    if false {
        let records = rt8.records();
        println!("{records:?}");

        for record in records {
            let cap = record.capital_de_notificacao()?;
            let ill = optionfmt(*record.illiterate);
            let ed = optionfmt(*record.incomplete_medium_education);
            println!("{cap} {ill} {ed}");
        }
    }
    let c8 = capitals_from_table(&rt8)?;

    assert_eq!(&c1, &c2);
    assert_eq!(&c1, &c3); assert_eq!(&c2, &c3);
    assert_eq!(&c1, &c4); // println!("c4 is odd: c1 vs c4 = {:?}", compare_sets(&c1, &c4));
    assert_eq!(&c1, &c5); // println!("odd: c1 vs c5 = {:?}", crate::util::compare_sets(&c1, &c5));
    assert_eq!(&c1, &c6); // println!("odd: c1 vs c6 = {:?}", crate::util::compare_sets(&c1, &c6));
    
    assert_eq!(&c1, &c8);
    
    Ok(())
}
