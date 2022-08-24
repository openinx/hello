use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};

pub struct FileIO {
    buf: Vec<u8>,
    r: BufReader<File>,
}

impl FileIO {
    /// Initialize a FileIO instance.
    pub fn new(path: PathBuf) -> Self {
        FileIO {
            buf: vec![0; 1024],
            r: BufReader::new(File::open(path).expect("open failed")),
        }
    }

    pub fn read_line(&mut self, buf: &mut String) -> usize {
        self.r.read_line(buf).expect("Failed to read a line")
    }

    pub fn read_char(&mut self) -> char {
        let n = self
            .r
            .read(&mut self.buf[0..1])
            .expect("Failed to read a char");
        assert_eq!(1, n, "Unexpected read length: {}", n);
        self.buf[0] as char
    }

    pub fn read_u8(&mut self) -> u8 {
        let n = self
            .r
            .read(&mut self.buf[0..1])
            .expect("Failed to read an u8");
        assert_eq!(1, n, "Unexpected read length: {}", n);
        self.buf[0]
    }

    pub fn read_i32(&mut self) -> i32 {
        let mut ret = 0;
        let mut is_negative = false;

        let mut c: char;
        loop {
            c = self.read_char();
            if c == '-' || (c >= '0' && c <= '9') {
                break;
            }
            // Just ignore the unexpected character, and let's continue.
        }

        if c == '-' {
            is_negative = true;
        } else if c >= '0' && c <= '9' {
            ret = c as i32 - '0' as i32;
        } else {
            panic!("Unexpected char: {}", c);
        }

        loop {
            // FIXME: The first non-digit char will be ignored when next read start.
            let c = self.read_char();
            if c >= '0' && c <= '9' {
                ret = ret * 10 + (c as i32 - '0' as i32);
            } else {
                break;
            }
        }

        if is_negative {
            -ret
        } else {
            ret
        }
    }

    pub fn read_u32(&mut self) -> u32 {
        let mut ret = 0 as u32;

        // Ignore all the non-digit characters.
        loop {
            let c = self.read_char();
            if c >= '0' && c <= '9' {
                ret = c as u32;
                break;
            }
        }

        loop {
            // FIXME: The first non-digit char will be ignored when next read start.
            let c = self.read_char();
            if c >= '0' && c <= '9' {
                ret = ret * 10 + (c as u32 - '0' as u32);
            } else {
                break;
            }
        }

        return ret;
    }

    pub fn read_i64(&mut self) -> i64 {
        todo!()
    }

    pub fn read_u64(&mut self) -> u64 {
        todo!()
    }

    pub fn read_str(&mut self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::basic::rand;
    use std::env;
    use std::fs;

    #[test]
    pub fn basic() {
        let tmp_dir = env::temp_dir();
        let fname = tmp_dir.join(rand::gen_u32().to_string());

        let data = String::from("323 abc 2342");
        fs::write(fname.clone(), data.clone()).expect("Failed to write");

        let mut io = FileIO::new(fname.clone());
        for i in 0..data.len() {
            assert_eq!(data.as_bytes()[i] as char, io.read_char());
        }

        io = FileIO::new(fname.clone());
        assert_eq!(323, io.read_i32());
        assert_eq!(2342, io.read_i32());
    }
}
