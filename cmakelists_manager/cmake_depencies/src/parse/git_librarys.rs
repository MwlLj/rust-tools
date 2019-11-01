/*
** parse format:
** git_lib { name = awnet_client, version = 0.2.1, platform = ${FILE_PREFIX}, extra = '' }
*/
use super::object;

const keyword_name: &str = "name";
const keyword_version: &str = "version";
const keyword_subs: &str = "subs";
pub const subs_sp: &str = ",";

#[derive(Default, Debug, Clone)]
pub struct CGitLibrarys {
    pub name: Option<String>,
    pub version: Option<String>,
    pub libs: Vec<String>
}

impl object::IObject for CGitLibrarys {
    fn on_kv(&mut self, key: &str, value: &str) {
        if key == keyword_name {
            self.name = Some(value.to_string());
        } else if key == keyword_version {
            self.version = Some(value.to_string());
        } else if key == keyword_subs {
            let mut subs = Vec::new();
            let vs: Vec<&str> = value.split(subs_sp).collect();
            for v in vs {
                subs.push(v.to_string());
            }
            self.libs = subs;
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
        if gitlib.libs.len() == 0 {
            gitlib.libs.push(gitlib.name.clone().expect("name field is null"));
        }
        gitlib
    }
}

impl CGitLibParser {
    pub fn new() -> CGitLibParser {
        CGitLibParser{}
    }
}
