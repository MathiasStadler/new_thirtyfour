use std::error::Error;
use csv::Writer;
use std::fs::File;
use std::io::prelude::*;

fn main(){
    let _ =  example();
}

fn example() -> Result<(), Box<dyn Error>> {

    let field_one = "Hello";
    let mut wtr = Writer::from_writer(vec![]);
    wtr.write_field(field_one)?;
    wtr.write_field("b")?;
    wtr.write_field("c")?;
    wtr.write_record(None::<&[u8]>)?;
    wtr.write_field("x")?;
    wtr.write_field("y")?;
    wtr.write_field("z")?;
    wtr.write_record(None::<&[u8]>)?;

    //let data = String::from_utf8(wtr.into_inner()?)?;
    //assert_eq!(data, "a,b,c\nx,y,z\n");
    let mut file = File::create("foo.csv")?;
    file.write_all(&wtr.into_inner()?)?;
    Ok(())
}