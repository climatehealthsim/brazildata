
use anyhow::Result;

use hdf5::file::File as HDF5File;
use ndarray::{Array, Dim};

fn main() -> Result<()> {

    let x = HDF5File::open("/home/chrisrust/tmp/ncei.noaa.gov/PERSIANN-CDR_v01r01_20230331_c20230707.nc")?;
    dbg!(&x);
    dbg!(x.name());
    dbg!(x.comment());
    dbg!(x.userblock());

    println!("---- Groups --------------------------------------------");
    for group in x.groups()? {
        dbg!(group);
    }
    // aha that's sub-groups. Groups in "/". Of which there are none.

    println!("---- Attributes --------------------------------------------");
    for name in x.attr_names()? {
        dbg!(&name);
        // let val = x.attr(&name)?;
        // dbg!((&name, val.comment()));
    }

    println!("---- Datasets --------------------------------------------");
    for ds in x.datasets()? {
        dbg!(ds.name());
        println!("---- Datasets Attributes ---------------------------------------");
        for name in ds.attr_names()? {
            dbg!(&name);
        }
        if ds.name() == "/precipitation" {
            dbg!(ds.ndim());
            dbg!(ds.shape());
            dbg!(ds.size());
            dbg!(ds.is_scalar());
            dbg!(ds.storage_size());
            // dbg!(ds.read_dyn()?);
            dbg!(ds.dtype()?);
            let a : Array<f32, Dim<[usize; 3]>> = ds.read()?;
            dbg!(a);

            dbg!();
            dbg!();
            dbg!();
        }
    }

    return Ok(());
}
