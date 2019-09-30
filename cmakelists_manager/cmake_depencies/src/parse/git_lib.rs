/*
** parse format:
** git_lib { name = awnet_client, version = 0.2.1, platform = ${FILE_PREFIX}, extra = '' }
*/
use super::object;

const keyword_name: &str = "name";
const keyword_version: &str = "version";
const keyword_platform: &str = "platform";
const keyword_extra: &str = "extra";
const keyword_extra_type: &str = "extra_type";

#[derive(Default, Debug)]
pub struct CGitLib {
    pub name: Option<String>,
    pub version: Option<String>,
    pub platform: Option<String>,
    pub extra: Option<String>,
    pub extraType: Option<String>
}

impl object::IObject for CGitLib {
    fn on_kv(&mut self, key: &str, value: &str) {
        if key == keyword_name {
            self.name = Some(value.to_string());
        } else if key == keyword_version {
            self.version = Some(value.to_string());
        } else if key == keyword_platform {
            self.platform = Some(value.to_string());
        } else if key == keyword_extra {
            self.extra = Some(value.to_string());
        } else if key == keyword_extra_type {
            self.extraType = Some(value.to_string());
        }
    }
}

pub struct CGitLibParser {
}

impl CGitLibParser {
    pub fn parseFromStr(&self, data: &str) -> CGitLib {
        let mut gitlib = CGitLib::default();
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
