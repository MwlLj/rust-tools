use crate::parse;
use crate::search;
use crate::structs;
use parse::git_librarys;
use parse::git_lib;
use search::dependencies::CDependSearcher;
use search::dependencies::CSearchResult;
use merge::CMerge;
use environments::CEnvironments;
use environments::CRepalce;
use path::pathconvert;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

const cmakelist_name: &str = "CMakeLists.txt";

pub struct CReplace {
    environmenter: CEnvironments
}

impl CReplace {
    pub fn replace(&self, cmakePath: &str, root: &str, cbbStoreRoot: &str, searchFilter: &structs::param::CSearchFilter) -> Result<(), &str> {
        // self.search(root, content, librarys, params)
        let merge = CMerge::new();
        let c = match merge.merge(cmakePath) {
            Ok(c) => c,
            Err(err) => {
                return Err("cmake merge error");
            }
        };
        let (librarys, params, replaces, mut content) = match self.environmenter.parse(&c.as_bytes().to_vec(), cbbStoreRoot) {
            Ok(r) => r,
            Err(err) => {
                return Err("cmake parse error");
            }
        };
        // println!("{:?}", params);
        self.search(cmakePath, root, cbbStoreRoot, &mut content, &librarys, &params, &replaces, searchFilter);
        Ok(())
    }
}

#[derive(Eq, Debug)]
struct CContent<'a> {
    pub index: &'a usize,
    pub content: &'a str
}

impl<'a> std::cmp::PartialEq for CContent<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<'a> std::cmp::PartialOrd for CContent<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> std::cmp::Ord for CContent<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl CReplace {
    fn search(&self, path: &str, root: &str, cbbStoreRoot: &str, content: &mut String, librarys: &Vec<git_librarys::CGitLibrarys>, params: &Vec<git_lib::CParam>, replaces: &Vec<CRepalce>, searchFilter: &structs::param::CSearchFilter) {
        let mut contents: HashMap<usize, String> = HashMap::new();
        let mut libs: HashSet<String> = HashSet::new();
        let p = Path::new(path);
        let parent = match p.parent() {
            Some(p) => p,
            None => {
                println!("get path parent path error");
                return;
            }
        };
        let cmakeDir = match parent.to_str() {
            Some(dir) => dir,
            None => {
                panic!("cmake dir not exist, should not error");
            }
        };
        for replace in replaces.iter() {
            let s = self.pathReplace(&replace.value, cbbStoreRoot, cmakeDir);
            match contents.get_mut(&replace.startIndex) {
                Some(c) => {
                    (*c).push_str(s.as_str());
                },
                None => {
                    contents.insert(replace.startIndex, s);
                }
            }
        }
        for library in librarys.iter() {
            let mut results: Vec<Vec<CSearchResult>> = Vec::new();
            let searcher = CDependSearcher::new(searchFilter);
            if let Err(err) = searcher.search(&root, cmakeDir, library, params, &mut results) {
                println!("[Error] search error, err: {}", err);
                return;
            };
            // println!("{:?}", &results);
            for result in results.iter() {
                for item in result.iter() {
                    let mut s = String::new();
                    match &item.paramType {
                        git_lib::ParamType::LibName => {
                            for n in &item.name {
                                if n.len() == 0 {
                                    continue;
                                }
                                s.push_str(n);
                                if cfg!(target_os="windows") {
                                    s.push_str("\r");
                                }
                                s.push_str("\n");
                            }
                        },
                        git_lib::ParamType::TargetName => {
                            if item.name.len() == 0 {
                                continue;
                            }
                            let name = &item.name[0];
                            s.push_str(name);
                        },
                        git_lib::ParamType::LibPath => {
                            for n in &item.name {
                                if n.len() == 0 {
                                    continue;
                                }
                                s.push('"');
                                s.push_str("${CMAKE_CURRENT_SOURCE_DIR}/");
                                s.push_str(n);
                                s.push('"');
                                if cfg!(target_os="windows") {
                                    s.push_str("\r");
                                }
                                s.push_str("\n");
                            }
                        },
                        git_lib::ParamType::InstallLibPath => {
                            if item.name.len() == 0 {
                                continue;
                            }
                            let name = &item.name[0];
                            let mut na = String::from("../");
                            na.push_str(name);
                            s.push_str(&na);
                        },
                        git_lib::ParamType::Include => {
                            for n in &item.name {
                                if n.len() == 0 {
                                    continue;
                                }
                                s.push('"');
                                s.push_str("${CMAKE_CURRENT_SOURCE_DIR}/");
                                s.push_str(n);
                                s.push('"');
                                if cfg!(target_os="windows") {
                                    s.push_str("\r");
                                }
                                s.push_str("\n");
                            }
                            /*
                            let name = match &library.name {
                                Some(n) => n,
                                None => {
                                    println!("name is not exist");
                                    return;
                                }
                            };
                            match libs.get(name) {
                                Some(_) => {
                                },
                                None => {
                                    for n in &item.name {
                                        if n.len() == 0 {
                                            continue;
                                        }
                                        s.push('"');
                                        s.push_str(n);
                                        s.push('"');
                                        if cfg!(target_os="windows") {
                                            s.push_str("\r");
                                        }
                                        s.push_str("\n");
                                    }
                                    libs.insert(name.to_string());
                                }
                            }
                            */
                        }
                        _ => {}
                    }
                    match contents.get_mut(&item.startIndex) {
                        Some(c) => {
                            (*c).push_str(s.as_str());
                        },
                        None => {
                            contents.insert(item.startIndex, s);
                        }
                    }
                }
            }
            /*
            let mut indexs = HashMap::new();
            for (resultIndex, result) in results.iter().enumerate() {
                let mut indexStep = 0;
                for (itemIndex, item) in result.iter().enumerate() {
                    let start = content.len();
                    content.insert_str(item.startIndex + indexStep, &item.name);
                    let end = content.len();
                    indexs.insert(itemIndex, end - start);
                    indexStep += (end - start);
                }
            }
            */
            // println!("{:?}", &results);
        }
        // println!("{:?}", &contents);
        let mut tmps = Vec::new();
        for (key, value) in &contents {
            tmps.push(CContent{
                index: key,
                content: value
            });
        }
        quick_sort::sort(&mut tmps);
        let mut indexStep = 0;
        for item in &tmps {
            content.insert_str(item.index + indexStep, item.content);
            indexStep += item.content.len();
        }
        // println!("{:?}", &content);
        let p = parent.join(cmakelist_name);
        /*
        ** Write file
        */
        if let Err(err) = fs::write(p, content.as_bytes()) {
            println!("write CMakelists.txtx error, err:{}", err);
            return;
        };
    }

    fn pathReplace(&self, value: &str, cbbStoreRoot: &str, cmakeDir: &str) -> String {
        let mut pathResult = String::new();
        let path = Path::new(cbbStoreRoot);
        let mut afterPath = value.trim_left_matches(environments::keyword_cbb_store_root).to_string();
        let bytes = afterPath.as_bytes();
        if bytes.len() > 0 {
            let c = bytes[0];
            if c == b'/' || c == b'\\' {
                afterPath.remove(0);
            }
        }
        let path = path.join(&afterPath);
        /*
        ** Convert to absolute path
        */
        match path.canonicalize() {
            Ok(p) => {
                match p.to_str() {
                    Some(s) => {
                        let mut pt = String::new();
                        if cfg!(target_os="windows"){
                            let t = s.trim_left_matches(r#"\\?\"#);
                            let c = Path::new(cmakeDir).canonicalize().expect("cmakeDir abs path error").to_str().expect("cmakeDir abs path to_str error").trim_left_matches(r#"\\?\"#).to_string();
                            pt = pathconvert::abs2rel(&c, &t).replace("\\", r#"/"#);
                            // println!("{:?}", t);
                        } else {
                            let c = Path::new(cmakeDir).canonicalize().unwrap().to_str().unwrap().to_string();
                            pt = pathconvert::abs2rel(&c, s);
                        }
                        pathResult.push('"');
                        pathResult.push_str(&pt);
                        pathResult.push('"');
                    },
                    None => {
                        println!("[Error] include path abs to_str error");
                    }
                }
            },
            Err(err) => {
                println!("[Error] include path, path: {}", &value);
            }
        }
        /*
        match path.canonicalize() {
            Ok(p) => {
                match p.to_str() {
                    Some(s) => {
                        if cfg!(target_os="windows"){
                            let t = s.trim_left_matches(r#"\\?\"#).replace(r#"\"#, r#"\\"#);
                            pathResult.insert(self.content.len(), '"');
                            pathResult.insert_str(self.content.len(), &t);
                            pathResult.insert(self.content.len(), '"');
                        } else {
                            self.removeContentRightLen(value.len() + 1);
                            pathResult.insert_str(self.content.len(), s);
                        }
                    },
                    None => {
                        println!("[Error] include path abs to_str error");
                    }
                }
            },
            Err(err) => {
                println!("[Error] include path, path: {}", &value);
            }
        }
        // println!("{:?}, {}, {:?}", path.to_str(), afterPath, &self.path);
        */
        pathResult
    }


    fn updateIndex(&self, results: &mut Vec<Vec<CSearchResult>>, index: usize, len: usize) {
        for result in results.iter_mut() {
            for item in result.iter_mut() {
                if item.startIndex == index {
                } else if item.startIndex > index {
                    (*item).startIndex += len;
                } else {
                }
            }
        }
    }
}

impl CReplace {
    pub fn new() -> CReplace {
        CReplace{
            environmenter: CEnvironments::new()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[ignore]
    fn replaceTest() {
        let replacer = CReplace::new();
        replacer.replace("./doc/exe_cmake/CMakeLists.config", ".", ".", &structs::param::CSearchFilter{
            parentPathIsnotIncluded: None
        });
    }
}

// pub mod cmake;
pub mod environments;
pub mod merge;
