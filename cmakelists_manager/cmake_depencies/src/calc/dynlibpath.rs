use crate::parse;
use crate::config;
use crate::structs;

use json::{JsonValue};
use path_abs::PathAbs;

use std::path::Path;

const libpath_rule_default: &str = "`var:'config'`/lib/`var:'version'`/`var:'target'`/`var:'platform'`";
const include_rule_default: &str = "`var:'config'``var:'version'";
const platform_default: &str = "";
const target_default: &str = "";

const extra_type_string: &str = "string";
const extra_type_json: &str = "json";

const keyword_extra: &str = "extra";
const keyword_version: &str = "version";
const keyword_target: &str = "target";
const keyword_platform: &str = "platform";
const keyword_config: &str = "config";

fn append(jsonValue: &JsonValue, result: &mut String) {
    match jsonValue {
        JsonValue::Null => {
        },
        JsonValue::Short(v) => {
            result.push_str(&v.to_string());
            // println!("short: {}, result: {}", v, result);
        },
        JsonValue::String(v) => {
            result.push_str(&v);
        },
        JsonValue::Number(v) => {
            result.push_str(&v.to_string());
        },
        JsonValue::Boolean(v) => {
            result.push_str(&v.to_string());
        },
        _ => {}
    }
}

fn join<'a, 'b:'a>(content: &'a str, configPath: &str, version: &str, platform: &str, target: &str, mut extraJson: &'a JsonValue, mut extraJsonClone: &'b JsonValue, result: &mut String) -> Result<(), &'a str> {
    parse::joinv2::parse(content
    , &mut |t: &str, valueType: parse::joinv2::ValueType| {
        if t == keyword_extra {
            match valueType {
                parse::joinv2::ValueType::Start => {
                    extraJson = &extraJsonClone;
                },
                parse::joinv2::ValueType::Object => {
                    extraJson = &extraJson[t];
                    append(extraJson, result);
                },
                parse::joinv2::ValueType::Array => {
                    match t.parse::<usize>() {
                        Ok(index) => {
                            extraJson = &extraJson[index];
                        },
                        Err(err) => {
                            println!("[Error] index change error, err: {}", err);
                            return;
                        }
                    }
                    append(extraJson, result);
                },
                _ => {}
            }
        } else {
            match valueType {
                parse::joinv2::ValueType::Char => {
                    match t.parse::<char>() {
                        Ok(c) => {
                            result.push(c);
                        },
                        Err(err) => {
                            println!("[Error] index change error, err: {}", err);
                            return;
                        }
                    }
                },
                parse::joinv2::ValueType::Var => {
                    if t == keyword_platform {
                        result.push_str(platform);
                    } else if t == keyword_target {
                        result.push_str(target);
                    } else if t == keyword_config {
                        result.push_str(configPath);
                    } else if t == keyword_version {
                        result.push_str(version);
                    }
                }
                _ => {}
            }
        }
    })
}

#[derive(Default, Debug)]
pub struct CResult {
    pub libpath: Option<String>,
    pub include: Option<String>
}

pub fn get(runArgs: &structs::param::CRunArgs, configPath: &str, version: &str, libPackage: &config::libconfig::CPackage, libVesion: &config::libconfig::CVersion) -> Option<CResult> {
    /*
    ** Determine the type of the extension field,
    ** if it is a json type, it will be parsed
    */
    let extraType = match &runArgs.extraType {
        Some(e) => e,
        None => {
            extra_type_string
        }
    };
    let extra = match &runArgs.extra {
        Some(e) => e,
        None => {
            ""
        }
    };
    let runPlatform  = match &runArgs.platform {
        Some(p) => p,
        None => {
            platform_default
        }
    };
    let runTarget = match &runArgs.target {
        Some(t) => t,
        None => {
            target_default
        }
    };
    let mut extraJson = JsonValue::Null;
    if extraType == extra_type_string {
    } else if extraType == extra_type_json {
        extraJson = match json::parse(&extra) {
            Ok(e) => e,
            Err(err) => {
                return None;
            }
        };
    }
    let mut extraJsonClone = extraJson.clone();
    /*
    ** Firstly find the splicing rules in each version.
    ** If it does not exist, look for the overall splicing rules.
    ** If none of them exist, set the default value.
    */
    let mut attributes = match &libVesion.attributes {
        Some(a) => {
            let libpathRule = match &a.libpathRule {
                Some(d) => d,
                None => {
                    libpath_rule_default
                }
            };
            let includeRule = match &a.includeRule {
                Some(r) => r,
                None => {
                    include_rule_default
                }
            };
            config::libconfig::CAttributes{
                platform: None,
                debug: None,
                release: None,
                rule: None,
                libpathRule: Some(libpathRule.to_string()),
                includeRule: Some(includeRule.to_string())
            }
        },
        None => {
            let libpathRule = match &libPackage.libpathRule {
                Some(d) => d,
                None => {
                    libpath_rule_default
                }
            };
            let includeRule = match &libPackage.includeRule {
                Some(r) => r,
                None => {
                    include_rule_default
                }
            };
            config::libconfig::CAttributes{
                platform: None,
                debug: None,
                release: None,
                rule: None,
                libpathRule: Some(libpathRule.to_string()),
                includeRule: Some(includeRule.to_string())
            }
        }
    };
    /*
    ** Parse each field in the attributes,
    ** and splice according to the parameters provided by the application,
    ** and update each field of the attributes with the result.
    */
    let mut libpathValue = String::new();
    // println!("attr: {:?}", &attributes);
    if let Err(err) = join(&attributes.libpathRule.unwrap(), configPath, version, runPlatform, runTarget, &mut extraJson, &mut extraJsonClone, &mut libpathValue) {
        println!("[Error] join parse error, err: {}", err);
        return None;
    };
    println!("{:?}", &libpathValue);
    let mut includeValue = String::new();
    if let Err(err) = join(&attributes.includeRule.unwrap(), configPath, version, runPlatform, runTarget, &mut extraJson, &mut extraJsonClone, &mut includeValue) {
        println!("[Error] join parse error, err: {}", err);
        return None;
    };
    let mut r = CResult::default();
    /*
    ** Get absolute path
    */
    match Path::new(&libpathValue).canonicalize() {
        Ok(p) => {
            match p.to_str() {
                Some(s) => {
                    if cfg!(target_os="windows"){
                        let t = s.trim_left_matches(r#"\\?\"#).replace(r#"\"#, r#"\\"#);
                        r.libpath = Some(t);
                    } else {
                        r.libpath = Some(s.to_string());
                    }
                },
                None => {
                    println!("[Error] libpath abs to_str error");
                }
            }
        },
        Err(err) => {
            println!("[Error] libpath rule join path error, libpathValue: {}", &libpathValue);
        }
    };
    match Path::new(&includeValue).canonicalize() {
        Ok(p) => {
            match p.as_os_str().to_str() {
                Some(s) => {
                    r.include = Some(s.to_string());
                },
                None => {
                    println!("[Erorr] include abs to_str error");
                }
            }
        },
        Err(err) => {
            println!("[Error] include rule kjoin path error");
        }
    }
    Some(r)
}
