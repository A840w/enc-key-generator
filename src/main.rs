use base64::display;
use base64::{engine::general_purpose, Engine as BASE64, };
use clap::builder::Str;
use clap::builder::styling::AnsiColor::Yellow;
use clap::{Parser, ValueEnum};
use colored::Styles::Underline;
use colored::*;
use rand::Rng;
use std::io::Write;
use serde::{Serialize};
use std::fs::File;
use std::path::{self, PathBuf};

#[derive(Copy , Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum KeyType{
    Aes128,
    Aes192,
    Aes256,
    ChaCha20,
    Custom,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
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

    #[arg(short, long, value_name = "FILE")]
    save: Option<PathBuf>
}

#[derive(Serialize)]
struct JsonRecord{
    key_type: KeyType,
    byte_length:usize,
    bit_length: usize,
    format: Format,
    key: Vec<String>,
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

fn save_keys_to_file(
    path: &PathBuf,
    keys: &[String],
    key_type: KeyType,
    byte_length: usize,
    format: Format
) -> Result<(),std::io::Error> {
    let mut file = File::create(path)?;

    let is_json =  path
        .extension()
        .map_or(false,|ext| ext.eq_ignore_ascii_case("json"));
        //.unwrap_or(false);

    if is_json {
        let record = JsonRecord{
            key_type,
            byte_length,
            bit_length: byte_length * 8,
            format,
            key: keys.to_vec(),
        };
        let json_content =  serde_json::to_string_pretty(&record)?;
        file.write_all(json_content.as_bytes())?;
    } else {
        for key in keys {
            writeln!(file, "{}", key)?;
        }
    }
    Ok(())
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
    println!("{}", "============================================".ansi_color(244));
    println!(
        "{} {:?} | {} {} bytes ({} bits) | {} {:?}",
        "Algorithm:".bold().magenta(),
        args.key_type,
        "Length:".bold().magenta(),
        byte_length,
        byte_length * 8,
        "Format:".bold().magenta(),
        args.format
    );
    println!("{}", "============================================".ansi_color(244));
    let mut generated_keys = Vec::with_capacity(args.count);
    for i in 1..= args.count{
        let raw_bytes =  generate_randomm_bytes(byte_length);
        let formatted_key = format_key(&raw_bytes, args.format);

        if args.count  > 1 {
            print!("{}", format!("[{:02}]", i ).dimmed());
        }
        println!(" {}", formatted_key.bright_green().bold());
        generated_keys.push(formatted_key);
    }
if let Some(ref path) = args.save {
        match save_keys_to_file(&path, &generated_keys, args.key_type, byte_length, args.format) {
            Ok(_) => {
                println!(
                    "\n{} Saved output to {}",
                    "✔".bright_green().bold(),
                    path.display().to_string().underline().yellow()
                );
            }
            Err(e) => {
                eprintln!(
                    "\n{} Failed to save file: {}",
                    "✖".bright_red().bold(),
                    e.to_string().red()
                );
                std::process::exit(1);
            }
        }
    }
    println!()
}
