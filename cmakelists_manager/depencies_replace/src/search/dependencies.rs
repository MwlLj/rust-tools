use crate::parse;
use crate::config;
use crate::calc;
use super::structs;

use path::walk;

use std::collections::HashMap;
use std::path::Path;
use std::fs;

const lib_config_file_name: &str = "LibraryConfig.toml";
const lib_config_file_suffix: &str = "library.config.toml";

pub struct CDependSearcher {
}

enum LibType {
    SelfLib,
    DependLib
}

impl CDependSearcher {
    pub fn search<'b>(&self, root: &'b str/*, defaultLibRel: &str*/, param: &parse::git_lib::CGitLib, results: &mut Vec<calc::dynlibname::CResult>) -> Result<(), &'b str> {
        /*
        ** Get the library's own lib search path
        */
        self.searchInner(root, &LibType::SelfLib, &None, param, results)
    }
}

impl CDependSearcher {
    fn searchInner<'b>(&self, root: &'b str/*, libRoot: &str, libRel: &str*/, libType: &LibType, dependInfo: &Option<&structs::libs::CLibInfo>, param: &parse::git_lib::CGitLib, results: &mut Vec<calc::dynlibname::CResult>) -> Result<(), &'b str> {
        let searchName = match &param.name {
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
                    if let Err(_) = self.readLibConfig(root, libType, path, dependInfo, param, results) {
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
                    if let Err(_) = self.readLibConfig(root, libType, path, dependInfo, param, results) {
                        return true;
                    };
                    true
                }
            }
        })
    }
}

impl CDependSearcher {
    fn readLibConfig(&self, root: &str, libType: &LibType, path: &str, dependInfo: &Option<&structs::libs::CLibInfo>, param: &parse::git_lib::CGitLib, results: &mut Vec<calc::dynlibname::CResult>) -> Result<(), &str> {
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
        let name = match &param.name {
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
        let searchVersion = match &param.version {
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
        results.push(fullName);
        /*
        ** Find the directory where the specified name library name is located
        ** Search rule:
        ** 1. Find the specified run path
        ** 2. Find the specified relative path
        ** 3. In case of non-existence, use the default value
        */
        // depends
        match &dependVersion.dependencies {
            Some(depends) => {
                /*
                ** Sort depends on no field
                */
                let mut ds = Vec::new();
                for (key, value) in depends.iter() {
                    let r = match &value.root {
                        Some(r) => r,
                        None => root
                    };
                    ds.push(structs::libs::CLibInfo{
                        name: &key,
                        version: &value.version,
                        no: &value.no,
                        root: r,
                        libRoot: &value.libroot,
                        libRel: &value.librel
                    });
                }
                quick_sort::sort(&mut ds);
                // println!("{:?}", &ds);
                for value in ds.iter() {
                    // let mut p = param.clone();
                    // p.name = Some(key.to_string());
                    // p.version = Some(value.version.to_string());
                    /*
                    ** Get the lib search path of the dependent library
                    */
                    if let Err(_) = self.searchInner(value.root, &LibType::DependLib, &Some(value), &parse::git_lib::CGitLib{
                        name: Some(value.name.to_string()),
                        version: Some(value.version.to_string()),
                        platform: param.platform.clone(),
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

#[derive(Debug, Default)]
struct CSearchPath {
    libRoot: String,
    libRel: String
}

impl CDependSearcher {
    fn findLibDir(&self, fullName: &calc::dynlibname::CResult
        , libType: &LibType
        , libPackage: &config::libconfig::CPackage
        , libVesion: &config::libconfig::CVersion
        , lib: &structs::libs::CLibInfo) -> Option<Vec<String>> {
        let mut names = Vec::new();
        match &fullName.dr {
            Some(name) => {
                names.push(name);
            },
            None => {
                match &fullName.debug {
                    Some(name) => {
                        names.push(name);
                    },
                    None => {
                        println!("[Warning denug is None]");
                        return None;
                    }
                }
                match &fullName.release {
                    Some(name) => {
                        names.push(name);
                    },
                    None => {
                        println!("[Warning release is None]");
                        return None;
                    }
                }
            }
        }
        let mut paths = Vec::new();
        match libType {
            LibType::SelfLib => {
                let search = self.getSelfLibSearch(libPackage, libVesion);
            },
            LibType::DependLib => {
                let search = self.getDependLibSearch(libPackage, libVesion, lib);
            }
        }
        Some(paths)
    }

    /*
    ** Find the directory where the library name is located based on the root directory and the library name
    */
    fn findLibParentDir(&self, root: &str, libName: &str) -> Option<String> {
        let mut dir: Option<String> = None;
        if let Err(err) = walk::walk_one_fn(root, &mut |path: &str, name: &str, t: walk::Type| -> bool {
            if name == libName {
                let parent = match Path::new(path).parent() {
                    Some(p) => p,
                    None => {
                        return true;
                    }
                };
                let parent = match parent.to_str() {
                    Some(p) => p,
                    None => {
                        return true;
                    }
                };
                dir = Some(parent.to_string());
                return false;
            }
            return true;
        }) {
            return None;
        };
        dir
    }

    /*
    ** Get the library's own lib search path
    ** search rule:
    ** version attr -> package attr
    */
    fn getSelfLibSearch(&self, libPackage: &config::libconfig::CPackage, libVesion: &config::libconfig::CVersion) -> CSearchPath {
        let mut searchPath = CSearchPath::default();
        match &libVesion.attributes {
            Some(attr) => {
                match &attr.libroot {
                    Some(r) => {
                        searchPath.libRoot = r.clone();
                    },
                    None => {
                        searchPath.libRoot = calc::dynlibname::libroot_default.to_string();
                    }
                }
                match &attr.librel {
                    Some(r) => {
                        searchPath.libRel = r.clone();
                    },
                    None => {
                        searchPath.libRel = calc::dynlibname::librel_default.to_string();
                    }
                }
            },
            None => {
                match &libPackage.libroot {
                    Some(r) => {
                        searchPath.libRoot = r.clone();
                    },
                    None => {
                        searchPath.libRoot = calc::dynlibname::libroot_default.to_string();
                    }
                }
                match &libPackage.librel {
                    Some(r) => {
                        searchPath.libRel = r.clone();
                    },
                    None => {
                        searchPath.libRel = calc::dynlibname::librel_default.to_string();
                    }
                }
            }
        }
        searchPath
    }

    /*
    ** Get the lib search path of the dependent library
    ** search rule:
    ** depend attr -> version attr -> package attr
    */
    fn getDependLibSearch(&self, libPackage: &config::libconfig::CPackage, libVesion: &config::libconfig::CVersion, lib: &structs::libs::CLibInfo) -> CSearchPath {
        let mut search = self.getSelfLibSearch(libPackage, libVesion);
        match &lib.libRoot {
            Some(r) => {
                search.libRoot = r.to_string();
            },
            None => {
                // use getSelfLibSearch libroot
            }
        }
        match &lib.libRel {
            Some(r) => {
                search.libRel = r.to_string();
            },
            None => {
                // use getSelfLibSearch librel
            }
        }
        search
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
        let searcher = CDependSearcher::new();
        searcher.search(".", &parse::git_lib::CGitLib{
            name: Some("test".to_string()),
            version: Some("0.1.10".to_string()),
            platform: Some("${FILE_PREFIX}".to_string()),
            // platform: Some("".to_string()),
            extra: Some("{\"name\": \"jake\", \"objs\": [\"1\", \"2\", \"3\"]}".to_string()),
            extraType: Some("json".to_string())
        }, &mut Vec::new());
    }
}
