use base64::{engine::general_purpose, Engine as BASE64, Engine};
use clap::{Arg, Parser, ValueEnum};
use colored::*;
use rand::Rng;

#[derive(Copy , Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum KeyType{
    Aes128,
    Aes192,
    Aes256,
    ChaCha20,
    Custom,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Format{
    Hex,
    Base64,
    RawBytes,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "CLI Encryption Key Generator", long_about = None)]
struct Args {
    #[arg(short, long,value_enum, default_value_t = KeyType::Aes256)]
    key_type: KeyType,

    #[arg(short, long)]
    length: Option<usize>,

    #[arg(short, long, value_enum, default_value_t = Format::Hex)]
    format: Format,

    #[arg(short = 'n' , long, default_value_t = 1)]
    count: usize,
}

fn generate_randomm_bytes(length: usize) -> Vec<u8> {
    let mut key = vec![0u8; length];
    rand::rng().fill_bytes(&mut key);
    key

}

fn format_key(bytes: &[u8], format: Format) -> String {
    match format {
        Format::Hex => hex::encode(bytes),
        Format::Base64 => general_purpose::STANDARD.encode(bytes),
        Format::RawBytes => format!("{:02x?}", bytes)

    }
}
//use base64::{engine::general_purpose, Engine};
// Ok(custom_len.unwrap_or(32))

fn get_key_length(key_type: KeyType, custom_len: Option<usize>) -> Result<usize, String> {
    match key_type {
        KeyType::Aes128 => Ok(16),
        KeyType::Aes192 => Ok(24),
        KeyType::Aes256  | KeyType::ChaCha20 => Ok(32),
        KeyType::Custom => custom_len.ok_or_else(|| {
            "Error: --length (-l) is required when using custom key type".to_string()
        }),
    }
}

fn main() {
    let args = Args::parse();

    let byte_length = match get_key_length(args.key_type, args.length) {
        Ok(len) => len,
        Err(e) => {
            eprintln!("{}", e.red().bold());
            std::process::exit(1);
        }
    };
    println!("{}", "\n GSRC Key Generator".bright_green().bold());
    println!("{}", "===============================".ansi_color(244));
    println!(
        "{} {:?} | bytes ({} bits | {} {:?}",
        "Algorithm:".bold().magenta(),
        args.key_type,
        "length:".bold().magenta(),
        byte_length,
        byte_length * 8,
        "Format:".bold().magenta(),
        args.format
    );

}
