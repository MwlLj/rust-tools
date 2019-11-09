/*
** Read the .cmake file and merge it into content
*/
use cmakelists_parse::parser::grammar::{CGrammar, ICall};
use path::walk;

use std::path::Path;
use std::fs;

const keyword_set: &str = "set";
const keyword_git_cmakes: &str = "git_cmakes";

enum Mode {
    Normal,
    GitCMakes
}

enum WriteStatus {
    Write,
    StopWrite
}

struct CCall {
    content: String,
    mode: Mode,
    writeStatus: WriteStatus,
    cmakesStartIndex: usize,
    cmakeDir: String
}

impl ICall for CCall {
    fn on_if(&mut self, value: &str){
    }

    fn on_else_if(&mut self, value: &str) {
    }

    fn on_else(&mut self) {
    }

    fn on_end_if(&mut self) {
    }

    fn on_kv(&mut self, key: &str, value: &str) {
        let k = key.to_ascii_lowercase();
        match self.mode {
            Mode::Normal => {
                if k == keyword_git_cmakes {
                    println!("{:?}", &self.content);
                    self.popUtilEqualWord(key);
                    self.mode = Mode::GitCMakes;
                    self.writeStatus = WriteStatus::StopWrite;
                }
            },
            Mode::GitCMakes => {
                self.removeContentRightLen(value.len());
                let p = Path::new(self.cmakeDir.as_str()).join(value);
                match fs::read(p) {
                    Ok(c) => {
                        match String::from_utf8(c) {
                            Ok(s) => {
                                self.content.push_str(&s);
                                if cfg!(target_os="windows") {
                                    self.content.push('\r');
                                }
                                self.content.push('\n');
                            },
                            Err(err) => {
                                println!("cmake file from_utf8 error, file: {}, err: {}", value, err);
                                return;
                            }
                        }
                    },
                    Err(err) => {
                        println!("[Error] read cmake file error, file: {}, err: {}", value, err);
                        return;
                    }
                }
            },
            _ => {}
        }
        /*
        if key.to_ascii_lowercase() == keyword_link_directories.to_ascii_lowercase() {
            // println!("key: {}, value: {}", key, value);
            self.writeStauts = WriteStatus::StopWrite;
        }
        */
    }

    fn on_k_end(&mut self, key: &str) {
        // println!("{:?}", self.content.len());
        self.content.pop();
        match self.mode {
            Mode::GitCMakes => {
                self.writeStatus = WriteStatus::StopWrite;
            },
            _ => {
            }
        }
        self.mode = Mode::Normal;
    }

    fn on_ch(&mut self, c: char) {
        match self.writeStatus {
            WriteStatus::Write => {
                self.content.push(c);
            },
            WriteStatus::StopWrite => {
                match self.mode {
                    Mode::GitCMakes => {
                        self.writeStatus = WriteStatus::Write;
                    },
                    _ => {
                    }
                }
            }
        }
    }

    fn on_double_quotes_start(&mut self) {
        match self.mode {
            Mode::GitCMakes => {
                self.writeStatus = WriteStatus::StopWrite;
            },
            _ => {
            }
        }
    }

    fn on_double_quotes_end(&mut self) {
        match self.mode {
            Mode::GitCMakes => {
                self.writeStatus = WriteStatus::StopWrite;
            },
            _ => {
            }
        }
    }

    fn on_back_quote_end(&mut self) {
    }
}

impl CCall {
    fn popUtilEqualWord(&mut self, word: &str) {
        let mut buffer = String::new();
        loop {
            match self.content.pop() {
                Some(c) => {
                    if c.is_ascii_whitespace() {
                        buffer.clear();
                    } else {
                        buffer.insert(0, c);
                    }
                },
                None => {
                    break;
                }
            }
            if buffer == word {
                break;
            }
        }
    }

    fn removeContentRightLen(&mut self, len: usize) {
        for i in 0..len {
            if self.content.len() == 0 {
                break;
            }
            self.content.pop();
        }
    }
}

impl CCall {
    pub fn new(cmakeDir: String) -> CCall {
        CCall{
            content: String::new(),
            mode: Mode::Normal,
            writeStatus: WriteStatus::Write,
            cmakesStartIndex: 0,
            cmakeDir: cmakeDir
        }
    }
}

pub struct CMerge {
    parser: CGrammar
}

impl CMerge {
    pub fn merge(&self, cmakePath: &str) -> Result<String, &str> {
        let cmakeDir = Path::new(cmakePath).parent().unwrap().to_str().unwrap().to_string();
        let mut call = CCall::new(cmakeDir);
        if let Err(err) = self.parser.parse(cmakePath, &mut call) {
            return Err(err);
        };
        println!("{:?}", &call.content);
        return Ok(call.content)
    }
}

impl CMerge {
    pub fn new() -> CMerge {
        CMerge{
            parser: CGrammar::new()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[ignore]
    fn mergeTest() {
        let merge = CMerge::new();
        merge.merge("./doc/exe_cmake/CMakeLists.config");
    }
}
