use std::{fs::File, io::Read};

fn read_string_fixed(file: &mut File, length: usize) -> String {
    let mut buffer = vec![0; length];  
    file.read(&mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

fn read_string(file: &mut File) -> String {
    // Get length of string
    let mut buffer = [0u8; 4];
    file.read(&mut buffer).unwrap();
    let length = u32::from_le_bytes(buffer);

    // Return string
    read_string_fixed(file, length as usize)
}

fn read_u8(file: &mut File) -> u8 {
    let mut buffer = [0u8; 1];
    file.read(&mut buffer).unwrap();
    u8::from_le_bytes(buffer)
}
fn read_u32(file: &mut File) -> u32 {
    let mut buffer = [0u8; 4];
    file.read(&mut buffer).unwrap();
    u32::from_le_bytes(buffer)
}
fn read_i32(file: &mut File) -> i32 {
    let mut buffer = [0u8; 4];
    file.read(&mut buffer).unwrap();
    i32::from_le_bytes(buffer)
}

fn read_sequence_1(file: &mut File) {
    let item_count = read_u32(file);
    let loop_point = read_i32(file);
    let release_point = read_i32(file);
    let setting = read_u32(file);
    println!("\tnew sequence:");
    println!("\t\tItem Count: {item_count}");
    println!("\t\tLoop Point: {loop_point}");
    println!("\t\tRelease Point: {release_point}");
    println!("\t\tSetting : {setting}");

    // Get sequence data
    print!("\t\tItems:\n\t\t\t");
    for j in 0..item_count {
        let item = read_u8(file);
        print!("{item}, ");
    }
    print!("\n");
}

fn main() {
    // Open a debug file
    let mut file = File::open("D:/Library/Desktop/Strings 96.fti").unwrap();

    // Read some metadata
    let file_magic = read_string_fixed(&mut file, 6);
    let inst_type = read_u8(&mut file);
    let instrument_name = read_string(&mut file);
    let n_sequences = read_u8(&mut file);

    println!("File Magic: {file_magic}");
    println!("Inst Type: {inst_type}");
    println!("instrument Name: {instrument_name}");
    println!("Number of Sequences: {n_sequences}");

    println!("SEQUENCES:");
    // Parse sequences
    for i in 0..n_sequences {

        // Get sequence metadata
        let sequence_type = read_u8(&mut file);
        println!("\t\tSequence Type: {sequence_type}");

        match sequence_type {
            0 => continue,
            1 => read_sequence_1(&mut file),
            _ => todo!(),
        }
        print!("\n");
    }

    // Read number of sequences
}
