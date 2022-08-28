use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Read, Result},
    path::Path,
};

pub struct FileIO {
    buf: Vec<u8>,
    r: BufReader<File>,
}

#[inline]
fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

#[inline]
fn to_digit(c: char) -> u8 {
    c as u8 - '0' as u8
}

impl FileIO {
    /// Initialize a FileIO instance.
    /// Use the generic `AsRef<Path>` here because it want us to pass &PathBuf here for convenience.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
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
        } else if is_digit(c) {
            ret = to_digit(c) as u32;
        } else {
            return Err(Error::new(ErrorKind::Other, "Unexpected byte"));
        }

        while self.read_char(&mut c)? != 0 {
            nread += 1;
            if is_digit(c) {
                ret = ret * 10 + to_digit(c) as u32;
            } else {
                break;
            }
        }

        if is_negative {
            if ret == (1 << 31) {
                *v = i32::MIN;
            } else if ret < (1 << 31) {
                *v = -(ret as i32);
            } else {
                panic!("Overflow to convert to i32: {}", ret);
            }
        } else {
            *v = ret as i32;
        }

        Ok(nread)
    }

    pub fn read_u32(&mut self, v: &mut u32) -> Result<usize> {
        let mut c = 0 as char;
        let mut nread = 0;

        while self.read_char(&mut c)? != 0 {
            nread += 1;
            if c != ' ' && c != '\n' {
                break;
            }
        }

        let mut ret = to_digit(c) as u32;
        while self.read_char(&mut c)? != 0 {
            nread += 1;
            if is_digit(c) {
                ret = ret * 10 + to_digit(c) as u32;
            } else {
                break;
            }
        }

        *v = ret;
        Ok(nread)
    }

    pub fn read_i64(&mut self) -> i64 {
        todo!()
    }

    pub fn read_u64(&mut self, v: &mut u64) -> Result<usize> {
        let mut c = 0 as char;
        let mut nread = 0;

        while self.read_char(&mut c)? != 0 {
            nread += 1;
            if c != ' ' && c != '\n' {
                break;
            }
        }

        let mut ret = to_digit(c) as u64;
        while self.read_char(&mut c)? != 0 {
            nread += 1;
            if is_digit(c) {
                ret = ret * 10 + to_digit(c) as u64;
            } else {
                break;
            }
        }

        *v = ret;
        Ok(nread)
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
    use std::path::PathBuf;

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

    fn f_write(s: String) -> Result<PathBuf> {
        let path = env::temp_dir().join(rand::gen_u32().to_string());
        fs::write(&path, s)?;
        Ok(path)
    }

    #[test]
    pub fn read_char() {
        let data = String::from("abc\n eof\n\nhello world.");
        let path = f_write(data.clone()).expect("Failed to write");

        let mut io = FileIO::new(&path);
        let mut c = 0x00 as char;
        for i in 0..data.len() {
            check_io(io.read_char(&mut c), 1);
            assert_eq!(data.as_bytes()[i] as char, c);
        }
    }

    #[test]
    pub fn read_line() {
        let data = String::from("abc\n eof\n\nhello world.");
        let path = f_write(data.clone()).expect("Failed to write");

        let mut io = FileIO::new(&path);
        let mut buf = String::new();

        check_io(io.read_line(&mut buf), 4);
        assert_eq!(buf, String::from("abc\n"));

        buf.clear();
        check_io(io.read_line(&mut buf), 5);
        assert_eq!(buf, String::from(" eof\n"));

        buf.clear();
        check_io(io.read_line(&mut buf), 1);
        assert_eq!(buf, String::from("\n"));

        buf.clear();
        check_io(io.read_line(&mut buf), 12);
        assert_eq!(buf, String::from("hello world."));

        buf.clear();
        check_io(io.read_line(&mut buf), 0);
        assert_eq!(buf, String::from(""));
    }

    #[test]
    pub fn read_i32() {
        let data = String::from("323 2342 -21 -2147483648   2147483647");
        let path = f_write(data.clone()).expect("Failed to write");

        let mut io = FileIO::new(&path);
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

    #[test]
    pub fn read_u64() {
        let data = String::from("18446744073709551615 0 123 2147483647");
        let path = f_write(data.clone()).expect("Failed to write");

        let mut io = FileIO::new(&path);
        let mut v = 0 as u64;
        check_io(io.read_u64(&mut v), 21);
        assert_eq!(18446744073709551615, v);

        check_io(io.read_u64(&mut v), 2);
        assert_eq!(0, v);

        check_io(io.read_u64(&mut v), 4);
        assert_eq!(123, v);

        check_io(io.read_u64(&mut v), 10);
        assert_eq!(2147483647, v);

        check_io(io.read_u64(&mut v), 0);
    }
}
