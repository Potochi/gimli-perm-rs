use gimli::permutation::naive::{GimliNaive, GimliState};
use std::fs;
use std::io::{Read, Result};
use gimli::constants::GIMLI_SIZE;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);

        return Ok(())
    }

    let mut input_file = fs::File::open(&args[1])?;
    let mut input_content = Vec::<u8>::new();

    input_file.read_to_end(&mut input_content)?;

    if (input_content.len() % (GIMLI_SIZE * std::mem::size_of::<u32>())) != 0 {
        eprintln!("Warning, file truncated");
    }

    let mut gimli = GimliState::from_arr([0u32; 12]);

    for chunk
        in input_content.chunks_exact(GIMLI_SIZE * std::mem::size_of::<u32>()) {

        // SAFETY: This uses chunks exact, so the slice should always be convertible to a
        // static array of the same size
        let chunk_fixed = chunk.chunks_exact(4).map(|x|
            u32::from_le_bytes(x.try_into().unwrap())).collect::<Vec<_>>();


        gimli = gimli ^ GimliState::from_arr(chunk_fixed.as_slice().try_into().unwrap());
    }

    println!("{:#?}", gimli.state);

    Ok(())
}