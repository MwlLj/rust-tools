use crate::parse::git_lib;
use crate::search;
use crate::structs;

use cmakelists_parse::parser::grammar::{CGrammar, ICall};

use std::path::Path;
use std::fs;

const keyword_git_include: &str = "git_include";
const keyword_git_lib: &str = "git_lib";

const keyword_target_link_libraries: &str = "target_link_libraries";
const keyword_link_directories: &str = "link_directories";

const cmake_keyword_debug: &str = "debug";
const cmake_keyword_release: &str = "release";

const cmakelist_name: &str = "CMakelists.txt";

enum WriteStatus {
    Write,
    BackQuoteEnd,
    StopWrite
}

struct CCall {
    content: String,
    writeStauts: WriteStatus,
    runArgs: structs::param::CRunArgs,
    root: String,
    linkDirectoriesIndex: usize
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
        // println!("key: {}, value: {}", key, value);
        if value.starts_with(keyword_git_lib) {
            let parser = git_lib::CGitLibParser::new();
            let params = parser.parseFromStr(value);
            self.removeContentRightLen(value.len() + 1);
            /*
            ** Get the collection of names of all libraries that specify the library and the specified library dependencies
            */
            let searcher = search::dependencies::CDependSearcher::new();
            let mut results = Vec::new();
            if let Err(err) = searcher.search(&self.runArgs, &self.root, &params, &mut results) {
                println!("search error, err: {}", err);
                return;
            };
            // println!("{:?}, {:?}", &params, &results);
            /*
            ** Record the total length before insertion
            */
            let startLen = self.content.len();
            for result in results.iter() {
                let fullName = &result.name;
                match &fullName.dr {
                    Some(name) => {
                        self.content.push_str(&name);
                    },
                    None => {
                        match &fullName.debug {
                            Some(name) => {
                                self.content.push_str(cmake_keyword_debug);
                                self.content.push_str(" ");
                                self.content.push_str(name);
                            },
                            None => {
                                println!("[Warning debug is None]");
                                continue;
                            }
                        }
                        self.content.push_str(" ");
                        match &fullName.release {
                            Some(name) => {
                                self.content.push_str(cmake_keyword_release);
                                self.content.push_str(" ");
                                self.content.push_str(&name);
                            },
                            None => {
                                println!("[Warning release is None]");
                                continue;
                            }
                        }
                        #[cfg(target_os="windows")]
                        self.content.push_str("\r");
                        self.content.push_str("\n");
                    }
                }
                /*
                ** Record the total length after insertion
                */
                let endLen = self.content.len();
                /*
                ** Calculate the length of this insertion
                */
                let writeLen = endLen - startLen;
                /*
                ** If self.linkDirectoriesIndex == 0,
                ** indicating that linkDirectories appears after this insertion,
                ** add self.linkDirectoriesIndex to this accumulated length.
                */
                self.plusOffset(writeLen);
                /*
                ** Insert path include path
                */
                match &result.libpath.libpath {
                    Some(path) => {
                        self.content.insert(self.linkDirectoriesIndex, '"');
                        self.linkDirectoriesIndex += 1;
                        self.content.insert_str(self.linkDirectoriesIndex, &path);
                        self.linkDirectoriesIndex += path.len();
                        self.content.insert(self.linkDirectoriesIndex, '"');
                        self.linkDirectoriesIndex += 1;
                        if cfg!(target_os="windows") {
                            self.content.insert(self.linkDirectoriesIndex, '\r');
                            self.linkDirectoriesIndex += 1;
                        }
                        self.content.insert(self.linkDirectoriesIndex, '\n');
                        self.linkDirectoriesIndex += 1;
                    },
                    None => {}
                }
            }
            println!("{:?}, {:?}", &params, &results);
            // println!("{:?}", params);
        }
        /*
        if key.to_ascii_lowercase() == keyword_link_directories.to_ascii_lowercase() {
            // println!("key: {}, value: {}", key, value);
            self.writeStauts = WriteStatus::StopWrite;
        }
        */
    }

    fn on_k_end(&mut self, key: &str) {
        if key.to_ascii_lowercase() == keyword_link_directories.to_ascii_lowercase() {
            self.linkDirectoriesIndex = self.content.len() - 1;
            /*
            if cfg!(target_os="windows") {
                self.content.push_str("\r");
            }
            self.content.push_str("\n");
            // self.writeStauts = WriteStatus::Write;
            */
        }
    }

    fn on_ch(&mut self, c: char) {
        match self.writeStauts {
            WriteStatus::BackQuoteEnd => {
                self.writeStauts = WriteStatus::Write;
            },
            WriteStatus::StopWrite => {
            },
            _ => {
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

    fn plusOffset(&mut self, len: usize) {
        if self.linkDirectoriesIndex == 0 {
            self.linkDirectoriesIndex += len;
        }
    }
}

impl CCall {
    fn new(runArgs: &structs::param::CRunArgs, root: &str) -> CCall {
        CCall{
            content: String::new(),
            writeStauts: WriteStatus::Write,
            runArgs: runArgs.clone(),
            root: root.to_string(),
            linkDirectoriesIndex: 0
        }
    }
}

struct CCmakeParser {
    parser: CGrammar,
    runArgs: structs::param::CRunArgs,
    root: String
}

impl CCmakeParser {
    pub fn parse(&self, path: &str) -> Result<(), &str> {
        let mut call = CCall::new(&self.runArgs, &self.root);
        if let Err(err) = self.parser.parse(path, &mut call) {
            return Err(err);
        };
        println!("{:?}", call.content);
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
        Ok(())
    }
}

impl CCmakeParser {
    pub fn new(runArgs: &structs::param::CRunArgs, root: &str) -> CCmakeParser {
        CCmakeParser{
            parser: CGrammar::new(),
            runArgs: runArgs.clone(),
            root: root.to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    // #[ignore]
    fn cmakeParserTest() {
        let parser = CCmakeParser::new(&structs::param::CRunArgs::default(), ".");
        parser.parse("./doc/exe_cmake/CMakelists.config");
    }
}

