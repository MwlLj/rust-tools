use crate::parse::{self, joinv2::ParseMode, joinv2::ValueCode, joinv2::ValueError};
use crate::config;
use crate::structs;

use json::{JsonValue};
use path_abs::PathAbs;

use std::path::Path;

const libpath_rule_default: &str = "`var:'config'`/lib/`var:'version'`/`var:'target'`/`var:'platform'`";
const include_rule_default: &str = "`var:'config'`/include/`var:'version'";
const platform_default: &str = "";
const target_default: &str = "";

const extra_type_string: &str = "string";
const extra_type_json: &str = "json";

const keyword_extra: &str = "extra";
const keyword_version: &str = "version";
const keyword_target: &str = "target";
const keyword_platform: &str = "platform";
const keyword_config: &str = "config";

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

fn join<'a, 'b:'a>(content: &'a str, configPath: &str, version: &str, platform: &str, target: &str, mut extraJson: &'a JsonValue, mut extraJsonClone: &'b JsonValue, result: &mut String) -> Result<(), &'a str> {
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
                } else if t == keyword_config {
                    match parseMode {
                        parse::joinv2::ParseMode::JudgeSub => {
                            // lastString = configPath.to_string();
                            lastStrings.push(configPath.to_string());
                        },
                        parse::joinv2::ParseMode::Normal => {
                            result.push_str(configPath);
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
                            println!("{:?}", &lastStrings);
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
                result.push_str(t);
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
    pub libpath: Option<String>,
    pub include: Option<String>
}

pub fn get(exeParam: &parse::git_lib::CParam, configPath: &str, version: &str, libPackage: &config::libconfig::CPackage, libVesion: &config::libconfig::CVersion) -> Option<CResult> {
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
    if let Err(err) = join(&attributes.libpathRule.unwrap(), configPath, version, platform, target, &mut extraJson, &mut extraJsonClone, &mut libpathValue) {
        println!("[Error] join parse error, err: {}", err);
        return None;
    };
    // println!("###### {:?}", &libpathValue);
    let mut includeValue = String::new();
    if let Err(err) = join(&attributes.includeRule.unwrap(), configPath, version, platform, target, &mut extraJson, &mut extraJsonClone, &mut includeValue) {
        println!("[Error] join parse error, err: {}", err);
        return None;
    };
    // println!("###### {:?}", &includeValue);
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
                    if cfg!(target_os="windows"){
                        let t = s.trim_left_matches(r#"\\?\"#).replace(r#"\"#, r#"\\"#);
                        r.include = Some(t);
                    } else {
                        r.include = Some(s.to_string());
                    }
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

pub fn get1(exeParam: &parse::git_lib::CGitLib, configPath: &str, version: &str, libPackage: &config::libconfig::CPackage, libVesion: &config::libconfig::CVersion) -> Option<CResult> {
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
    if let Err(err) = join(&attributes.libpathRule.unwrap(), configPath, version, platform, target, &mut extraJson, &mut extraJsonClone, &mut libpathValue) {
        println!("[Error] join parse error, err: {}", err);
        return None;
    };
    println!("{:?}", &libpathValue);
    let mut includeValue = String::new();
    if let Err(err) = join(&attributes.includeRule.unwrap(), configPath, version, platform, target, &mut extraJson, &mut extraJsonClone, &mut includeValue) {
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

#[cfg(test)]
mod test {
	use super::*;

	#[test]
    #[ignore]
	fn joinTest() {
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
			"#, ".", "1.0.0", "", "", &mut extraJson, &mut extraJsonClone, &mut result);
        println!("{:?}", result);
	}
}
