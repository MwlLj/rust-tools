use crate::parse::{self, git_lib, joinv2::ParseMode, joinv2::ValueCode, joinv2::ValueError};
use crate::config;
use crate::structs;
use crate::search;
use git_lib::ParamType;
use path::pathconvert;

use json::{JsonValue};
use path_abs::PathAbs;

use std::path::Path;
use std::collections::HashMap;
use std::fs;

const libpath_rule_default: &str = "`var:'config'`/`var:'version'`/lib/`var:'platform'`_`var:'target'`";
const binpath_rule_default: &str = "`var:'config'`/`var:'version'`/bin/`var:'platform'`_`var:'target'`";
const include_rule_default: &str = "`var:'config'`/`var:'version'`/include";
const include_rules_default: [&str; 0] = [];
const platform_default: &str = "";
const target_default: &str = "";
const enable_default: &str = "true";
const libpath_enable_default: &str = "true";
const include_enable_default: &str = "true";

const extra_type_string: &str = "string";
const extra_type_json: &str = "json";

const keyword_name: &str = "name";
const keyword_extra: &str = "extra";
const keyword_version: &str = "version";
const keyword_target: &str = "target";
const keyword_platform: &str = "platform";
const keyword_config: &str = "config";

const enable_true: &str = "true";
const enable_false: &str = "false";

const sub_split: &str = ",";

const bin_dir_default: &str = "${CMAKE_CURRENT_BINARY_DIR}";

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

fn join<'a, 'b:'a>(content: &'a str, name: &str, configPath: &str, version: &str, platform: &str, target: &str, map: &Option<HashMap<String, String>>, mut extraJson: &'a JsonValue, mut extraJsonClone: &'b JsonValue, result: &mut String) -> Result<(), &'a str> {
    let mut flag: u8 = 0;
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
                // println!("{:?}", extraJson);
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
                } else if t == keyword_name {
                    match parseMode {
                        parse::joinv2::ParseMode::JudgeSub => {
                            // lastString = target.to_string();
                            lastStrings.push(name.to_string());
                        },
                        parse::joinv2::ParseMode::Normal => {
                            result.push_str(name);
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
                } else {
                    if let Some(map) = map {
                        match map.get(t) {
                            Some(v) => {
                                match parseMode {
                                    parse::joinv2::ParseMode::JudgeSub => {
                                        // lastString = configPath.to_string();
                                        lastStrings.push(v.to_string());
                                    },
                                    parse::joinv2::ParseMode::Normal => {
                                        result.push_str(v);
                                    },
                                    _ => {}
                                }
                            },
                            None => {
                            }
                        }
                    };
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
    pub binpath: Option<String>,
    pub include: Option<Vec<String>>
}

pub fn get(library: &parse::git_librarys::CGitLibrarys, exeParam: &parse::git_lib::CParam, configPath: &str, cmakeDir: &str, version: &str, libPackage: &config::libconfig::CPackage, libVesion: &config::libconfig::CVersion) -> Option<CResult> {
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
        Some(e) => e,
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
            let libpathRule = match &a.libpathRule {
                Some(d) => d,
                None => {
                    libpath_rule_default
                }
            };
            let binpathRule = match &a.binpathRule {
                Some(d) => d,
                None => {
                    binpath_rule_default
                }
            };
            let includeRule = match &a.includeRule {
                Some(r) => r,
                None => {
                    include_rule_default
                }
            };
            let includeRulesTmp = Vec::new();
            let includeRules = match &a.includeRules {
                Some(r) => r,
                None => {
                    &includeRulesTmp
                }
            };
            let includeEnable = match &a.includeEnable {
                Some(e) => {
                    match &library.includeEnable {
                        Some(en) => {
                            en
                        },
                        None => {
                            e
                        }
                    }
                },
                None => {
                    match &library.includeEnable {
                        Some(en) => {
                            en
                        },
                        None => {
                            include_enable_default
                        }
                    }
                }
            };
            let libpathEnable = match &a.libpathEnable {
                Some(e) => {
                    match &library.libpathEnable {
                        Some(en) => {
                            en
                        },
                        None => {
                            e
                        }
                    }
                },
                None => {
                    match &library.libpathEnable {
                        Some(en) => {
                            en
                        },
                        None => {
                            libpath_enable_default
                        }
                    }
                }
            };
            let includeSubs = match &a.includeSubs {
                Some(s) => s,
                None => {
                    &libPackage.name
                }
            };
            config::libconfig::CAttributes{
                platform: None,
                target: None,
                subs: None,
                dllSubs: None,
                includeSubs: Some(includeSubs.to_string()),
                includeEnable: Some(includeEnable.to_string()),
                libpathEnable: Some(libpathEnable.to_string()),
                libnameEnable: None,
                debug: None,
                release: None,
                rule: None,
                dllRule: None,
                libpathRule: Some(libpathRule.to_string()),
                binpathRule: Some(binpathRule.to_string()),
                includeRule: Some(includeRule.to_string()),
                includeRules: Some(includeRules.clone()),
                binCopyMode: None,
                map: a.map.clone()
            }
        },
        None => {
            let libpathRule = match &libPackage.libpathRule {
                Some(d) => d,
                None => {
                    libpath_rule_default
                }
            };
            let binpathRule = match &libPackage.binpathRule {
                Some(d) => d,
                None => {
                    binpath_rule_default
                }
            };
            let includeRule = match &libPackage.includeRule {
                Some(r) => r,
                None => {
                    include_rule_default
                }
            };
            let includeRulesTmp = Vec::new();
            let includeRules = match &libPackage.includeRules {
                Some(r) => r,
                None => {
                    &includeRulesTmp
                }
            };
            let includeEnable = match &libPackage.includeEnable {
                Some(e) => {
                    match &library.includeEnable {
                        Some(en) => {
                            en
                        },
                        None => {
                            e
                        }
                    }
                },
                None => {
                    match &library.includeEnable {
                        Some(en) => {
                            en
                        },
                        None => {
                            include_enable_default
                        }
                    }
                }
            };
            let libpathEnable = match &libPackage.libpathEnable {
                Some(e) => {
                    match &library.libpathEnable {
                        Some(en) => {
                            en
                        },
                        None => {
                            e
                        }
                    }
                },
                None => {
                    match &library.libpathEnable {
                        Some(en) => {
                            en
                        },
                        None => {
                            libpath_enable_default
                        }
                    }
                }
            };
            let includeSubs = match &libPackage.includeSubs {
                Some(s) => s,
                None => {
                    &libPackage.name
                }
            };
            config::libconfig::CAttributes{
                platform: None,
                target: None,
                subs: None,
                dllSubs: None,
                includeSubs: Some(includeSubs.to_string()),
                includeEnable: Some(includeEnable.to_string()),
                libpathEnable: Some(libpathEnable.to_string()),
                libnameEnable: None,
                debug: None,
                release: None,
                rule: None,
                dllRule: None,
                libpathRule: Some(libpathRule.to_string()),
                binpathRule: Some(binpathRule.to_string()),
                includeRule: Some(includeRule.to_string()),
                includeRules: Some(includeRules.clone()),
                binCopyMode: None,
                map: libPackage.map.clone()
            }
        }
    };
    let mut r = CResult::default();
    /*
    ** Determine whether it is enabled
    */
    let mut enableValue = String::new();
    if let Err(err) = join(enable, &libPackage.name, configPath, version, platform, target, &attributes.map, &mut extraJson, &mut extraJsonClone, &mut enableValue) {
        println!("[Error] join parse error, err: {}", err);
        return None;
    };
    if enableValue == enable_false {
        return Some(r);
    }
    let mut libpathEnableValue = String::new();
    if let Err(err) = join(&attributes.libpathEnable.expect("libpathEnable is null"), &libPackage.name, configPath, version, platform, target, &attributes.map, &mut extraJson, &mut extraJsonClone, &mut libpathEnableValue) {
        println!("[Error] join parse error, err: {}", err);
        return None;
    };
    let mut includeEnableValue = String::new();
    if let Err(err) = join(&attributes.includeEnable.expect("includeEnable is null"), &libPackage.name, configPath, version, platform, target, &attributes.map, &mut extraJson, &mut extraJsonClone, &mut includeEnableValue) {
        println!("[Error] join parse error, err: {}", err);
        return None;
    };
    /*
    ** Parse each field in the attributes,
    ** and splice according to the parameters provided by the application,
    ** and update each field of the attributes with the result.
    */
    let mut libpathValue = String::new();
    // println!("attr: {:?}", &attributes);
    if let Err(err) = join(&attributes.libpathRule.unwrap(), &libPackage.name, configPath, version, platform, target, &attributes.map, &mut extraJson, &mut extraJsonClone, &mut libpathValue) {
        println!("[Error] join parse error, err: {}", err);
        return None;
    };
    let mut binpathValue = String::new();
    if let Err(err) = join(&attributes.binpathRule.unwrap(), &libPackage.name, configPath, version, platform, target, &attributes.map, &mut extraJson, &mut extraJsonClone, &mut binpathValue) {
        println!("[Error] join parse error, err: {}", err);
        return None;
    };
    // println!("###### {:?}", &libpathValue);
    let includeRule = match attributes.includeRule {
        Some(r) => r,
        None => {
            panic!("attribute includeRule is none");
        }
    };
    let mut includeRules = match attributes.includeRules {
        Some(r) => r,
        None => {
            panic!("attribute includeRules is none");
        }
    };
    if includeRules.len() == 0 {
        includeRules.push(includeRule);
    }
    let includeSubs = match attributes.includeSubs {
        Some(s) => s,
        None => {
            panic!("attributes include subs is none");
        }
    };
    let includeSubVec = includeSubs.split(sub_split);
    let mut includeValues = Vec::new();
    for name in includeSubVec {
	for rule in &includeRules {
	    let mut includeValue = String::new();
	    if let Err(err) = join(&rule, &name, configPath, version, platform, target, &attributes.map, &mut extraJson, &mut extraJsonClone, &mut includeValue) {
		println!("[Error] join parse error, err: {}", err);
		return None;
	    };
	    includeValues.push(includeValue);
	}
    }
    // println!("{:?}", &includeValues);
    // println!("###### {:?}", &includeValue);
    /*
    ** Get absolute path
    */
    match exeParam.paramType {
        ParamType::LibPath
        | ParamType::InstallLibPath => {
            if libpathEnableValue == enable_true {
                match Path::new(&libpathValue).canonicalize() {
                    Ok(p) => {
                        match p.to_str() {
                            Some(s) => {
                                if cfg!(target_os="windows"){
                                    // let t = s.trim_left_matches(r#"\\?\"#).replace(r#"\"#, r#"\\"#);
                                    let t = s.trim_left_matches(r#"\\?\"#);
                                    let c = Path::new(cmakeDir).canonicalize().unwrap().to_str().unwrap().trim_left_matches(r#"\\?\"#).to_string();
                                    // println!("{}, {}", &c,&t);
                                    let t = pathconvert::abs2rel(&c, &t).replace("\\", r#"/"#);
                                    r.libpath = Some(t);
                                } else {
                                    let c = Path::new(cmakeDir).canonicalize().unwrap().to_str().unwrap().to_string();
                                    r.libpath = Some(pathconvert::abs2rel(&c, s));
                                }
                            },
                            None => {
                                println!("[Error] libpath abs to_str error");
                            }
                        }
                    },
                    Err(err) => {
                        println!("[Error] libpath rule join path error, libpathValue: {}", &libpathValue);
                        fs::create_dir_all(&libpathValue);
                    }
                };
            } else {
                // println!("[Info] libpathEnable is false");
            }
            // if cfg!(target_os="windows") {
            // }
        },
        ParamType::InstallBinPath
        | ParamType::BinDirInstall
        | ParamType::BinFilesInstall => {
            match Path::new(&binpathValue).canonicalize() {
                Ok(p) => {
                    match p.to_str() {
                        Some(s) => {
                            if cfg!(target_os="windows"){
                                // let t = s.trim_left_matches(r#"\\?\"#).replace(r#"\"#, r#"\\"#);
                                let t = s.trim_left_matches(r#"\\?\"#);
                                let c = Path::new(cmakeDir).canonicalize().unwrap().to_str().unwrap().trim_left_matches(r#"\\?\"#).to_string();
                                // println!("{}, {}", &c,&t);
                                let t = pathconvert::abs2rel(&c, &t).replace("\\", r#"/"#);
                                r.binpath = Some(t);
                            } else {
                                let c = Path::new(cmakeDir).canonicalize().unwrap().to_str().unwrap().to_string();
                                r.binpath = Some(pathconvert::abs2rel(&c, s));
                            }
                        },
                        None => {
                            // r.binpath = Some(bin_dir_default.to_string());
                            // println!("[Error] binpath abs to_str error");
                        }
                    }
                },
                Err(err) => {
                    // println!("[Error] binpath rule join path error, binpathValue: {}", &binpathValue);
                    // r.binpath = Some(bin_dir_default.to_string());
                    fs::create_dir_all(&binpathValue);
                }
            };
        },
        ParamType::Include => {
            let isSelf = match &library.isSelf {
                Some(isSelf) => {
                    isSelf.to_string()
                },
                None => {
                    search::dependencies::is_self_false.to_string()
                }
            };
            if includeEnableValue == enable_true
            || isSelf == search::dependencies::is_self_last_true {
                let mut includes = Vec::new();
                for includeValue in includeValues.iter() {
                    match Path::new(&includeValue).canonicalize() {
                        Ok(p) => {
                            match p.as_os_str().to_str() {
                                Some(s) => {
                                    if cfg!(target_os="windows"){
                                        // let t = s.trim_left_matches(r#"\\?\"#).replace(r#"\"#, r#"\\"#);
                                        let t = s.trim_left_matches(r#"\\?\"#);
                                        let c = Path::new(cmakeDir).canonicalize().unwrap().to_str().unwrap().trim_left_matches(r#"\\?\"#).to_string();
                                        let t = pathconvert::abs2rel(&c, &t).replace("\\", r#"/"#);
                                        includes.push(t);
                                    } else {
                                        // r.include = Some(s.to_string());
                                        let c = Path::new(cmakeDir).canonicalize().unwrap().to_str().unwrap().to_string();
                                        includes.push(pathconvert::abs2rel(&c, s));
                                    }
                                },
                                None => {
                                    println!("[Erorr] include abs to_str error");
                                }
                            }
                        },
                        Err(err) => {
                            // println!("[Error] include rule join path error");
                            includes.push(includeValue.to_string());
                        }
                    }
                }
                r.include = Some(includes);
            } else {
                // println!("[Info] includeEnable is false");
            }
        },
        _ => {}
    }
    Some(r)
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
			"#, "", ".", "1.0.0", "", "", &None, &mut extraJson, &mut extraJsonClone, &mut result);
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
        join("abcd.`json:'extra.name'`.`json:'extra.objs[0]'`.`json:'extra.objs[1]'`", "", ".", "1.0.0", "", "", &None, &mut extraJson, &mut extraJsonClone, &mut result);
        println!("{:?}", result);
    }
}
