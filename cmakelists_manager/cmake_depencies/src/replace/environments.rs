use crate::parse;
use crate::search;
use crate::structs;
use parse::git_lib;
use parse::git_librarys;
use git_lib::ParamType;

use cmakelists_parse::parser::grammar::{CGrammar, ICall};

use std::path::Path;
use std::fs;

const keyword_git_include: &str = "git_include";
const keyword_git_lib: &str = "git_lib";
const keyword_git_libpath: &str = "git_libpath";
const keyword_git_librarys: &str = "git_librarys";
const keyword_set: &str = "set";

const keyword_target_link_libraries: &str = "target_link_libraries";
const keyword_link_directories: &str = "link_directories";
const keyword_include_directories: &str = "include_directories";

const keyword_cbb_store_root: &str = "${CBB_STORE_ROOT}";

const cmake_keyword_debug: &str = "debug";
const cmake_keyword_release: &str = "release";

const cmakelist_name: &str = "CMakelists.txt";

enum WriteStatus {
    Write,
    BackQuoteEnd,
    StopWrite
}

enum Mode {
    Normal,
    GitLibrarys
}

struct CCall {
    content: String,
    writeStauts: WriteStatus,
    mode: Mode,
    gitLibrarys: Vec<git_librarys::CGitLibrarys>,
    libraryConfigs: Vec<git_lib::CParam>,
    cbbStoreRoot: String
}

impl ICall for CCall {
    fn on_if(&mut self, value: &str){
        // println!("on if, value: {}", value);
    }

    fn on_else_if(&mut self, value: &str) {
        // println!("on else if, value: {}", value);
    }

    fn on_else(&mut self) {
    }

    fn on_end_if(&mut self) {
    }

    fn on_kv(&mut self, key: &str, value: &str) {
        let k = key.to_ascii_lowercase();
        let v = value.to_ascii_lowercase();
        match self.mode {
            Mode::Normal => {
                if k == keyword_set && v == keyword_git_librarys {
                    self.mode = Mode::GitLibrarys;
                } else if self.starts_with(&v, keyword_git_lib) {
                    self.removeContentRightLen(value.len() + 1);
                    // println!("{}, {:?}, {}", self.content.len(), &v, keyword_git_lib);
                    let parser = git_lib::CGitLibParser::new(&self.cbbStoreRoot);
                    let mut param = parser.parseFromStr(&value);
                    param.paramType = ParamType::LibName;
                    param.startIndex = self.content.len();
                    self.libraryConfigs.push(param);
                } else if self.starts_with(&v, keyword_git_libpath) {
                    self.removeContentRightLen(value.len() + 1);
                    // println!("{}, {:?}, {}, {}", self.content.len(), &v, keyword_git_libpath, &self.content);
                    let parser = git_lib::CGitLibParser::new(&self.cbbStoreRoot);
                    let mut param = parser.parseFromStr(&value);
                    param.paramType = ParamType::LibPath;
                    param.startIndex = self.content.len();
                    self.libraryConfigs.push(param);
                } else if self.starts_with(&v, keyword_git_include) {
                    self.removeContentRightLen(value.len() + 1);
                    // println!("{}, {:?}, {}", self.content.len(), &v, keyword_git_include);
                    let parser = git_lib::CGitLibParser::new(&self.cbbStoreRoot);
                    let mut param = parser.parseFromStr(&value);
                    param.paramType = ParamType::Include;
                    param.startIndex = self.content.len();
                    self.libraryConfigs.push(param);
                } else if (key.to_ascii_lowercase() == keyword_include_directories.to_ascii_lowercase())
                && (value.starts_with(keyword_cbb_store_root)) {
                    self.appendIncludeReplace(value);
                }
            },
            Mode::GitLibrarys => {
                // println!("before Mode::GitLibrarys, {:?}", self.content.len());
                self.removeContentRightLen(value.len() + 1);
                // println!("after Mode::GitLibrarys, {:?}", self.content.len());
                let parser = git_librarys::CGitLibParser::new();
                let params = parser.parseFromStr(&value);
                self.gitLibrarys.push(params);
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
        self.mode = Mode::Normal;
    }

    fn on_ch(&mut self, c: char) {
        match self.writeStauts {
            WriteStatus::BackQuoteEnd => {
                self.writeStauts = WriteStatus::Write;
            },
            WriteStatus::StopWrite => {
            },
            _ => {
                // if self.content.len() < 1000 {
                //     println!("{:?}, {}", self.content.len(), c);
                // }
                self.content.push(c);
            }
        }
    }

    fn on_double_quotes_end(&mut self) {
    }

    fn on_back_quote_end(&mut self) {
        self.writeStauts = WriteStatus::BackQuoteEnd;
    }
}

impl CCall {
    fn removeContentRightLen(&mut self, len: usize) {
        for i in 0..len {
            if self.content.len() == 0 {
                break;
            }
            self.content.pop();
        }
    }

    fn appendIncludeReplace(&mut self, value: &str) {
        let path = Path::new(&self.cbbStoreRoot);
        let mut afterPath = value.trim_left_matches(keyword_cbb_store_root).to_string();
        let bytes = afterPath.as_bytes();
        if bytes.len() > 0 {
            let c = bytes[0];
            if c == b'/' || c == b'\\' {
                afterPath.remove(0);
            }
        }
        let path = path.join(&afterPath);
        /*
        ** Convert to absolute path
        */
        match path.canonicalize() {
            Ok(p) => {
                match p.to_str() {
                    Some(s) => {
                        if cfg!(target_os="windows"){
                            let t = s.trim_left_matches(r#"\\?\"#).replace(r#"\"#, r#"\\"#);
                            self.removeContentRightLen(value.len() + 1);
                            self.content.insert(self.content.len(), '"');
                            self.content.insert_str(self.content.len(), &t);
                            self.content.insert(self.content.len(), '"');
                        } else {
                            self.removeContentRightLen(value.len() + 1);
                            self.content.insert_str(self.content.len(), s);
                        }
                    },
                    None => {
                        println!("[Error] include path abs to_str error");
                    }
                }
            },
            Err(err) => {
                println!("[Error] include path, path: {}", &value);
            }
        }
        // println!("{:?}, {}, {:?}", path.to_str(), afterPath, &self.path);
    }

    fn starts_with(&self, content: &str, s: &str) -> bool {
        let mut word = String::new();
        for c in content.chars() {
            if c == '{' {
                break;
            } else {
                if !c.is_ascii_whitespace() {
                    word.push(c);
                }
            }
        }
        if word == s {
            true
        } else {
            false
        }
        /*
        let sLen = s.len();
        let bs = s.as_bytes();
        let mut whiteLen = 0;
        for (i, c) in content.as_bytes().iter().enumerate() {
            if *c == b'{' {
                break;
            } else {
                if !c.is_ascii_whitespace() {
                    if i - whiteLen > sLen - 1 {
                        return false;
                    }
                    if *c != bs[i - whiteLen] {
                        return false;
                    }
                } else {
                    whiteLen += 1;
                }
            }
        }
        return true;
        */
    }
}

impl CCall {
    fn new(cbbStoreRoot: &str) -> CCall {
        CCall{
            content: String::new(),
            writeStauts: WriteStatus::Write,
            mode: Mode::Normal,
            gitLibrarys: Vec::new(),
            libraryConfigs: Vec::new(),
            cbbStoreRoot: cbbStoreRoot.to_string()
        }
    }
}

pub struct CEnvironments {
    parser: CGrammar
}

impl CEnvironments {
    pub fn parse(&self, path: &str, cbbStoreRoot: &str) -> Result<(Vec<git_librarys::CGitLibrarys>, Vec<git_lib::CParam>, String), &str> {
        let mut call = CCall::new(cbbStoreRoot);
        if let Err(err) = self.parser.parse(path, &mut call) {
            return Err(err);
        };
        // println!("{:?}", call.content);
        // println!("{:?}, {:?}", call.gitLibrarys, call.libraryConfigs);
        // println!("{:?}", call.gitLibrarys);
        // println!("{:?}", call.libraryConfigs);
        /*
        let p = Path::new(path);
        let parent = match p.parent() {
            Some(p) => p,
            None => {
                println!("get path parent path error");
                return Err("get path parent path error");
            }
        };
        let p = parent.join(cmakelist_name);
        /*
        ** Write file
        */
        if let Err(err) = fs::write(p, call.content) {
            println!("write CMakelists.txtx error, err:{}", err);
            return Err("write error");
        };
        */
        Ok((call.gitLibrarys, call.libraryConfigs, call.content))
    }
}

impl CEnvironments {
    pub fn new() -> CEnvironments {
        CEnvironments{
            parser: CGrammar::new()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    // #[ignore]
    fn environmentsTest() {
        let parser = CEnvironments::new();
        parser.parse("./doc/exe_cmake/CMakelists.config");
    }

    #[test]
    #[ignore]
    fn callStartWithTest() {
        let call = CCall::new();
        println!("{}", call.starts_with(" git_lib {}", "git_lib"));
        println!("{}", call.starts_with("git_libpath {}", "git_libpath"));
    }
}

