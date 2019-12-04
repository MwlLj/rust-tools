use crate::parse;
use crate::config;
use crate::calc;
use super::structs;
use crate::structs as cratestructs;
use parse::git_lib;
use parse::git_librarys;
use parse::replace;
use git_lib::ParamType;

use path::walk;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

const lib_config_file_name: &str = "LibraryConfig.toml";
const lib_config_file_suffix: &str = "library.config.toml";

pub const is_self_defult: &str = "false";
pub const is_self_true: &str = "true";
pub const is_self_false: &str = "false";
pub const is_self_last_true: &str = "last_true";

#[derive(Debug, Default)]
pub struct CResults {
    pub name: calc::dynlibname::CResult,
    pub libpath: calc::dynlibpath::CResult
}

#[derive(Debug, Default, Clone)]
pub struct CSearchResult {
    pub startIndex: usize,
    pub name: Vec<String>,
    pub paramType: git_lib::ParamType,
    /*
    ** Whether it is itself
    */
    pub isSelf: String
}

pub struct CDependSearcher<'b> {
    searchFilter: &'b cratestructs::param::CSearchFilter
}

impl<'b> CDependSearcher<'b> {
    pub fn search<'a>(&self, root: &'a str, cmakeDir: &str, library: &git_librarys::CGitLibrarys, params: &Vec<git_lib::CParam>, results: &mut Vec<Vec<CSearchResult>>) -> Result<(), &'a str> {
        let searchName = match &library.name {
            Some(n) => n,
            None => {
                println!("name field is not found");
                return Err("name field is not found");
            }
        };
        // println!("{:?}", root);
        walk::scan::walk_one_fn(root, &mut |path: &str, name: &str, t: walk::scan::Type| -> bool {
            match t {
                walk::scan::Type::Dir => {
                    // println!("{:?}", path);
                    // dir
                    if name != searchName {
                        return true;
                    }
                    // find lib-name/LibraryConfig.toml
                    let filePath = Path::new(path).join(lib_config_file_name);
                    if !filePath.exists() {
                        return true
                    }
                    let path = match filePath.to_str() {
                        Some(p) => p,
                        None => {
                            println!("filePath to_str error");
                            return true;
                        }
                    };
                    if let Err(_) = self.readLibConfig(root, path, cmakeDir, library, params, results) {
                        return true;
                    };
                    false
                },
                walk::scan::Type::File => {
                    // file
                    let mut n = String::new();
                    n.push_str(searchName);
                    n.push_str(".");
                    n.push_str(lib_config_file_suffix);
                    if name != n {
                        return true;
                    }
                    // find lib.libraryconfig.toml
                    if let Err(_) = self.readLibConfig(root, path, cmakeDir, library, params, results) {
                        return true;
                    };
                    true
                },
                walk::scan::Type::OnceEnd => {
                    match &self.searchFilter.parentPathIsnotIncluded {
                        Some(parents) => {
                            if parents.contains(&name.to_string()) {
                                return false;
                            }
                        },
                        None => {
                        }
                    }
                    true
                }
            }
        })
    }

    pub fn searchByObject(&self, root: &str, path: &str, libConfig: &config::libconfig::CLibConfig, cmakeDir: &str, library: &git_librarys::CGitLibrarys, params: &Vec<parse::git_lib::CParam>, results: &mut Vec<Vec<CSearchResult>>) -> Result<(), &str> {
        self.readLibConfigByObject(root, path, libConfig, cmakeDir, library, params, results
        , &mut |dependVersion, parentPath, results| -> Result<(), &str> {
            Ok(())
        })
    }

    fn readLibConfig(&self, root: &str, path: &str, cmakeDir: &str, library: &git_librarys::CGitLibrarys, params: &Vec<parse::git_lib::CParam>, results: &mut Vec<Vec<CSearchResult>>) -> Result<(), &str> {
        let content = match fs::read(path) {
            Ok(f) => f,
            Err(err) => {
                println!("read lib config error, err: {}, path: {}", err, path);
                return Err("read config file error");
            }
        };
        let libConfig = match config::libconfig::parse(&content) {
            Ok(c) => c,
            Err(err) => {
                println!("config file parse error, err: {}", err);
                return Err("parse error");
            }
        };
        self.readLibConfigByObject(root, path, &libConfig, cmakeDir, library, params, results
            , &mut |dependencies, parentPath, results| -> Result<(), &str> {
            match dependencies {
                Some(depends) => {
                    for value in depends.iter() {
                        // let mut p = param.clone();
                        // p.name = Some(key.to_string());
                        // p.version = Some(value.version.to_string());
                        let r = match &value.root {
                            Some(r) => {
                                match parentPath.join(r).to_str() {
                                    Some(p) => p.to_string(),
                                    None => {
                                        root.to_string()
                                    }
                                }
                            },
                            None => root.to_string()
                        };
                        let mut libs = String::new();
                        match &value.subs {
                            Some(subs) => {
                                libs = subs.to_string();
                            },
                            None => {
                                libs = value.name.to_string();
                            }
                        }
                        let mut dlls = String::new();
                        match &value.dllSubs {
                            Some(subs) => {
                                dlls = subs.to_string();
                            },
                            None => {
                                dlls = value.name.to_string();
                            }
                        }
                        let mut enable = None;
                        let mut includeEnable = None;
                        let mut libpathEnable = None;
                        let mut libnameEnable = None;
                        if let Some(v) = &value.enable {
                            enable = Some(v.to_string());
                        };
                        if let Some(v) = &value.includeEnable {
                            includeEnable = Some(v.to_string());
                        };
                        if let Some(v) = &value.libpathEnable {
                            libpathEnable = Some(v.to_string());
                        };
                        if let Some(v) = &value.libnameEnable {
                            libnameEnable = Some(v.to_string());
                        };
                        if let Err(_) = self.search(&r, cmakeDir, &git_librarys::CGitLibrarys{
                            name: Some(value.name.to_string()),
                            version: Some(value.version.to_string()),
                            libs: libs,
                            dlls: dlls,
                            enable: enable,
                            includeEnable: includeEnable,
                            libpathEnable: libpathEnable,
                            libnameEnable: libnameEnable,
                            isSelf: match &library.isSelf {
                                Some(is) => {
                                    if is == is_self_true {
                                        Some(is_self_last_true.to_string())
                                    } else {
                                        None
                                    }
                                },
                                None => {
                                    None
                                }
                            },
                            config: None
                        }, &params, results) {
                            return Err("search error");
                        };
                    }
                },
                None => {
                }
            }
            Ok(())
        })
    }

    fn readLibConfigByObject<'a, DependFn>(&'a self, root: &str, path: &str, libConfig: &config::libconfig::CLibConfig, cmakeDir: &str, library: &git_librarys::CGitLibrarys, params: &Vec<parse::git_lib::CParam>, results: &mut Vec<Vec<CSearchResult>>, dependFn: &mut DependFn) -> Result<(), &str>
        where DependFn: FnMut(&Option<Vec<config::libconfig::CLib>>, &Path, &mut Vec<Vec<CSearchResult>>) -> Result<(), &'a str> {
        // judge name is equal
        let name = match &library.name {
            Some(n) => n,
            None => {
                println!("name field is not found");
                return Err("name field is not found");
            }
        };
        if &libConfig.package.name != name {
            println!("[Warning] name is not equal, name: {}", name);
        }
        // find version dependencies
        let searchVersion = match &library.version {
            Some(v) => v,
            None => {
                // println!("version field is not found");
                // return Err("version field is not found");
                ""
            }
        };
        let dependVersion = match libConfig.versions.get(searchVersion) {
            Some(v) => v,
            None => {
                println!("search dependencies lib: {}, version: {} error, [not found]", name, searchVersion);
                return Err("search failed");
            }
        };
        let parentPath = match Path::new(path).parent() {
            Some(p) => p,
            None => {
                return Err("get parent dir error");
            }
        };
        let parent = match parentPath.as_os_str().to_str() {
            Some(s) => s,
            None => {
                return Err("parent to_str error");
            }
        };
        let mut rs = Vec::new();
        for param in params {
            match param.paramType {
                ParamType::LibName
                | ParamType::DebugTargetName
                | ParamType::ReleaseTargetName => {
                    match param.paramType {
                        ParamType::DebugTargetName
                        | ParamType::ReleaseTargetName => {
                            match &library.isSelf {
                                Some(is) => {
                                    if is == is_self_false
                                    || is == is_self_last_true {
                                        continue;
                                    } else {
                                        // println!("########, {:?}", param.paramType);
                                    }
                                },
                                None => {
                                    continue;
                                }
                            }
                        },
                        _ => {}
                    }
                    // dynamic calc this version lib - full name
                    let names = match calc::dynlibname::getLib(library, param, searchVersion, &library.libs, &libConfig.package, dependVersion) {
                        Some(n) => n,
                        None => {
                            println!("calc full name error");
                            return Err("calc full name error");
                        }
                    };
                    if names.len() == 0 {
                        continue;
                    }
                    rs.push(CSearchResult{
                        startIndex: param.startIndex,
                        name: names,
                        paramType: param.paramType.clone(),
                        isSelf: library.isSelf.clone().unwrap_or(is_self_defult.to_string())
                    });
                },
                ParamType::LibPath
                | ParamType::InstallLibPath => {
                    match param.paramType {
                        ParamType::InstallLibPath => {
                            match &library.isSelf {
                                Some(is) => {
                                    if is == is_self_false
                                    || is == is_self_last_true {
                                        continue;
                                    } else {
                                        // println!("########, {:?}", param.paramType);
                                    }
                                },
                                None => {
                                    continue;
                                }
                            }
                        },
                        _ => {}
                    }
                    /*
                    ** Get the dependent library path
                    */
                    // println!("path: {}, parent: {}", path, parent);
                    let libpath = match calc::dynlibpath::get(library, param, parent, cmakeDir, searchVersion, &libConfig.package, dependVersion) {
                        Some(l) => l,
                        None => {
                            println!("calc libpath error");
                            return Err("calc libpath error");
                        }
                    };
                    let libpath = match &libpath.libpath {
                        Some(p) => p.to_string(),
                        None => {
                            // println!("get libpath error");
                            // return Err("get libpath error");
                            // "".to_string()
                            continue;
                        }
                    };
                    rs.push(CSearchResult{
                        startIndex: param.startIndex,
                        name: vec![libpath],
                        paramType: param.paramType.clone(),
                        isSelf: library.isSelf.clone().unwrap_or(is_self_defult.to_string())
                    });
                },
                ParamType::InstallBinPath => {
                    match &library.isSelf {
                        Some(is) => {
                            if is == is_self_false
                            || is == is_self_last_true {
                                continue;
                            } else {
                                // println!("########, {:?}", param.paramType);
                            }
                        },
                        None => {
                            continue;
                        }
                    }
                    /*
                    ** Get the dependent binary path
                    */
                    // println!("path: {}, parent: {}", path, parent);
                    let binpath = match calc::dynlibpath::get(library, param, parent, cmakeDir, searchVersion, &libConfig.package, dependVersion) {
                        Some(l) => l,
                        None => {
                            println!("calc binpath error");
                            return Err("calc binpath error");
                        }
                    };
                    let binpath = match &binpath.binpath {
                        Some(p) => p.to_string(),
                        None => {
                            // println!("get binpath error");
                            // return Err("get binpath error");
                            "".to_string()
                            // continue;
                        }
                    };
                    rs.push(CSearchResult{
                        startIndex: param.startIndex,
                        name: vec![binpath],
                        paramType: param.paramType.clone(),
                        isSelf: library.isSelf.clone().unwrap_or(is_self_defult.to_string())
                    });
                },
                ParamType::Include => {
                    let libpath = match calc::dynlibpath::get(library, param, parent, cmakeDir, searchVersion, &libConfig.package, dependVersion) {
                        Some(l) => l,
                        None => {
                            println!("calc include error");
                            return Err("calc include error");
                        }
                    };
                    let includes = match libpath.include {
                        Some(p) => p,
                        None => {
                            continue;
                        }
                    };
                    // println!("{:?}", &includes);
                    rs.push(CSearchResult{
                        startIndex: param.startIndex,
                        name: includes,
                        paramType: param.paramType.clone(),
                        isSelf: library.isSelf.clone().unwrap_or(is_self_defult.to_string())
                    });
                },
                ParamType::BinDirInstall => {
                    // dynamic calc this version lib - full name
                    let names = match calc::dynlibname::getDll(library, param, searchVersion, &library.dlls, &libConfig.package, dependVersion) {
                        Some(n) => n,
                        None => {
                            println!("calc full name error");
                            return Err("calc full name error");
                        }
                    };
                    if names.len() == 0 {
                        continue;
                    }
                    let template = match &param.template {
                        Some(t) => t,
                        None => {
                            continue;
                        }
                    };
                    /*
                    ** Get bin directory
                    */
                    let binpath = match calc::dynlibpath::get(library, param, parent, cmakeDir, searchVersion, &libConfig.package, dependVersion) {
                        Some(l) => l,
                        None => {
                            println!("calc binpath error");
                            return Err("calc binpath error");
                        }
                    };
                    let binpath = match &binpath.binpath {
                        Some(p) => p.to_string(),
                        None => {
                            continue;
                        }
                    };
                    let c = replace::parse(&template, &vec![&binpath]);
                    rs.push(CSearchResult{
                        startIndex: param.startIndex,
                        name: vec![c],
                        paramType: param.paramType.clone(),
                        isSelf: library.isSelf.clone().unwrap_or(is_self_defult.to_string())
                    });
                },
                ParamType::BinFilesInstall => {
                    // dynamic calc this version lib - full name
                    let names = match calc::dynlibname::getDll(library, param, searchVersion, &library.dlls, &libConfig.package, dependVersion) {
                        Some(n) => n,
                        None => {
                            println!("calc full name error");
                            return Err("calc full name error");
                        }
                    };
                    if names.len() == 0 {
                        continue;
                    }
                    let template = match &param.template {
                        Some(t) => t,
                        None => {
                            continue;
                        }
                    };
                    /*
                    ** First: Get bin directory
                    ** Second: Get dll name collection
                    */
                    let binpath = match calc::dynlibpath::get(library, param, parent, cmakeDir, searchVersion, &libConfig.package, dependVersion) {
                        Some(l) => l,
                        None => {
                            println!("calc binpath error");
                            return Err("calc binpath error");
                        }
                    };
                    let binpath = match &binpath.binpath {
                        Some(p) => p.to_string(),
                        None => {
                            continue;
                        }
                    };
                    let mut ns = Vec::new();
                    for item in names.iter() {
                        let nameResult: calc::dynlibname::CNameResult = serde_json::from_str(item).expect("CNameResult from_str error => replace/mode.rs");
                        match Path::new(&binpath).join(&nameResult.fullName).to_str() {
                            Some(s) => {
                                let c = replace::parse(&template, &vec![s]);
                                ns.push(c);
                            },
                            None => {
                            }
                        }
                    }
                    rs.push(CSearchResult{
                        startIndex: param.startIndex,
                        name: ns,
                        paramType: param.paramType.clone(),
                        isSelf: library.isSelf.clone().unwrap_or(is_self_defult.to_string())
                    });
                },
                _ => {
                }
            }
        }
        // println!("{:?}", fullName);
        // println!("results.push: {:?}", &rs);
        results.push(rs);
        dependFn(&dependVersion.dependencies, parentPath, results);
        Ok(())
    }
}

impl<'b> CDependSearcher<'b> {
    pub fn new(searchFilter: &'b cratestructs::param::CSearchFilter) -> CDependSearcher {
        CDependSearcher{
            searchFilter: searchFilter
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[ignore]
    fn dependSearcherTest() {
        let librarys = parse::git_librarys::CGitLibrarys{
            name: Some("test".to_string()),
            version: Some("0.1.10".to_string()),
            libs: String::new(),
            dlls: String::new(),
            enable: None,
            includeEnable: None,
            libpathEnable: None,
            libnameEnable: None,
            isSelf: None,
            config: None
        };
        let searcher = CDependSearcher::new(&cratestructs::param::CSearchFilter::default());
        /*
        searcher.search1(&cratestructs::param::CRunArgs{
        }, ".", &parse::git_lib::CGitLib{
            library: Some(&librarys),
            platform: Some("${FILE_PREFIX}".to_string()),
            target: Some("win64".to_string()),
            // platform: Some("".to_string()),
            extra: Some("{\"name\": \"jake\", \"objs\": [\"1\", \"2\", \"3\"]}".to_string()),
            extraType: Some("json".to_string())
        }, &mut Vec::new());
        */
    }
}
