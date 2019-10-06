use crate::parse;
use crate::config;
use crate::calc;
use super::structs;
use crate::structs as cratestructs;

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

pub struct CDependSearcher {
}

impl CDependSearcher {
    pub fn search<'b>(&self, runArgs: &cratestructs::param::CRunArgs, root: &'b str/*, defaultLibRel: &str*/, param: &parse::git_lib::CGitLib, results: &mut Vec<CResults>) -> Result<(), &'b str> {
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
                    if let Err(_) = self.readLibConfig(runArgs, root/*, defaultLibRel*/, path, param, results) {
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
                    if let Err(_) = self.readLibConfig(runArgs, root/*, defaultLibRel*/, path, param, results) {
                        return true;
                    };
                    true
                }
            }
        })
    }
}

impl CDependSearcher {
    fn readLibConfig(&self, runArgs: &cratestructs::param::CRunArgs, root: &str/*, defaultLibRel: &str*/, path: &str, param: &parse::git_lib::CGitLib, results: &mut Vec<CResults>) -> Result<(), &str> {
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
        let fullName = match calc::dynlibname::get(param, searchVersion, &libConfig.package, dependVersion) {
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
        let libpath = match calc::dynlibpath::get(param, parent, searchVersion, &libConfig.package, dependVersion) {
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
                    if let Err(_) = self.search(runArgs, &value.root, &parse::git_lib::CGitLib{
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
        searcher.search(&cratestructs::param::CRunArgs{
        }, ".", &parse::git_lib::CGitLib{
            library: Some(&librarys),
            platform: Some("${FILE_PREFIX}".to_string()),
            target: Some("win64".to_string()),
            // platform: Some("".to_string()),
            extra: Some("{\"name\": \"jake\", \"objs\": [\"1\", \"2\", \"3\"]}".to_string()),
            extraType: Some("json".to_string())
        }, &mut Vec::new());
    }
}
