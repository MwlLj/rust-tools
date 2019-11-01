use crate::parse;
use crate::search;
use parse::git_librarys;
use parse::git_lib;
use search::dependencies::CDependSearcher;
use search::dependencies::CSearchResult;
use environments::CEnvironments;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

const cmakelist_name: &str = "CMakelists.txt";

pub struct CReplace {
    environmenter: CEnvironments
}

impl CReplace {
    pub fn replace(&self, cmakePath: &str, root: &str) -> Result<(), &str> {
        // self.search(root, content, librarys, params)
        let (librarys, params, mut content) = match self.environmenter.parse(cmakePath) {
            Ok(r) => r,
            Err(err) => {
                return Err("cmake parse error");
            }
        };
        // println!("{:?}", params);
        self.search(cmakePath, root, &mut content, &librarys, &params);
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
    fn search(&self, path: &str, root: &str, content: &mut String, librarys: &Vec<git_librarys::CGitLibrarys>, params: &Vec<git_lib::CParam>) {
        let mut contents: HashMap<usize, String> = HashMap::new();
        let mut libs: HashSet<String> = HashSet::new();
        for library in librarys.iter() {
            let mut results: Vec<Vec<CSearchResult>> = Vec::new();
            let searcher = CDependSearcher::new();
            if let Err(err) = searcher.search(&root, library, params, &mut results) {
                println!("[Error] search error, err: {}", err);
                return;
            };
            for result in results.iter() {
                for item in result.iter() {
                    let mut s = String::new();
                    match &item.paramType {
                        git_lib::ParamType::LibName => {
                            for n in &item.name {
                                s.push_str(n);
                                if cfg!(target_os="windows") {
                                    s.push_str("\r");
                                }
                                s.push_str("\n");
                            }
                        },
                        git_lib::ParamType::LibPath => {
                            for n in &item.name {
                                s.push('"');
                                s.push_str(n);
                                s.push('"');
                                if cfg!(target_os="windows") {
                                    s.push_str("\r");
                                }
                                s.push_str("\n");
                            }
                        },
                        git_lib::ParamType::Include => {
                            let name = match &library.name {
                                Some(n) => n,
                                None => {
                                    println!("name is not exist");
                                    return;
                                }
                            };
                            match libs.get(name) {
                                Some(_) => {},
                                None => {
                                    for n in &item.name {
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
                        },
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
        let p = Path::new(path);
        let parent = match p.parent() {
            Some(p) => p,
            None => {
                println!("get path parent path error");
                return;
            }
        };
        let p = parent.join(cmakelist_name);
        /*
        ** Write file
        */
        if let Err(err) = fs::write(p, content.as_bytes()) {
            println!("write CMakelists.txtx error, err:{}", err);
            return;
        };
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
        replacer.replace("./doc/exe_cmake/CMakelists.config", ".");
    }
}

// pub mod cmake;
pub mod environments;
