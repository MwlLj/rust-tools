/*
** parse format:
** git_lib { name = awnet_client, version = 0.2.1, platform = ${FILE_PREFIX}, extra = '' }
*/
use super::object;

const keyword_name: &str = "name";
const keyword_version: &str = "version";

#[derive(Default, Debug, Clone)]
pub struct CGitLibrarys {
    pub name: Option<String>,
    pub version: Option<String>
}

impl object::IObject for CGitLibrarys {
    fn on_kv(&mut self, key: &str, value: &str) {
        if key == keyword_name {
            self.name = Some(value.to_string());
        } else if key == keyword_version {
            self.version = Some(value.to_string());
        }
    }
}

pub struct CGitLibParser {
}

impl CGitLibParser {
    pub fn parseFromStr(&self, data: &str) -> CGitLibrarys {
        let mut gitlib = CGitLibrarys::default();
        let parser = object::CObjectParser::new();
        parser.parse(data, &mut gitlib);
        gitlib
    }
}

impl CGitLibParser {
    pub fn new() -> CGitLibParser {
        CGitLibParser{}
    }
}
