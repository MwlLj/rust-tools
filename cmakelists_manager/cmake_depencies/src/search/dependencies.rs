use crate::parse;
use crate::config;
use crate::calc;
use super::structs;
use crate::structs as cratestructs;
use parse::git_lib;
use parse::git_librarys;
use git_lib::ParamType;

use path::walk;

use std::collections::HashMap;
use std::path::Path;
use std::fs;

const lib_config_file_name: &str = "LibraryConfig.toml";
const lib_config_file_suffix: &str = "library.config.toml";

#[derive(Debug, Default)]
pub struct CResults {
    pub name: calc::dynlibname::CResult,
    pub libpath: calc::dynlibpath::CResult
}

#[derive(Debug, Default, Clone)]
pub struct CSearchResult {
    pub startIndex: usize,
    pub name: Vec<String>,
    pub paramType: git_lib::ParamType
}

pub struct CDependSearcher {
}

impl CDependSearcher {
    pub fn search<'a>(&self, root: &'a str, library: &git_librarys::CGitLibrarys, params: &Vec<git_lib::CParam>, results: &mut Vec<Vec<CSearchResult>>) -> Result<(), &'a str> {
        let searchName = match &library.name {
            Some(n) => n,
            None => {
                println!("name field is not found");
                return Err("name field is not found");
            }
        };
        walk::walk_one_fn(root, &mut |path: &str, name: &str, t: walk::Type| -> bool {
            match t {
                walk::Type::Dir => {
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
                    if let Err(_) = self.readLibConfig(root, path, library, params, results) {
                        return true;
                    };
                    true
                },
                walk::Type::File => {
                    // file
                    let mut n = String::new();
                    n.push_str(searchName);
                    n.push_str(".");
                    n.push_str(lib_config_file_suffix);
                    if name != n {
                        return true;
                    }
                    // find lib.libraryconfig.toml
                    if let Err(_) = self.readLibConfig(root, path, library, params, results) {
                        return true;
                    };
                    true
                }
            }
        })
    }

    fn readLibConfig(&self, root: &str, path: &str, library: &git_librarys::CGitLibrarys, params: &Vec<parse::git_lib::CParam>, results: &mut Vec<Vec<CSearchResult>>) -> Result<(), &str> {
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
                println!("version field is not found");
                return Err("version field is not found");
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
                ParamType::LibName => {
                    // dynamic calc this version lib - full name
                    let fullNames = match calc::dynlibname::get(param, searchVersion, &library.libs, &libConfig.package, dependVersion) {
                        Some(n) => n,
                        None => {
                            println!("calc full name error");
                            return Err("calc full name error");
                        }
                    };
                    if fullNames.len() == 0 {
                        continue;
                    }
                    rs.push(CSearchResult{
                        startIndex: param.startIndex,
                        name: fullNames,
                        paramType: param.paramType.clone()
                    });
                },
                ParamType::LibPath => {
                    /*
                    ** Get the dependent library path
                    */
                    // println!("path: {}, parent: {}", path, parent);
                    let libpath = match calc::dynlibpath::get(param, parent, searchVersion, &libConfig.package, dependVersion) {
                        Some(l) => l,
                        None => {
                            println!("calc libpath error");
                            return Err("calc libpath error");
                        }
                    };
                    let libpath = match &libpath.libpath {
                        Some(p) => p.to_string(),
                        None => {
                            println!("get libpath error");
                            // return Err("get libpath error");
                            // "".to_string()
                            continue;
                        }
                    };
                    rs.push(CSearchResult{
                        startIndex: param.startIndex,
                        name: vec![libpath],
                        paramType: param.paramType.clone()
                    });
                },
                ParamType::Include => {
                    /*
                    ** Get the dependent library path
                    */
                    // println!("path: {}, parent: {}", path, parent);
                    /*
                    let path = match &param.path {
                        Some(path) => {
                            path.to_string()
                        },
                        None => {
                            let libpath = match calc::dynlibpath::get(param, parent, searchVersion, &libConfig.package, dependVersion) {
                                Some(l) => l,
                                None => {
                                    println!("calc include error");
                                    return Err("calc include error");
                                }
                            };
                            let include = match &libpath.include {
                                Some(p) => p,
                                None => {
                                    println!("get include error");
                                    return Err("get include error");
                                }
                            };
                            include.to_string()
                        }
                    };
                    */
                    // println!("############, {}", &path);
                    let libpath = match calc::dynlibpath::get(param, parent, searchVersion, &libConfig.package, dependVersion) {
                        Some(l) => l,
                        None => {
                            println!("calc include error");
                            return Err("calc include error");
                        }
                    };
                    let include = match &libpath.include {
                        Some(p) => p,
                        None => {
                            println!("get include error");
                            continue;
                            // return Err("get include error");
                        }
                    };
                    rs.push(CSearchResult{
                        startIndex: param.startIndex,
                        name: vec![include.to_string()],
                        paramType: param.paramType.clone()
                    });
                },
                _ => {
                }
            }
        }
        // println!("{:?}", fullName);
        // println!("results.push: {:?}", &rs);
        results.push(rs);
        // depends
        match &dependVersion.dependencies {
            Some(depends) => {
                /*
                ** Sort depends on no field
                */
                let mut ds = Vec::new();
                for (key, value) in depends.iter() {
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
                    ds.push(structs::libs::CLibInfo{
                        name: &key,
                        enable: &value.enable,
                        includeEnable: &value.includeEnable,
                        libpathEnable: &value.libpathEnable,
                        libnameEnable: &value.libnameEnable,
                        subs: &value.subs,
                        version: &value.version,
                        no: &value.no,
                        root: r.to_string()
                    });
                }
                quick_sort::sort(&mut ds);
                // println!("{:?}", &ds);
                for value in ds.iter() {
                    // let mut p = param.clone();
                    // p.name = Some(key.to_string());
                    // p.version = Some(value.version.to_string());
                    let mut libs = Vec::new();
                    match &value.subs {
                        Some(subs) => {
                            if subs.trim() == git_librarys::subs_null {
                            } else {
                                let vs: Vec<&str> = subs.split(git_librarys::subs_sp).collect();
                                for v in vs {
                                    libs.push(v.trim().to_string());
                                }
                            }
                        },
                        None => {
                            libs.push(value.name.to_string());
                        }
                    }
                    let mut paramsClone = params.clone();
                    /*
                    if let Some(enable) = value.enable {
                        for paramMut in paramsClone.iter_mut() {
                            (*paramMut).enable = Some(enable.to_string());
                        }
                    };
                    */
                    for paramMut in paramsClone.iter_mut() {
                        if let Some(enable) = value.enable {
                            (*paramMut).enable = Some(enable.to_string());
                        };
                        if let Some(includeEnable) = value.includeEnable {
                            (*paramMut).includeEnable = Some(includeEnable.to_string());
                        };
                        if let Some(libpathEnable) = value.libpathEnable {
                            (*paramMut).libpathEnable = Some(libpathEnable.to_string());
                        };
                        if let Some(libnameEnable) = value.libnameEnable {
                            (*paramMut).libnameEnable = Some(libnameEnable.to_string());
                        };
                    }
                    if let Err(_) = self.search(&value.root, &git_librarys::CGitLibrarys{
                        name: Some(value.name.to_string()),
                        version: Some(value.version.to_string()),
                        libs: libs
                    }, &paramsClone, results) {
                        return Err("search error");
                    };
                }
            },
            None => {
            }
        }
        Ok(())
    }
}

/*
impl CDependSearcher {
    pub fn search1<'b>(&self, runArgs: &cratestructs::param::CRunArgs, root: &'b str/*, defaultLibRel: &str*/, param: &parse::git_lib::CGitLib, results: &mut Vec<CResults>) -> Result<(), &'b str> {
        let librarys = match &param.library {
            Some(n) => n,
            None => {
                println!("librarys field is not found");
                return Err("librarys field is not found");
            }
        };
        let searchName = match &librarys.name {
            Some(n) => n,
            None => {
                println!("name field is not found");
                return Err("name field is not found");
            }
        };
        walk::walk_one_fn(root, &mut |path: &str, name: &str, t: walk::Type| -> bool {
            match t {
                walk::Type::Dir => {
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
                    if let Err(_) = self.readLibConfig1(runArgs, root/*, defaultLibRel*/, path, param, results) {
                        return true;
                    };
                    true
                },
                walk::Type::File => {
                    // file
                    let mut n = String::new();
                    n.push_str(searchName);
                    n.push_str(".");
                    n.push_str(lib_config_file_suffix);
                    if name != n {
                        return true;
                    }
                    // find lib.libraryconfig.toml
                    if let Err(_) = self.readLibConfig1(runArgs, root/*, defaultLibRel*/, path, param, results) {
                        return true;
                    };
                    true
                }
            }
        })
    }
}

impl CDependSearcher {
    fn readLibConfig1(&self, runArgs: &cratestructs::param::CRunArgs, root: &str/*, defaultLibRel: &str*/, path: &str, param: &parse::git_lib::CGitLib, results: &mut Vec<CResults>) -> Result<(), &str> {
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
        // judge name is equal
        let librarys = match &param.library {
            Some(n) => n,
            None => {
                println!("librarys field is not found");
                return Err("librarys field is not found");
            }
        };
        let name = match &librarys.name {
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
        let searchVersion = match &librarys.version {
            Some(v) => v,
            None => {
                println!("version field is not found");
                return Err("version field is not found");
            }
        };
        let dependVersion = match libConfig.versions.get(searchVersion) {
            Some(v) => v,
            None => {
                println!("search dependencies lib: {}, version: {} error, [not found]", name, searchVersion);
                return Err("search failed");
            }
        };
        // dynamic calc this version lib - full name
        let fullName = match calc::dynlibname::get1(param, searchVersion, &libConfig.package, dependVersion) {
            Some(n) => n,
            None => {
                return Err("calc full name error");
            }
        };
        // println!("{:?}", fullName);
        /*
        ** Get the dependent library path
        */
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
        // println!("path: {}, parent: {}", path, parent);
        let libpath = match calc::dynlibpath::get1(param, parent, searchVersion, &libConfig.package, dependVersion) {
            Some(l) => l,
            None => {
                return Err("calc libpath error");
            }
        };
        results.push(CResults{
            name: fullName,
            libpath: libpath
        });
        // depends
        match &dependVersion.dependencies {
            Some(depends) => {
                /*
                ** Sort depends on no field
                */
                let mut ds = Vec::new();
                for (key, value) in depends.iter() {
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
                    ds.push(structs::libs::CLibInfo{
                        name: &key,
                        version: &value.version,
                        no: &value.no,
                        root: r.to_string()
                    });
                }
                quick_sort::sort(&mut ds);
                // println!("{:?}", &ds);
                for value in ds.iter() {
                    // let mut p = param.clone();
                    // p.name = Some(key.to_string());
                    // p.version = Some(value.version.to_string());
                    if let Err(_) = self.search1(runArgs, &value.root, &parse::git_lib::CGitLib{
                        library: Some(&parse::git_librarys::CGitLibrarys{
                            name: Some(value.name.to_string()),
                            version: Some(value.version.to_string())
                        }),
                        platform: param.platform.clone(),
                        target: param.target.clone(),
                        extra: param.extra.clone(),
                        extraType: param.extraType.clone()
                    }, results) {
                        return Err("search error");
                    };
                }
            },
            None => {
            }
        }
        Ok(())
    }
}
*/

impl CDependSearcher {
    pub fn new() -> CDependSearcher {
        CDependSearcher{}
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
            version: Some("0.1.10".to_string())
        };
        let searcher = CDependSearcher::new();
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
