/*
** parse format:
** git_lib { name = awnet_client, version = 0.2.1, platform = ${FILE_PREFIX}, extra = '' }
*/
use super::object;

const keyword_name: &str = "name";
const keyword_version: &str = "version";
const keyword_subs: &str = "subs";
const keyword_enable: &str = "enable";
const keyword_include_enable: &str = "includeEnable";
const keyword_libpath_enable: &str = "libpathEnable";
const keyword_libname_enable: &str = "libnameEnable";
const keyword_self: &str = "self";
const keyword_config: &str = "config";
pub const subs_sp: &str = ",";
pub const subs_null: &str = "_";

#[derive(Default, Debug, Clone)]
pub struct CGitLibrarys {
    pub name: Option<String>,
    pub version: Option<String>,
    pub libs: String,
    pub dlls: String,
    pub enable: Option<String>,
    pub includeEnable: Option<String>,
    pub libpathEnable: Option<String>,
    pub libnameEnable: Option<String>,
    pub isSelf: Option<String>,
    pub config: Option<String>
}

impl object::IObject for CGitLibrarys {
    fn on_kv(&mut self, key: &str, value: &str) {
        if key == keyword_name {
            self.name = Some(value.to_string());
            // self.libs = vec![value.to_string()];
            self.libs = value.to_string();
        } else if key == keyword_version {
            self.version = Some(value.to_string());
        } else if key == keyword_subs {
            /*
            self.libs = Vec::new();
            if value.trim() == subs_null {
                return;
            }
            let vs: Vec<&str> = value.split(subs_sp).collect();
            for v in vs {
                self.libs.push(v.trim().to_string());
            }
            */
            if value.trim() == subs_null {
                return;
            }
            self.libs = value.to_string();
        } else if key == keyword_enable {
            self.enable = Some(value.to_string());
        } else if key == keyword_include_enable {
            self.includeEnable = Some(value.to_string());
        } else if key == keyword_libpath_enable {
            self.libpathEnable = Some(value.to_string());
        } else if key == keyword_libname_enable {
            self.libnameEnable = Some(value.to_string());
        } else if key == keyword_self {
            self.isSelf = Some(value.to_string());
        } else if key == keyword_config {
            self.config = Some(value.to_string());
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
