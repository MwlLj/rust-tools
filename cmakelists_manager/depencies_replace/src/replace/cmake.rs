use crate::parse::git_lib;
use crate::search;
use crate::calc;

use cmakelists_parse::parser::grammar::{CGrammar, ICall};

const keyword_git_include: &str = "git_include";
const keyword_git_lib: &str = "git_lib";

const keyword_target_link_libraries: &str = "target_link_libraries";

const cmake_keyword_debug: &str = "debug";
const cmake_keyword_release: &str = "release";

enum WriteStatus {
    Write,
    BackQuoteEnd
}

struct CCall {
    content: String,
    writeStauts: WriteStatus,
    root: String
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
            if let Err(err) = searcher.search(&self.root, &params, &mut results) {
                println!("search error, err: {}", err);
                return;
            };
            // println!("{:?}, {:?}", &params, &results);
            for result in results.iter() {
                match &result.dr {
                    Some(name) => {
                        self.content.push_str(&name);
                    },
                    None => {
                        match &result.debug {
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
                        match &result.release {
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
            }
            println!("{:?}, {:?}", &params, &results);
            // println!("{:?}", params);
        }
    }

    fn on_ch(&mut self, c: char) {
        match self.writeStauts {
            WriteStatus::BackQuoteEnd => {
                self.writeStauts = WriteStatus::Write;
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
}

impl CCall {
    fn new(root: &str) -> CCall {
        CCall{
            content: String::new(),
            writeStauts: WriteStatus::Write,
            root: root.to_string()
        }
    }
}

struct CCmakeParser {
    parser: CGrammar,
    root: String
}

impl CCmakeParser {
    pub fn parse(&self, path: &str) -> Result<(), &str> {
        let mut call = CCall::new(&self.root);
        if let Err(err) = self.parser.parse(path, &mut call) {
            return Err(err);
        };
        println!("{:?}", call.content);
        Ok(())
    }
}

impl CCmakeParser {
    pub fn new(root: &str) -> CCmakeParser {
        CCmakeParser{
            parser: CGrammar::new(),
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
        let parser = CCmakeParser::new(".");
        parser.parse("./doc/exe_cmake/CMakelists.config");
    }
}

