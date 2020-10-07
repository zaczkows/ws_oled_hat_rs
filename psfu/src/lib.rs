use std::io::Read;

pub struct Font {
    data: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub enum Error {
    Unknown,
    FileNotFound,
    FileIo,
    InvalidFontFormat,
}

impl std::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        match e {
            _ => Error::FileIo,
        }
    }
}

impl Font {
    pub fn new_from_str(path: &str) -> Result<Font, Error> {
        Font::new(&std::path::Path::new(path))
    }

    pub fn new(path: &std::path::Path) -> Result<Font, Error> {
        if !path.exists() && !path.is_file() {
            return Err(Error::FileNotFound);
        }

        let filename = path.file_name();
        if filename.is_none() {
            return Err(Error::Unknown);
        }

        let mut data = std::fs::read(path)?;
        let filename = filename.unwrap();
        if filename.to_str().unwrap().ends_with(".gz") {
            // gunzip first
            let mut gzd = flate2::read::GzDecoder::new(&data[..]);
            let mut decoded_data = Vec::new();
            gzd.read_to_end(&mut decoded_data)?;
            data = decoded_data;
        }

        Font::parse_font_data(&data)
    }

    fn parse_font_data(raw_data: &[u8]) -> Result<Font, Error> {
        if raw_data.is_empty() {
            return Err(Error::InvalidFontFormat);
        }

        let height;
        let width;
        let byte_width;
        let number: u32;
        let mut data = raw_data.iter();
        let mode = match *data.next().unwrap() {
            0x36 => 1,
            0x72 => 2,
            _ => return Err(Error::InvalidFontFormat),
        };
        if mode == 1 {
            if raw_data.len() < 4 {
                return Err(Error::InvalidFontFormat);
            }
            if *data.next().unwrap() != 0x04 {
                return Err(Error::InvalidFontFormat);
            }
            number = match *data.next().unwrap() {
                0 => 256,
                1 => 512,
                2 => 256,
                3 => 512,
                _ => return Err(Error::InvalidFontFormat),
            };
            height = *data.next().unwrap();
            width = 8;
            byte_width = 1;
        } else {
            if raw_data.len() < 32 {
                return Err(Error::InvalidFontFormat);
            }
            if *data.next().unwrap() != 0xb5
                || *data.next().unwrap() != 0x4a
                || *data.next().unwrap() != 0x86
            {
                return Err(Error::InvalidFontFormat);
            }
            let version = get_data(&mut data, 4);
            if version == [0, 0, 0, 0] {
                return Err(Error::InvalidFontFormat);
            }
            let offset = as_le_u32(&mut data);
            if offset != 0x20 {
                return Err(Error::InvalidFontFormat);
            }
            let _flags = as_le_u32(&mut data);
            number = *data.next().unwrap() as u32 + *data.next().unwrap() as u32 * 256;
            let no_chars = as_le_u16(&mut data);
            if no_chars as u32 > 64 * 1024 {
                return Err(Error::InvalidFontFormat);
            }
            let _sizeof_char = as_le_u32(&mut data);
            height = *data.next().unwrap();
            let _heigh_char_in_lines = get_data(&mut data, 4);
            width = *data.next().unwrap();
            if get_data(&mut data, 3) != [0, 0, 0] {
                return Err(Error::InvalidFontFormat);
            }
            byte_width = (width + 7) / 8;
        }

        println!(
            "Parsing psf mode {} font file, with {} characters {} x {} (width x height)",
            &mode, &number, &width, &height
        );

        let mut vvv: Vec<Vec<u8>> = Vec::with_capacity(number as usize);
        for n in 0..number {
            vvv.push(Vec::with_capacity(height as usize * byte_width as usize));
            for _ in 0..height {
                for _ in 0..byte_width {
                    vvv[n as usize].push(*data.next().unwrap());
                }
            }
            assert!(
                vvv[n as usize].len()
                    == height as usize * width as usize / (byte_width as usize * 8)
            );
        }

        Ok(Font { data: vvv })
    }
}

fn as_le_u32(data: &mut std::slice::Iter<u8>) -> u32 {
    (*data.next().unwrap() as u32) << 24
        | (*data.next().unwrap() as u32) << 16
        | (*data.next().unwrap() as u32) << 8
        | (*data.next().unwrap() as u32)
}

fn as_le_u16(data: &mut std::slice::Iter<u8>) -> u16 {
    (*data.next().unwrap() as u16) << 8 | (*data.next().unwrap() as u16)
}

fn get_data(data: &mut std::slice::Iter<u8>, count: usize) -> Vec<u8> {
    let mut v = Vec::with_capacity(count);
    for _ in 0..count {
        v.push(*data.next().unwrap());
    }
    v
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
