use std::io::{Error, ErrorKind};
use std::{env, io, path::PathBuf};

pub fn testdata_dir() -> io::Result<PathBuf> {
    let cur_dir = env::current_dir()?;

    let testdata_dir = cur_dir
        .parent()
        .unwrap()
        .join("testdata")
        .join("algorithm-and-analysis");

    let testdata_tgz = cur_dir
        .parent()
        .unwrap()
        .join("testdata")
        .join("algorithm-and-analysis.tar.gz");

    if !testdata_dir.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!(
                "Cannot locate the algorithm test data directory in {:?}, please uncompress 'tar xzvf {:?}'",
                testdata_dir.to_str(), testdata_tgz.to_str()
            ),
        ));
    }

    Ok(testdata_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let ret = testdata_dir();
        assert_eq!(true, ret.unwrap().exists());
    }
}
