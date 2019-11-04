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
const keyword_path: &str = "path";
const keyword_extra: &str = "extra";
const keyword_extra_type: &str = "extra_type";

const keyword_cbb_store_root: &str = "$CBB_STORE_ROOT";

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
    cbbStoreRoot: String,
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
        /*
        else if key == keyword_path {
            if value.starts_with(keyword_cbb_store_root) {
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
                                    self.path = Some(t);
                                } else {
                                    self.path = Some(s.to_string());
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
        }
        */
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
