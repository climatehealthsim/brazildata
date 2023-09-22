use osmpbfreader::OsmObj;
use osmpbfreader::reader::OsmPbfReader;
use anyhow::{Result, anyhow};
use std::{fs::File, ffi::OsString};
// use std::io::BufReader;
use smartstring::{SmartString, SmartStringMode};
use osm_boundaries_utils;

struct Opt {
    path: OsString,
}

fn parse_args() -> Result<Opt> {
    use lexopt::prelude::*;
    let mut path: Option<OsString> = None;
    let mut parser = lexopt::Parser::from_env();
    loop {
        match parser.next()? {
            Some(Value(v)) => path = Some(v),
            Some(arg) => Err(arg.unexpected())?,
            None => break
        }
    }
    Ok(Opt {
        path: path.ok_or_else(|| anyhow!("Missing path argument"))?
    })
}


type Tag<'t, S> = (&'t SmartString<S>,
                   &'t SmartString<S>);

    
fn is_proper_tag<S: SmartStringMode>(tag: &Tag<S>) -> bool {
    match tag.0.as_str() { "created_by" | "source" => false, _  => true }
}

fn has_proper_tags(n: &OsmObj) -> bool {
    for tag in n.tags().iter() {
        if is_proper_tag(&tag) {
            // println!("  {:?} = {:?}", tag.0, tag.1);
            return true
        }
    }
    false
}

fn print_node(n: &OsmObj) {
    //println!("Node {:?}", n);
    println!("Node {:?}", n.id().inner_id());
    for tag in n.tags().iter() {
        if is_proper_tag(&tag) {
            println!("  {:?} = {:?}", tag.0, tag.1);
        }
    }
}

fn main() -> Result<()> {
    let opt = parse_args()?;
    let f = File::open(opt.path)?;
    let mut pbf = OsmPbfReader::new(f);
    // ^ looks like using BufReader would slow it down if anything
    let mut totalcount = 0;
    let mut selectedcount = 0;
    let countdown_from = 1000000;
    let mut countdown = countdown_from;
    if true {
        pbf.par_iter().filter(
            |x| match x {
                Ok(x) => x.is_node(),
                Err(_) => true
            }).for_each(
            |x| {
                match x {
                    Ok(x) => {
                        if has_proper_tags(&x) {
                            print_node(&x);
                        }
                    },
                    Err(e) => {
                        println!("Err: {e}");
                    }
                }
            });
    } else {
        let items = pbf.get_objs_and_deps(|n| {
            totalcount += 1;
            countdown -= 1;
            if countdown == 0 {
                countdown = countdown_from;
                println!("processed {totalcount} items (selected {selectedcount})");
            }
            if n.is_way() && n.tags().len() >= 2 && has_proper_tags(&n) {
                selectedcount += 1;
                // true
                print_node(&n);
                false
            } else {
                false
            }
        })?;
        println!("=> processed {totalcount} items (selected {selectedcount})");
        for item in items {
            if has_proper_tags(&item.1) {
                print_node(&item.1);
            }
        }
    }
    Ok(())
}
