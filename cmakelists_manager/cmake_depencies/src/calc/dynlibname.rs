use crate::parse::{self, joinv2::ParseMode, joinv2::ValueCode, joinv2::ValueError};
use crate::config;

use json::{JsonValue};
use serde_derive::{Serialize, Deserialize};

use std::collections::HashMap;

const platform_default: &str = "win64";
const enable_default: &str = "true";
const libname_enable_default: &str = "true";
const debug_default: &str = "_d";
const release_default: &str = "";
const rule_default: &str = "$name.$version.$platform$d_r";
const target_default: &str = "";

const extra_type_string: &str = "string";
const extra_type_json: &str = "json";

const keyword_extra: &str = "extra";
const keyword_name: &str = "name";
const keyword_version: &str = "version";
const keyword_platform: &str = "platform";
const keyword_target: &str = "target";
const keyword_d_r: &str = "d_r";

const cmake_keyword_debug: &str = "debug";
const cmake_keyword_release: &str = "optimized";

const enable_true: &str = "true";
const enable_false: &str = "false";

fn jsonToString(jsonValue: &JsonValue) -> String {
    let mut r = String::new();
    match jsonValue {
        JsonValue::Null => {
        },
        JsonValue::Short(v) => {
            r = v.to_string();
            // println!("short: {}, result: {}", v, result);
        },
        JsonValue::String(v) => {
            r = v.to_string();
        },
        JsonValue::Number(v) => {
            r = v.to_string();
        },
        JsonValue::Boolean(v) => {
            r = v.to_string();
        },
        _ => {}
    }
    r
}

fn append(jsonValue: &JsonValue, result: &mut String) -> String {
    let r = jsonToString(jsonValue);
    result.push_str(&r);
    r
}

fn join<'a, 'b:'a>(content: &'a str, version: &str, platform: &str, target: &str, mut extraJson: &'a JsonValue, mut extraJsonClone: &'b JsonValue, result: &mut String) -> Result<(), &'a str> {
    let mut lastString = String::new();
    let mut lastStrings = Vec::new();
    let mut lastSymbol = String::new();
    let mut lastIsJudgeSymbol = false;
    parse::joinv2::parse(content
    , &mut |t: &str, parseMode: &parse::joinv2::ParseMode, valueType: parse::joinv2::ValueType| -> Result<ValueCode, ValueError> {
        // println!("t: {}, parseMode: {:?}, valueType: {:?}", t, parseMode, valueType);
        match valueType {
            parse::joinv2::ValueType::Start => {
                extraJson = &extraJsonClone;
                extraJson = &extraJson[t];
            },
            parse::joinv2::ValueType::Object => {
                extraJson = &extraJson[t];
            },
            parse::joinv2::ValueType::End
                | parse::joinv2::ValueType::EndAfterArray => {
                match valueType {
                    parse::joinv2::ValueType::End => {
                        extraJson = &extraJson[t];
                    },
                    _ => {}
                }
                match parseMode {
                    parse::joinv2::ParseMode::JudgeSub => {
                        // lastString = jsonToString(extraJson);
                        lastStrings.push(jsonToString(extraJson));
                    },
                    parse::joinv2::ParseMode::Normal => {
                        append(extraJson, result);
                    },
                    _ => {}
                }
            },
            parse::joinv2::ValueType::Array => {
                match t.parse::<usize>() {
                    Ok(index) => {
                        extraJson = &extraJson[index];
                    },
                    Err(err) => {
                        println!("[Error] index change error, err: {}", err);
                        return Err(ValueError::Unknow);
                    }
                }
            },
            parse::joinv2::ValueType::Char => {
                match t.parse::<char>() {
                    Ok(c) => {
                        match parseMode {
                            parse::joinv2::ParseMode::Normal => {
                                result.push(c);
                            },
                            _ => {}
                        }
                    },
                    Err(err) => {
                        println!("[Error] index change error, err: {}", err);
                        return Err(ValueError::Unknow);
                    }
                }
            },
            parse::joinv2::ValueType::Var => {
                if t == keyword_platform {
                    match parseMode {
                        parse::joinv2::ParseMode::JudgeSub => {
                            // lastString = platform.to_string();
                            lastStrings.push(platform.to_string());
                        },
                        parse::joinv2::ParseMode::Normal => {
                            result.push_str(platform);
                        },
                        _ => {}
                    }
                } else if t == keyword_target {
                    match parseMode {
                        parse::joinv2::ParseMode::JudgeSub => {
                            // lastString = target.to_string();
                            lastStrings.push(target.to_string());
                        },
                        parse::joinv2::ParseMode::Normal => {
                            result.push_str(target);
                        },
                        _ => {}
                    }
                } else if t == keyword_version {
                    match parseMode {
                        parse::joinv2::ParseMode::JudgeSub => {
                            // lastString = version.to_string();
                            lastStrings.push(version.to_string());
                        },
                        parse::joinv2::ParseMode::Normal => {
                            result.push_str(version);
                        },
                        _ => {}
                    }
                }
            },
            parse::joinv2::ValueType::Condition(condType) => {
                // println!("{:?}, {}", t, &lastString);
                match condType {
                    parse::joinv2::CondType::Symbol => {
                        if t != "&&" && t != "||" {
                            lastIsJudgeSymbol = true;
                        }
                        lastSymbol = t.to_string();
                    },
                    parse::joinv2::CondType::Else => {
                        lastIsJudgeSymbol = false;
                    },
                    _ => {
                        // json / str / var / judge
                        let mut code = ValueCode::Normal;
                        if lastIsJudgeSymbol {
                            // println!("{:?}", &lastStrings);
                            // compare
                            if lastSymbol == "==" {
                                if lastStrings.len() < 2 {
                                    println!("equal error on both sides");
                                    return Err(ValueError::Unknow);
                                }
                                if lastStrings[0] == lastStrings[1] {
                                    code = ValueCode::DonotContinueJudge;
                                }
                                lastStrings.clear();
                            } else if lastSymbol == "!=" {
                                if lastStrings.len() < 2 {
                                    println!("unequal error on both sides");
                                    return Err(ValueError::Unknow);
                                }
                                if lastStrings[0] != lastStrings[1] {
                                    code = ValueCode::DonotContinueJudge;
                                }
                                lastStrings.clear();
                            }
                        }
                        lastIsJudgeSymbol = false;
                        return Ok(code);
                    }
                }
            },
            parse::joinv2::ValueType::JudgeBody => {
                let mut extJson = extraJson.clone();
                let mut extJsonClone = extJson.clone();
                let mut body = String::new();
                if let Ok(()) = join(t, version, platform, target, &mut extJson, &mut extJsonClone, &mut body) {
                    result.push_str(&body);
                } else {
                    result.push_str(t);
                }
            },
            parse::joinv2::ValueType::Str => {
                match parseMode {
                    ParseMode::Normal => {
                        result.push_str(t);
                    },
                    ParseMode::JudgeSub => {
                        lastStrings.push(t.to_string());
                    },
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(ValueCode::Normal)
    })
}

#[derive(Default, Debug)]
pub struct CResult {
    pub debug: Option<String>,
    pub release: Option<String>,
    pub dr: Option<String>
}

#[derive(Debug)]
pub enum NameType {
    Full,
    Debug,
    Release
}

impl Default for NameType {
    fn default() -> Self {
        NameType::Full
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CNameResult {
    pub fullName: String,
    pub debugName: String,
    pub releaseName: String
}

pub fn get(library: &parse::git_librarys::CGitLibrarys, exeParam: &parse::git_lib::CParam, version: &str, libs: &Vec<String>, libPackage: &config::libconfig::CPackage, libVesion: &config::libconfig::CVersion) -> Option<Vec<String>> {
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
    let platform  = match &exeParam.platform {
        Some(p) => p,
        None => {
            platform_default
        }
    };
    let target = match &exeParam.target {
        Some(t) => t,
        None => {
            target_default
        }
    };
    let enable = match &library.enable {
        Some(t) => t,
        None => {
            enable_default
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
            let target = match &a.target {
                Some(t) => t,
                None => {
                    target_default
                }
            };
            let libnameEnable = match &a.libnameEnable {
                Some(e) => {
                    match &library.libnameEnable {
                        Some(en) => {
                            en
                        },
                        None => {
                            e
                        }
                    }
                },
                None => {
                    match &library.libnameEnable {
                        Some(en) => {
                            en
                        },
                        None => {
                            libname_enable_default
                        }
                    }
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
                target: Some(target.to_string()),
                includeEnable: None,
                libpathEnable: None,
                libnameEnable: Some(libnameEnable.to_string()),
                debug: Some(debug.to_string()),
                release: Some(release.to_string()),
                rule: Some(rule.to_string()),
                libpathRule: None,
                includeRule: None,
                map: a.map.clone()
            }
        },
        None => {
            let platform = match &libPackage.platform {
                Some(p) => p,
                None => {
                    platform_default
                }
            };
            let target = match &libPackage.platform {
                Some(t) => t,
                None => {
                    target_default
                }
            };
            let libnameEnable = match &libPackage.libnameEnable {
                Some(e) => {
                    match &library.libnameEnable {
                        Some(en) => {
                            en
                        },
                        None => {
                            e
                        }
                    }
                },
                None => {
                    match &library.libnameEnable {
                        Some(en) => {
                            en
                        },
                        None => {
                            libname_enable_default
                        }
                    }
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
                target: Some(target.to_string()),
                includeEnable: None,
                libpathEnable: None,
                libnameEnable: Some(libnameEnable.to_string()),
                debug: Some(debug.to_string()),
                release: Some(release.to_string()),
                rule: Some(rule.to_string()),
                libpathRule: None,
                includeRule: None,
                map: libPackage.map.clone()
            }
        }
    };
    /*
    ** Determine whether it is enabled
    */
    let mut enableValue = String::new();
    if let Err(err) = join(enable, version, platform, target, &mut extraJson, &mut extraJsonClone, &mut enableValue) {
        println!("[Error] join parse error, err: {}", err);
        return None;
    };
    if enableValue == enable_false {
        return Some(Vec::new());
    }
    /*
    ** Determine whether it is enabled
    */
    let mut libnameEnableValue = String::new();
    let le = match &attributes.libnameEnable {
        Some(e) => e,
        None => {
            panic!("libnameEnable is not exist");
        }
    };
    if let Err(err) = join(le, version, platform, target, &mut extraJson, &mut extraJsonClone, &mut libnameEnableValue) {
        println!("[Error] join parse error, err: {}", err);
        return None;
    };
    if libnameEnableValue == enable_false {
        return Some(Vec::new());
    }
    /*
    ** Parse each field in the attributes,
    ** and splice according to the parameters provided by the application,
    ** and update each field of the attributes with the result.
    */
    let mut platformValue = String::new();
    let p = match &attributes.platform {
        Some(p) => p,
        None => {
            panic!("platform is not exist");
        }
    };
    if let Err(err) = join(p, version, platform, target, &mut extraJson, &mut extraJsonClone, &mut platformValue) {
        println!("[Error] join parse error, err: {}", err);
        return None;
    };
    let mut targetValue = String::new();
    let t = match &attributes.target {
        Some(t) => t,
        None => {
            panic!("target is not exist");
        }
    };
    if let Err(err) = join(t, version, platform, target, &mut extraJson, &mut extraJsonClone, &mut targetValue) {
        println!("[Error] join parse error, err: {}", err);
        return None;
    };
    // println!("#####, {}, {}", p, &platformValue);
    let mut debugValue = String::new();
    let d = match &attributes.debug {
        Some(d) => d,
        None => {
            panic!("debug is not exist");
        }
    };
    if let Err(err) = join(d, version, platform, target, &mut extraJson, &mut extraJsonClone, &mut debugValue) {
        println!("[Error] join parse error, err: {}", err);
        return None;
    };
    let mut releaseValue = String::new();
    let r = match &attributes.release {
        Some(r) => r,
        None => {
            panic!("release id not exist");
        }
    };
    if let Err(err) = join(r, version, platform, target, &mut extraJson, &mut extraJsonClone, &mut releaseValue) {
        println!("[Error] join parse error, err: {}", err);
        return None;
    };
    let mut maps = HashMap::new();
    match &attributes.map {
        Some(m) => {
            for (k, v) in m {
                let mut value = String::new();
                if let Err(err) = join(v, version, platform, target, &mut extraJson, &mut extraJsonClone, &mut value) {
                    println!("[Error] join parse error, err: {}", err);
                    continue;
                };
                maps.insert(k.to_string(), value);
            }
        },
        None => {
        }
    }
    /*
    ** Parse the rules and then combine the rules
    */
    let ru = match &attributes.rule {
        Some(ru) => ru,
        None => {
            panic!("rule is not exist");
        }
    };
    let mut results = Vec::new();
    for lib in libs {
        let mut debugName = String::new();
        let mut releaseName = String::new();
        parse::rule::parse(ru, &mut |t: &str, valueType: parse::rule::ValueType| {
            match valueType {
                parse::rule::ValueType::Var => {
                    if t == keyword_name {
                        debugName.push_str(&lib);
                        releaseName.push_str(&lib);
                    } else if t == keyword_platform {
                        debugName.push_str(&platformValue);
                        releaseName.push_str(&platformValue);
                    } else if t == keyword_version {
                        debugName.push_str(version);
                        releaseName.push_str(version);
                    } else if t == keyword_d_r {
                        debugName.push_str(&debugValue);
                        releaseName.push_str(&releaseValue);
                    } else {
                        match maps.get(t) {
                            Some(v) => {
                                debugName.push_str(v);
                                releaseName.push_str(v);
                            },
                            None => {
                            }
                        }
                    }
                },
                parse::rule::ValueType::Char => {
                    debugName.push_str(t);
                    releaseName.push_str(t);
                }
            }
        });
        let mut fullName = String::new();
        if debugName == releaseName {
            fullName = releaseName.clone();
        } else {
            // debug
            fullName.push_str(cmake_keyword_debug);
            fullName.push_str(" ");
            fullName.push_str(&debugName);
            fullName.push_str(" ");
            // release
            fullName.push_str(cmake_keyword_release);
            fullName.push_str(" ");
            fullName.push_str(&releaseName);
        }
        let s = serde_json::to_string(&CNameResult{
            fullName: fullName,
            debugName: debugName,
            releaseName: releaseName
        }).expect("serde_json CNameResult to_string error");
        results.push(s);
    }
    Some(results)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn joinJudgeTest() {
        let mut extraJson = match json::parse(&r#"
            {
                "extra": {
                    "name": "win32",
                    "objs": ["one", "two", "third"],
                    "dr": "release"
                }
            }
            "#) {
            Ok(e) => e,
            Err(err) => {
                return;
            }
        };
        let mut extraJsonClone = extraJson.clone();
        let mut result = String::new();
        join(r#"
        `judge:"
        if json:'extra.name' == str:'win64' {
            64
        } elseif json:'extra.dr' == str:'debug' {
            _d
        } else {
            _
        }
        "`
            "#, "1.0.0", "", "", &mut extraJson, &mut extraJsonClone, &mut result);
        println!("{:?}", result);
    }

    #[test]
    #[ignore]
    fn joinJsonTest() {
        let mut extraJson = match json::parse(&r#"
            {
                "extra": {
                    "name": "win32",
                    "objs": ["one", "two", "third"],
                    "dr": "release"
                },
                "objs": ["1", "2", "3"]
            }
            "#) {
            Ok(e) => e,
            Err(err) => {
                return;
            }
        };
        let mut extraJsonClone = extraJson.clone();
        let mut result = String::new();
        join("abcd.`json:'extra.name'`.`json:'extra.objs[0]'`.`json:'objs[1]'`"
            , "1.0.0", "", "", &mut extraJson, &mut extraJsonClone, &mut result);
        println!("{:?}", result);
    }
}

