use super::git_lib;

use cmakelists_parse::parser::grammar::{CGrammar, ICall};

const keyword_git_include: &str = "git_include";
const keyword_git_lib: &str = "git_lib";

const keyword_target_link_libraries: &str = "target_link_libraries";

struct CCall {
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
            println!("{:?}", params);
        }
    }

    fn on_ch(&mut self, c: char) {
    }

    fn on_double_quotes_end(&mut self) {
    }
}

struct CCmakeParser {
    parser: CGrammar
}

impl CCmakeParser {
    pub fn parse(&self, path: &str) -> Result<(), &str> {
        let mut call = CCall{};
        if let Err(err) = self.parser.parse(path, &mut call) {
            return Err(err);
        };
        Ok(())
    }
}

impl CCmakeParser {
    pub fn new() -> CCmakeParser {
        CCmakeParser{
            parser: CGrammar::new()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[ignore]
    fn cmakeParserTest() {
        let parser = CCmakeParser::new();
        parser.parse("./resources/test3.txt");
    }
}

