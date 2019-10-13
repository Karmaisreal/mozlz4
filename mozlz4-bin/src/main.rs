use clap::{crate_version, App, Arg};
use mozlz4::*;

use std::{fs::File, io::{Read, Write}};


fn main() -> Result<(), String> {
    let matches = App::new("mozlz4")
        .version(crate_version!())
        .author("Justin Wong")
        .about("Decompress and compress mozlz4 files. Overwrites existing files.")
        .arg(
            Arg::with_name("decompress")
                .help("decompress mozlz4 (default)")
                .short("x")
                .long("extract")
                .conflicts_with("compress")
                .display_order(1),
        )
        .arg(
            Arg::with_name("compress")
                .help("compress to mozlz4")
                .short("z")
                .long("compress"),
        )
        .arg(
            Arg::with_name("input")
                .help("input file, - for stdin")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .help("output file, - for stdout (default)")
                .index(2),
        )
        .get_matches();

    let ifilename = matches.value_of("input").unwrap();
    let ofilename = matches.value_of("output").unwrap_or("-");
    let do_compress = matches.is_present("compress");

    let ibuffer = read_to_buffer(ifilename)?;

    let obuffer = if !do_compress {
        decompress(ibuffer).or(Err("Failed to decompress"))
    } else {
        compress(ibuffer).or(Err("Failed to compress"))
    }?;
    write_to_file(obuffer, ofilename)?;

    Ok(())
}

fn write_to_file(obuffer: Vec<u8>, ofilename: &str) -> Result<(), String> {
    let mut ofile: Box<dyn Write> = if ofilename == "-" {
        Box::new(std::io::stdout())
    } else {
        Box::new(
            File::create(ofilename).or_else(|_| Err(format!("Unable to create file {}", ofilename)))?,
        )
    };

    ofile
        .write_all(&obuffer[..])
        .or_else(|_| Err(format!("Unable to write to file {}", ofilename)))?;
    Ok(())
}

fn read_to_buffer(ifilename: &str) -> Result<Vec<u8>, String> {
    let mut ifile: Box<dyn Read> = if ifilename == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(ifilename).or_else(|_| Err(format!("Unable to open file {}", ifilename)))?)
    };

    let mut ibuffer: Vec<u8> = Vec::new();
    let bytes_read = ifile
        .read_to_end(&mut ibuffer)
        .or_else(|_| Err(format!("Unable to read file {}", ifilename)))?;
    assert_eq!(ibuffer.len(), bytes_read);
    Ok(ibuffer)
}