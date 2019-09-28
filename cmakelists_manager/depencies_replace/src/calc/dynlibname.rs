use crate::parse;
use crate::config;

use json::{JsonValue};

const platform_default: &str = "${FILE_PREFIX}";
const debug_default: &str = "_d";
const release_default: &str = "";
const rule_default: &str = "$name$version$platform$d_r";

const extra_type_string: &str = "string";
const extra_type_json: &str = "json";

const keyword_extra: &str = "extra";
const keyword_name: &str = "name";
const keyword_version: &str = "version";
const keyword_platform: &str = "platform";
const keyword_d_r: &str = "d_r";

struct CJoin<'a> {
    extraJsonValue: &'a mut JsonValue,
    result: String
}

impl<'a> parse::join::IJoin for CJoin<'a> {
    fn on_ch(&mut self, c: char) {
        self.result.push(c);
    }

    fn on_var(&mut self, var: &str) {
        if var == keyword_extra {
        }
    }

    fn on_field(&mut self, var: &str, field: &str) {
        if var == keyword_extra {
            // self.extraJsonValue = &mut self.extraJsonValue[field];
        }
    }

    fn on_arr(&mut self, var: &str, index: &u32) {
    }
}

impl<'a> CJoin<'a> {
    fn new(extraJsonValue: &mut JsonValue) -> CJoin {
        CJoin{
            extraJsonValue: extraJsonValue,
            result: String::new()
        }
    }
}

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

fn join<'a, 'b:'a>(content: &'a str, platform: &str, mut extraJson: &'a JsonValue, mut extraJsonClone: &'b JsonValue, result: &mut String) -> Result<(), &'a str> {
    parse::join::parse_with_fn(content
    , &mut |var: &str, t: &str, valueType: parse::join::ValueType| {
        // on_field
        if var == keyword_extra {
            match valueType {
                parse::join::ValueType::Object => {
                    extraJson = &extraJson[t];
                    append(extraJson, result);
                },
                parse::join::ValueType::Array => {
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
                parse::join::ValueType::Var => {
                    extraJson = &extraJsonClone;
                },
                _ => {}
            }
        } else {
            match valueType {
                parse::join::ValueType::Char => {
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
                parse::join::ValueType::Var => {
                    if var == keyword_platform {
                        result.push_str(platform);
                    }
                }
                _ => {}
            }
        }
    })
}

#[derive(Default, Debug)]
pub struct CResult {
    debug: Option<String>,
    release: Option<String>,
    dr: Option<String>
}

pub fn get(exeParam: &parse::git_lib::CGitLib, version: &str, libPackage: &config::libconfig::CPackage, libVesion: &config::libconfig::CVersion) -> Option<CResult> {
    /*
    ** Determine the type of the extension field,
    ** if it is a json type, it will be parsed
    */
    let extraType = match &exeParam.extraType {
        Some(e) => e,
        None => {
            extra_type_string
        }
    };
    let extra = match &exeParam.extra {
        Some(e) => e,
        None => {
            ""
        }
    };
    let exePlatform  = match &exeParam.platform {
        Some(p) => p,
        None => {
            platform_default
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
            let platform = match &a.platform {
                Some(p) => p,
                None => {
                    platform_default
                }
            };
            let debug = match &a.debug {
                Some(d) => d,
                None => {
                    debug_default
                }
            };
            let release = match &a.release {
                Some(r) => r,
                None => {
                    release_default
                }
            };
            let rule = match &a.rule {
                Some(r) => r,
                None => {
                    rule_default
                }
            };
            config::libconfig::CAttributes{
                platform: Some(platform.to_string()),
                debug: Some(debug.to_string()),
                release: Some(release.to_string()),
                rule: Some(rule.to_string())
            }
        },
        None => {
            let platform = match &libPackage.platform {
                Some(p) => p,
                None => {
                    platform_default
                }
            };
            let debug = match &libPackage.debug {
                Some(d) => d,
                None => {
                    debug_default
                }
            };
            let release = match &libPackage.release {
                Some(r) => r,
                None => {
                    release_default
                }
            };
            let rule = match &libPackage.rule {
                Some(r) => r,
                None => {
                    rule_default
                }
            };
            config::libconfig::CAttributes{
                platform: Some(platform.to_string()),
                debug: Some(debug.to_string()),
                release: Some(release.to_string()),
                rule: Some(rule.to_string())
            }
        }
    };
    /*
    ** Parse each field in the attributes,
    ** and splice according to the parameters provided by the application,
    ** and update each field of the attributes with the result.
    */
    let mut platformValue = String::new();
    if let Err(err) = join(&attributes.platform.unwrap(), exePlatform, &mut extraJson, &mut extraJsonClone, &mut platformValue) {
        println!("[Error] join parse error, err: {}", err);
        return None;
    };
    let mut debugValue = String::new();
    if let Err(err) = join(&attributes.debug.unwrap(), exePlatform, &mut extraJson, &mut extraJsonClone, &mut debugValue) {
        println!("[Error] join parse error, err: {}", err);
        return None;
    };
    let mut releaseValue = String::new();
    if let Err(err) = join(&attributes.release.unwrap(), exePlatform, &mut extraJson, &mut extraJsonClone, &mut releaseValue) {
        println!("[Error] join parse error, err: {}", err);
        return None;
    };
    /*
    ** Parse the rules and then combine the rules
    */
    let mut result = CResult::default();
    let mut debugName = String::new();
    let mut releaseName = String::new();
    parse::rule::parse(&attributes.rule.unwrap(), &mut |t: &str, valueType: parse::rule::ValueType| {
        match valueType {
            parse::rule::ValueType::Var => {
                if t == keyword_name {
                    debugName.push_str(&libPackage.name);
                    releaseName.push_str(&libPackage.name);
                } else if t == keyword_platform {
                    debugName.push_str(&platformValue);
                    releaseName.push_str(&platformValue);
                } else if t == keyword_version {
                    debugName.push_str(version);
                    releaseName.push_str(version);
                } else if t == keyword_d_r {
                    debugName.push_str(&debugValue);
                    releaseName.push_str(&releaseValue);
                }
            },
            parse::rule::ValueType::Char => {
                debugName.push_str(t);
                releaseName.push_str(t);
            }
        }
    });
    if debugName == releaseName {
        result.dr = Some(releaseName);
    } else {
        result.debug = Some(debugName);
        result.release = Some(releaseName);
    }
    Some(result)
}
