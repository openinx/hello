use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Read, Result},
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

    pub fn read_line(&mut self, buf: &mut String) -> Result<usize> {
        self.r.read_line(buf)
    }

    pub fn read_char(&mut self, v: &mut char) -> Result<usize> {
        let n = self.r.read(&mut self.buf[0..1])?;
        *v = self.buf[0] as char;
        Ok(n)
    }

    pub fn read_u8(&mut self, v: &mut u8) -> Result<usize> {
        let n = self.r.read(&mut self.buf[0..1])?;
        *v = self.buf[0];
        Ok(n)
    }

    pub fn read_i32(&mut self, v: &mut i32) -> Result<usize> {
        let mut ret = 0;
        let mut is_negative = false;

        let mut c: char = 0x00 as char;
        let mut nread = 0;

        while self.read_char(&mut c)? != 0 {
            nread += 1;
            if c != ' ' && c != '\n' {
                break;
            }
        }

        if c == '-' {
            is_negative = true;
        } else if c >= '0' && c <= '9' {
            ret = c as i32 - '0' as i32;
        } else {
            return Err(Error::new(ErrorKind::Other, "Unexpected byte"));
        }

        while self.read_char(&mut c)? != 0 {
            nread += 1;
            if c >= '0' && c <= '9' {
                ret = ret * 10 + (c as i32 - '0' as i32);
            } else {
                break;
            }
        }

        *v = if is_negative { -ret } else { ret };
        Ok(nread)
    }

    pub fn read_u32(&mut self, v: &mut u32) -> Result<usize> {
        todo!()
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

    fn check_io(r: Result<usize>, io_bytes: usize) {
        match r {
            Ok(n) => {
                assert_eq!(n, io_bytes);
            }
            Err(e) => {
                panic!("Unexpected err: {:?}", e)
            }
        }
    }

    #[test]
    pub fn basic() {
        let tmp_dir = env::temp_dir();
        let fname = tmp_dir.join(rand::gen_u32().to_string());

        let data = String::from("323 2342 -21 -2147483648   2147483647");
        fs::write(fname.clone(), data.clone()).expect("Failed to write");

        let mut io = FileIO::new(fname.clone());
        let mut c: char = 0x00 as char;
        for i in 0..data.len() {
            check_io(io.read_char(&mut c), 1);
            assert_eq!(data.as_bytes()[i] as char, c);
        }

        io = FileIO::new(fname.clone());
        let mut v = 0;
        check_io(io.read_i32(&mut v), 4);
        assert_eq!(323, v);

        check_io(io.read_i32(&mut v), 5);
        assert_eq!(2342, v);

        check_io(io.read_i32(&mut v), 4);
        assert_eq!(-21, v);

        check_io(io.read_i32(&mut v), 12);
        assert_eq!(i32::MIN, v);

        check_io(io.read_i32(&mut v), 12);
        assert_eq!(i32::MAX, v);
    }
}
