/*
** parse format:
** git_lib { name = awnet_client, version = 0.2.1, platform = ${FILE_PREFIX}, extra = '' }
*/
use super::object;
use super::git_librarys;

const keyword_name: &str = "name";
const keyword_version: &str = "version";
const keyword_platform: &str = "platform";
const keyword_target: &str = "target";
const keyword_extra: &str = "extra";
const keyword_extra_type: &str = "extra_type";

#[derive(Default, Debug)]
pub struct CGitLib<'a> {
    pub library: Option<&'a git_librarys::CGitLibrarys>,
    pub platform: Option<String>,
    pub target: Option<String>,
    pub extra: Option<String>,
    pub extraType: Option<String>
}

#[derive(Debug, Clone)]
pub enum ParamType {
    Unknow,
    LibName,
    LibPath,
    Include
}

impl Default for ParamType {
    fn default() -> ParamType {
        ParamType::Unknow
    }
}

#[derive(Default, Debug)]
pub struct CParam {
    pub paramType: ParamType,
    pub startIndex: usize,
    pub platform: Option<String>,
    pub target: Option<String>,
    pub extra: Option<String>,
    pub extraType: Option<String>
}

impl object::IObject for CParam {
    fn on_kv(&mut self, key: &str, value: &str) {
        if key == keyword_platform {
            self.platform = Some(value.to_string());
        } else if key == keyword_target {
            self.target = Some(value.to_string());
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
    pub fn parseFromStr(&self, data: &str) -> CParam {
        let mut param = CParam::default();
        let parser = object::CObjectParser::new();
        parser.parse(data, &mut param);
        param
    }
}

impl CGitLibParser {
    pub fn new() -> CGitLibParser {
        CGitLibParser{}
    }
}
