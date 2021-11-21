use anyhow::{bail, Result};
use sqlite_starter_rust::{
    header::PageHeader, record::parse_record, schema::Schema, varint::parse_varint,
};
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<()> {
    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();
    validate(&args)?;

    // Read database file
    let mut file = File::open(&args[1])?;
    let mut database = Vec::new();
    file.read_to_end(&mut database)?;

    // Parse command and act accordingly
    let command = &args[2];
    match command.as_str() {
        ".dbinfo" => dbinfo(&mut database),
        ".tables" => tables(&mut database),
        _ => bail!("Invalid command: {}", command),
    }
}

fn validate(args: &Vec<String>) -> Result<()> {
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => Ok(()),
    }
}

fn dbinfo(database: &mut Vec<u8>) -> Result<()> {
    let schema = parse_db_schema(database)?;
    println!("number of tables: {}", schema.len());
    Ok(())
}

fn tables(database: &mut Vec<u8>) -> Result<()> {
    let tables = parse_db_schema(database)?
        .into_iter()
        .map(|schema| schema.name)
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", tables);
    Ok(())
}

fn parse_db_schema(database: &mut Vec<u8>) -> Result<Vec<Schema>> {
    // Parse page header from database
    let page_header = PageHeader::parse(&database[100..108])?;

    // Obtain all cell pointers
    let cell_pointers = database[108..]
        .chunks_exact(2)
        .take(page_header.number_of_cells.into())
        .map(|bytes| u16::from_be_bytes(bytes.try_into().unwrap()))
        .collect::<Vec<_>>();

    // Obtain all records from column 5
    cell_pointers
        .into_iter()
        .map(|cell_pointer| {
            let stream = &database[cell_pointer as usize..];
            let (_, offset) = parse_varint(stream);
            let (_rowid, read_bytes) = parse_varint(&stream[offset..]);
            parse_record(&stream[offset + read_bytes..], 5)
                .map(|record| Schema::parse(record).expect("Invalid record"))
        })
        .collect::<Result<Vec<_>>>()
}
