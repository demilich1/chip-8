use std::fs::File;
use std::io::Read;

pub struct Rom {
    raw: Vec<u8>,
}

impl Rom {
    pub fn load(name: &str) -> Self {
        let mut file = File::open(name).expect(&format!("Could not open ROM file at {:?}", name));
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).expect(&format!("Could not open ROM file at {:?}", name));
        Rom {
            raw: buf
        }
    }
}

impl Into<Vec<u8>> for Rom {
    fn into(self) -> Vec<u8> {
        self.raw
    }
}
