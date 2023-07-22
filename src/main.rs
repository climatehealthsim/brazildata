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
use crate::casos_e_pluviosidade::cidades_do_acre::CidadesDoAcreTable;
use crate::coordinates::parse_coordinates;
use crate::recent1::RecentTable1;
use crate::recent2::RecentTable2;
use crate::recent3::RecentTable3;
use crate::recent4::RecentTable4;
use crate::recent5::RecentTable5;
use crate::recent6::RecentTable6;
use crate::recent7::RecentTable7;
use crate::recent8::RecentTable8;
use crate::staticdatabase::{StaticDatabase, MunicipalityName, StateName};
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

    let _cpr = CasosPorRegiaoTable::get()?;
    // dbg!(_cpr);
    let _cda = CidadesDoAcreTable::get()?;
    dbg!(_cda);
    

    let sdb = StaticDatabase::get()?;
    sdb.check()?;
    for municipalityname_str in c1 {
        let municipalityname = MunicipalityName(municipalityname_str);
        if let Some(municipality) = sdb.get_municipality(municipalityname) {
            // dbg!(municipality);
            let is_capital =
                if let Some(state) = sdb.municipality_opt_capital_of_state(municipality) {
                    println!("Municipality is state capital of: {:?} {:?}",
                             municipality.name, state.name);
                    true
                } else {
                    println!("Municipality is not a state capital: {:?}",
                             municipality.name);
                    false
                };
            assert_eq!(is_capital, sdb.municipality_is_notification_capital(&municipality)?);
        } else {
            println!("WARNING: unknown {municipalityname:?}");
        }
    }

    for (_, municipality) in sdb.municipality_by_municipalityname {
        if let Some(coord) = municipality.coordinates {
            let c = parse_coordinates(coord, 0.)?;
            println!("Municipality {:?} is at {},{} ({c:?})", municipality.name.0,
                     c.latitude_degrees(), c.longitude_degrees());
        }
    }

    for (_, municipality) in sdb.municipalities_by_statename(StateName("Acre"))? {
        
    }
    
    println!("===OK===");
    Ok(())
}
