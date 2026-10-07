use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

use zip::ZipArchive;

pub struct Jar {
    path: PathBuf,
    pub strings: Vec<String>,
}

impl Jar {
    pub fn new(path: PathBuf) -> io::Result<Jar> {
        let file = File::open(&path)?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let mut strings = Vec::new();

        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            if entry.is_file() {
                let mut data = Vec::new();
                entry.read_to_end(&mut data)?;

                strings.extend(get_strings(&data));
            }
        }

        Ok(Self { path, strings })
    }
}

pub fn get_strings(data: &[u8]) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = Vec::new();

    for &byte in data {
        if byte.is_ascii_alphabetic() || byte == b' ' {
            current.push(byte);
        } else {
            if current.len() >= 4 {
                if let Ok(s) = String::from_utf8(current.clone()) {
                    result.push(s);
                }
            }

            current.clear();
        }
    }

    if current.len() >= 4 {
        if let Ok(s) = String::from_utf8(current) {
            result.push(s);
        }
    }

    result
}