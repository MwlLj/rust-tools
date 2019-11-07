/*
** parse format:
** git_lib { name = awnet_client, version = 0.2.1, platform = ${FILE_PREFIX}, extra = '' }
*/
use super::object;
use super::git_librarys;

use std::path::Path;

const keyword_name: &str = "name";
const keyword_version: &str = "version";
const keyword_platform: &str = "platform";
const keyword_target: &str = "target";
const keyword_enable: &str = "enable";
const keyword_include_enable: &str = "include_enable";
const keyword_libpath_enable: &str = "libpath_enable";
const keyword_libname_enable: &str = "libname_enable";
const keyword_extra: &str = "extra";
const keyword_extra_type: &str = "extra_type";
const keyword_self: &str = "self";

#[derive(Default, Debug)]
pub struct CGitLib<'a> {
    pub library: Option<&'a git_librarys::CGitLibrarys>,
    pub platform: Option<String>,
    pub target: Option<String>,
    pub enable: Option<String>,
    pub includeEnable: Option<String>,
    pub libpathEnable: Option<String>,
    pub libnameEnable: Option<String>,
    pub extra: Option<String>,
    pub extraType: Option<String>,
    pub isSelf: Option<String>
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

#[derive(Default, Debug, Clone)]
pub struct CParam {
    pub cbbStoreRoot: String,
    pub paramType: ParamType,
    pub startIndex: usize,
    pub platform: Option<String>,
    pub target: Option<String>,
    pub enable: Option<String>,
    pub includeEnable: Option<String>,
    pub libpathEnable: Option<String>,
    pub libnameEnable: Option<String>,
    pub extra: Option<String>,
    pub extraType: Option<String>,
    pub isSelf: Option<String>
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
        }
        println!("{:?}", key);
    }
}

#[derive(Default)]
pub struct CGitLibParser {
    cbbStoreRoot: String
}

impl CGitLibParser {
    pub fn parseFromStr(&self, data: &str) -> CParam {
        let mut param = CParam::default();
        param.cbbStoreRoot = self.cbbStoreRoot.to_string();
        let parser = object::CObjectParser::new();
        parser.parse(data, &mut param);
        param
    }
}

impl CGitLibParser {
    pub fn new(cbbStoreRoot: &str) -> CGitLibParser {
        let mut obj = CGitLibParser::default();
        obj.cbbStoreRoot = cbbStoreRoot.to_owned();
        obj
    }
}
