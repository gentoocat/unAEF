use std::fs;
use std::path::PathBuf;
use aes::Aes128;
use cbc::cipher::{KeyIvInit, BlockDecryptMut, block_padding::Pkcs7};
use cbc::Decryptor;
use clap::Parser;
use thiserror::Error;

type Aes128CbcDecryptor = Decryptor<Aes128>;

const KEY: &[u8; 16] = &[
    191, 14, 15, 183, 231, 28, 35, 232,
    38, 142, 203, 89, 18, 139, 91, 155,
];
const IV: &[u8; 16] = &[
    207, 202, 227, 230, 145, 185, 59, 226,
    212, 211, 219, 67, 128, 4, 160, 36,
];


#[derive(Debug, Error)]
enum AppError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("crypto error: this file maybe not AEF or my program was patched")]
    Crypto, 
}

fn decrypt_aef(in_path: &PathBuf, out_path: &PathBuf) -> Result<(), AppError> {
    println!("decrypting {} to {}", in_path.display(), out_path.display());

    let mut buffer = fs::read(in_path)?;

    if buffer.is_empty() {
        return Err(AppError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "input file is empty",
        )));
    }


    let cipher = Aes128CbcDecryptor::new(KEY.into(), IV.into());


    let decrypted_data = cipher
        .decrypt_padded_mut::<Pkcs7>(&mut buffer)
        .map_err(|_e| AppError::Crypto)?; 

    fs::write(out_path, decrypted_data)?;

    Ok(())
}


#[derive(Parser, Debug)]
#[command(author, version, about = "AEF file decryptor written in Rust", long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    match decrypt_aef(&cli.input, &cli.output) {
        Ok(()) => {
            println!("file successfully unAEFed");
            println!("saved to: {}", cli.output.display());
        }
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1); 
        }
    }
}
