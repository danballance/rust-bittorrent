use crate::bencode::decoder::Decoder;
use crate::torrent::types::Torrent;

pub struct TorrentReader {
    decoder: Decoder,
}

impl TorrentReader {
    pub(crate) fn new(decoder: Decoder) -> Self {
        Self { decoder }
    }

    pub fn read(self, data: String) -> Result<Torrent, String> {
        Ok(Torrent {
            url: String::new(),
            length: 0,
        })
    }
}
