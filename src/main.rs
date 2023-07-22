mod easycsv;
mod util;
mod staticdatabase;
mod coordinates;
mod recent1;
mod recent2;
mod recent3;
mod recent4;
mod recent5;
mod recent6;
mod recent7;
mod recent8;
mod casos_e_pluviosidade;

use anyhow::Result;
use crate::casos_e_pluviosidade::casos_por_regiao::CasosPorRegiaoTable;
use crate::coordinates::parse_coordinates;
use crate::recent1::RecentTable1;
use crate::recent2::RecentTable2;
use crate::recent3::RecentTable3;
use crate::recent4::RecentTable4;
use crate::recent5::RecentTable5;
use crate::recent6::RecentTable6;
use crate::recent7::RecentTable7;
use crate::recent8::RecentTable8;
use crate::staticdatabase::{StaticDatabase, CityName};
use crate::easycsv::optionfmt;
use crate::util::{capitals_from_table, CapitalDeNotificacao, RecentTable};


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

    let rt7 = RecentTable7::get()?;
    if false {
        dbg!(&rt7);
    }
    let c7 = capitals_from_table(&rt7)?;
    
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
    assert_eq!(&c1, &c7); // println!("odd: c1 vs c7 = {:?}", crate::util::compare_sets(&c1, &c7));
    assert_eq!(&c1, &c8);

    let cpr = CasosPorRegiaoTable::get()?;
    dbg!(cpr);

    let sdb = StaticDatabase::get()?;
    sdb.check()?;
    for cityname_str in c1 {
        let cityname = CityName(cityname_str);
        if let Some(city) = sdb.get_city(cityname) {
            // dbg!(city);
            let is_capital =
                if let Some(state) = sdb.city_opt_capital_of_state(city) {
                    println!("City is state capital of: {:?} {:?}",
                             city.name, state.name);
                    true
                } else {
                    println!("City is not a state capital: {:?}",
                             city.name);
                    false
                };
            assert_eq!(is_capital, sdb.city_is_notification_capital(&city)?);
        } else {
            println!("WARNING: unknown {cityname:?}");
        }
    }

    for (_, city) in sdb.cities {
        if let Some(coord) = city.coordinates {
            let c = parse_coordinates(coord, 0.)?;
            println!("City {:?} is at {},{} ({c:?})", city.name.0,
                     c.latitude_degrees(), c.longitude_degrees());
        }
    }
    
    println!("===OK===");
    Ok(())
}
