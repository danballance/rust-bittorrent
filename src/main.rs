mod bencode;
pub mod torrent;

use crate::bencode::decoder::Decoder;
use std::env;
use torrent::reader::TorrentReader;
#[derive(Debug)]
enum Command {
    Decode,
    Info,
}

impl Command {
    fn from_cli(arg: &str) -> Result<Self, String> {
        match arg {
            "decode" => Ok(Command::Decode),
            "info" => Ok(Command::Info),
            _ => Err(format!("unknown command: {arg}")),
        }
    }
}

// Usage: your_program.sh decode "<encoded_value>"
fn main() {
    let usage = "Usage: your_program.sh decode \"<encoded_value>\"";
    let args: Vec<String> = env::args().collect();
    let Some(arg1) = args.get(1) else {
        eprintln!("{usage}");
        return;
    };
    match Command::from_cli(arg1) {
        Ok(Command::Decode) => {
            let Some(encoded_value) = args.get(2) else {
                eprintln!("{usage}");
                return;
            };
            let decoder = Decoder::new();
            match decoder.decode(encoded_value) {
                Ok(value) => println!("{}", value),
                Err(err) => eprintln!("Decode error: {}", err),
            }
        }
        Ok(Command::Info) => {
            let Some(torrent_file_path) = args.get(2) else {
                eprintln!("{usage}");
                return;
            };
            let file_contents = std::fs::read_to_string(torrent_file_path).unwrap();
            let decoder = Decoder::new();
            let torrent_reader: TorrentReader = TorrentReader::new(decoder);
            let Ok(torrent) = torrent_reader.read(file_contents) else {
                eprintln!("Unable to read torrent file: {torrent_file_path}");
                return;
            };
            println!("Tracker URL: {}", torrent.url);
            println!("Length: {}", torrent.length);
        }
        Err(err) => {
            eprintln!("Unknown command: {}", err);
            eprintln!("{usage}");
        }
    }
}
