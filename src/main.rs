use std::env;
use std::fs::File;
use std::io::prelude::*;

fn read_u32(buf: &Vec<u8>, offset: usize) -> u32 {
    let a = buf[offset + 0] as u32;
    let b = buf[offset + 1] as u32;
    let c = buf[offset + 2] as u32;
    let d = buf[offset + 3] as u32;

    (a | (b << 8) | (c << 16) | (d << 24))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let file_name = &args[1];
        let mut file = File::open(file_name).expect("file not found");

        let mut content: Vec<u8> = Vec::new();
        let file_size = file
            .read_to_end(&mut content)
            .expect("Can't read file content");

        let mut snd_idx: u32 = 0;
        let mut offset: usize = 0;
        while offset < file_size {
            let riff: u32 = read_u32(&content, offset);
            let size = read_u32(&content, offset + 4) as usize;

            if riff == 0x46464952 {
                let wav_name = format!("{}{:03}.wav", "snd_", snd_idx);
                let mut wav_file = File::create(wav_name).expect("Can't create output file");
                wav_file
                    .write(&content[offset..(offset + size + 8)])
                    .expect("Can't write to output file");
                wav_file.sync_all().unwrap();

                println!("Unpacking wav #{} of size {}", snd_idx, size + 8);

                offset += size + 8;
                while content[offset] == 0 {
                    offset = offset + 1;
                }

                snd_idx = snd_idx + 1;
            } else {
                // quit
                offset = file_size
            }
        }
    }
}
