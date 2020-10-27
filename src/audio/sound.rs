use rodio;
use std::{sync::Arc, io};
use std::convert::AsRef;

pub struct Sound (Arc<Vec<u8>>);

impl AsRef<[u8]> for Sound {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Sound {
    pub fn load(filename: &str) -> io::Result<Sound> {
        use std::fs::File;
        use std::io::Read;

        let mut buf = Vec::new();
        let mut file = File::open(filename)?;
        file.read_to_end(&mut buf)?;
        Ok(Sound(Arc::new(buf)))
    }
    pub fn cursor(self: &Self) -> io::Cursor<Sound> {
        io::Cursor::new(Sound(self.0.clone()))
    }
    pub fn decoder(self: &Self) -> rodio::Decoder<io::Cursor<Sound>> {
        rodio::Decoder::new(self.cursor()).unwrap()
    }
}