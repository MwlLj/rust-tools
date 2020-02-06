use path::walk;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Default)]
pub struct CFinder {
}

impl CFinder {
    pub fn find(&self, root: &str, keyword: &str) {
        walk::scan::walk_one_fn(root, &mut |path: &str, name: &str, t: walk::scan::Type| -> bool {
            match t {
                walk::scan::Type::Dir => {
                    // println!("{:?}", path);
                    true
                },
                walk::scan::Type::File => {
                    self.read_file(path, keyword.as_bytes());
                    true
                },
                walk::scan::Type::OnceEnd => {
                    true
                }
            }
        });
    }

    fn read_file(&self, path: &str, keyword: &[u8]) {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(err) => {
                println!("read file error, err: {}", err);
                return;
            }
        };
        let mut bufReader = BufReader::new(file);
        let mut buffer = Vec::new();
        let mut lineNum = 0;
        loop {
            let len = match bufReader.read_until(b'\n', &mut buffer) {
                Ok(l) => l,
                Err(err) => {
                    break;
                }
            };
            if len == 0 {
                break;
            }
            lineNum += 1;
            if self.contains(keyword, buffer.as_slice()) {
                let content = match String::from_utf8(buffer.clone()) {
                    Ok(c) => c,
                    Err(err) => {
                        String::from("")
                    }
                };
                println!("path: {}\n\tline: {}\n\tcontent: {}", path, lineNum, content);
            }
            buffer.clear();
        }
    }

    fn contains(&self, key: &[u8], buf: &[u8]) -> bool {
        // println!("{:?}, {:?}", key, buf);
        let keyLen = key.len();
        let mut bufSlice = &buf[..];
        loop {
            let bufLen = bufSlice.len();
            if bufLen < keyLen {
                break;
            }
            if bufSlice.starts_with(key) {
                return true;
            } else {
                if bufLen <= 1 {
                    break;
                } else {
                    bufSlice = &bufSlice[1..];
                }
            }
        }
        false
    }

    /*
    ** 递归的方式可能会引起栈溢出
    */
    fn contains_recursion(&self, key: &[u8], buf: &[u8]) -> bool {
        // println!("{:?}, {:?}", key, buf);
        let keyLen = key.len();
        let bufLen = buf.len();
        if bufLen < keyLen {
            return false;
        }
        if buf.starts_with(key) {
            return true;
        } else {
            if bufLen <= 1 {
                return false;
            } else {
                return self.contains_recursion(key, &buf[1..])
            }
        }
    }
}

impl CFinder {
    pub fn new() -> CFinder {
        let f = CFinder{};
        return f;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    // #[ignore]
    fn containsTest() {
        let finder = CFinder::new();
        let b = finder.contains(&[1, 2, 3], &[0, 1, 2, 4, 1, 2, 3, 3]);
        // let b = finder.contains(&[1, 2, 3], &[0]);
        println!("{:?}", b);
    }
}
