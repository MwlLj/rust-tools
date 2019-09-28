use crate::parse;
use crate::config;
use crate::calc;

use path::walk;

use std::collections::HashMap;
use std::path::Path;
use std::fs;

const lib_config_file_name: &str = "LibraryConfig.toml";
const lib_config_file_suffix: &str = "library.config.toml";

struct CDependSearcher {
}

impl CDependSearcher {
    pub fn search<'b>(&self, root: &'b str, param: &parse::git_lib::CGitLib) -> Result<(), &'b str> {
        let searchName = match &param.name {
            Some(n) => n,
            None => {
                println!("name field is not found");
                return Err("name field is not found");
            }
        };
        walk::walk(root, &mut |path: &str, name: &str| -> bool {
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
            if let Err(_) = self.readLibConfig(path, param) {
                return true;
            };
            true
        }, &mut |path: &str, name: &str| -> bool {
            // file
            let mut n = String::new();
            n.push_str(searchName);
            n.push_str(".");
            n.push_str(lib_config_file_suffix);
            if name != n {
                return true;
            }
            // find lib.libraryconfig.toml
            if let Err(_) = self.readLibConfig(path, param) {
                return true;
            };
            true
        })
    }
}

impl CDependSearcher {
    fn readLibConfig(&self, path: &str, param: &parse::git_lib::CGitLib) -> Result<(), &str> {
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
        println!("{:?}", fullName);
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
    // #[ignore]
    fn dependSearcherTest() {
        let searcher = CDependSearcher::new();
        searcher.search(".", &parse::git_lib::CGitLib{
            name: Some("test".to_string()),
            version: Some("0.1.10".to_string()),
            platform: Some("${FILE_PREFIX}".to_string()),
            // platform: Some("".to_string()),
            extra: Some("{\"name\": \"jake\", \"objs\": [\"1\", \"2\", \"3\"]}".to_string()),
            extraType: Some("json".to_string())
        });
    }
}
