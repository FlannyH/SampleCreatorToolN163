use std::{fs::File, io::Write};

use hound::WavReader;

fn main() {
    // Define samples per loop
    const samples_per_loop: usize = 169;
    let skip_first_samples = 0;
    let sample_stride = 2;
    let output_sample_size = 64;
    let output_sample_pos = 0;
    let name_string = "Output File";

    // Write .fti header
    let mut file = File::create("output.fti").unwrap();
    file.write(b"FTI2.4").unwrap();
    file.write(&(5u8).to_le_bytes()).unwrap();
    file.write(&(name_string.len() as u32).to_le_bytes()).unwrap();
    file.write(&name_string.as_bytes()).unwrap();
    file.write(&5u8.to_le_bytes()).unwrap();
    file.write(&0u8.to_le_bytes()).unwrap();
    file.write(&0u8.to_le_bytes()).unwrap();
    file.write(&0u8.to_le_bytes()).unwrap();
    file.write(&0u8.to_le_bytes()).unwrap();
    file.write(&1u8.to_le_bytes()).unwrap();
    
    // Load the wave file into a buffer
    let mut reader = hound::WavReader::open("D:/Programming/GBA/music_engine_windows/tools/samples/e1m1/overdrive.wav").unwrap();
    
    let output_wave_count = (reader.duration() - skip_first_samples) as usize / (samples_per_loop * sample_stride);


    file.write(&(output_wave_count as i32).to_le_bytes()).unwrap();
    file.write(&(-1i32).to_le_bytes()).unwrap();
    file.write(&(-1i32).to_le_bytes()).unwrap();
    file.write(&0u32.to_le_bytes()).unwrap();
    for i in 0..output_wave_count {
        file.write(&(i as u8).to_le_bytes()).unwrap();
    }

    file.write(&(output_sample_size as i32).to_le_bytes()).unwrap();
    file.write(&(output_sample_pos as i32).to_le_bytes()).unwrap();
    file.write(&(output_wave_count as i32).to_le_bytes()).unwrap();

    for j in 0..output_wave_count {
        // Seek to part
        reader.seek((skip_first_samples as usize + j * sample_stride * samples_per_loop) as u32);
    
        // Get one buffer
        let chunk_loop = reader.samples::<i16>()
            .take(samples_per_loop)
            .map(|result| result.unwrap())
            .collect::<Vec<i16>>();
    
        for i in 0..output_sample_size {
            let index = i * samples_per_loop / output_sample_size;
            let value = chunk_loop[index] / 4096 + 8;
            file.write(&(value as u8).to_le_bytes());
            print!("{value} ");
        }
        print!("\n");
    }


}