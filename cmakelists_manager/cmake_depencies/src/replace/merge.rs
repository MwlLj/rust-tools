/*
** Read the .cmake file and merge it into content
*/
use cmakelists_parse::parser::grammar::{CGrammar, ICall};
use path::walk;
use super::var_replace::CVarReplace;

use std::path::Path;
use std::fs;
use std::collections::HashMap;

const keyword_set: &str = "set";
const keyword_git_cmakes: &str = "git_cmakes";

enum Mode {
    Normal,
    GitCMakes
}

enum ValueMode {
    Normal,
    First,
    AfterFirst(String)
}

enum WriteStatus {
    Write,
    KStartStopWrite,
    KEndStopWrite
}

struct CCall {
    content: String,
    mode: Mode,
    valueMode: ValueMode,
    writeStatus: WriteStatus,
    cmakesStartIndex: usize,
    cmakeDir: String,
    vars: HashMap<String, Vec<String>>
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
                    // println!("{:?}", &self.content);
                    self.popUtilEqualWord(key);
                    self.mode = Mode::GitCMakes;
                    self.writeStatus = WriteStatus::KStartStopWrite;
                } else if k == keyword_set {
                    match &self.valueMode {
                        ValueMode::Normal => {
                            self.valueMode = ValueMode::First;
                        },
                        ValueMode::First => {
                            self.vars.insert(value.to_string(), Vec::new());
                            self.valueMode = ValueMode::AfterFirst(value.to_string());
                        },
                        ValueMode::AfterFirst(firstKey) => {
                            match self.vars.get_mut(firstKey.as_str()) {
                                Some(var) => {
                                    (*var).push(value.to_string());
                                },
                                None => {
                                }
                            }
                        }
                    }
                }
            },
            Mode::GitCMakes => {
                self.removeContentRightLen(value.len());
                let p = Path::new(self.cmakeDir.as_str()).join(value);
                match fs::read(p) {
                    Ok(c) => {
                        match String::from_utf8(c) {
                            Ok(s) => {
                                // println!("{:?}", &self.vars);
                                self.content.push_str(&self.varReplace(&s));
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
        match self.mode {
            Mode::GitCMakes => {
                self.writeStatus = WriteStatus::KEndStopWrite;
            },
            _ => {
                self.mode = Mode::Normal;
            }
        }
        self.valueMode = ValueMode::Normal;
    }

    fn on_ch(&mut self, c: char) {
        match self.writeStatus {
            WriteStatus::Write => {
                self.content.push(c);
            },
            WriteStatus::KStartStopWrite => {
                match self.mode {
                    Mode::GitCMakes => {
                        self.writeStatus = WriteStatus::Write;
                        // self.mode = Mode::Normal;
                    },
                    _ => {
                    }
                }
            },
            WriteStatus::KEndStopWrite => {
                match self.mode {
                    Mode::GitCMakes => {
                        self.writeStatus = WriteStatus::Write;
                        self.mode = Mode::Normal;
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
                self.writeStatus = WriteStatus::KStartStopWrite;
            },
            _ => {
            }
        }
    }

    fn on_double_quotes_end(&mut self) {
        match self.mode {
            Mode::GitCMakes => {
                self.writeStatus = WriteStatus::KStartStopWrite;
            },
            _ => {
            }
        }
    }

    fn on_back_quote_end(&mut self) {
    }
}

impl CCall {
    fn varReplace(&self, content: &str) -> String {
        let replacer = CVarReplace::new();
        replacer.replace(content, &self.vars)
    }

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
            valueMode: ValueMode::Normal,
            writeStatus: WriteStatus::Write,
            cmakesStartIndex: 0,
            cmakeDir: cmakeDir,
            vars: HashMap::new()
        }
    }
}

pub struct CMerge {
    parser: CGrammar
}

impl CMerge {
    pub fn merge(&self, cmakePath: &str) -> Result<String, &str> {
        let cmakeDir = Path::new(cmakePath).parent().expect("cmakePath parent error").to_str().expect("cmakePath parent to_str error").to_string();
        let mut call = CCall::new(cmakeDir);
        if let Err(err) = self.parser.parse(cmakePath, &mut call) {
            return Err(err);
        };
        // println!("{:?}", &call.content);
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
