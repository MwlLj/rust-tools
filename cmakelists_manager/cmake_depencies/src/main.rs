use cmake_depencies::replace::{CReplace, keyword_cmake_file};
use cmake_depencies::structs;
use rust_parse::cmd::CCmd;
use path::walk;

fn start(root: &str, libRoot: &str, cbbStoreRoot: &str, searchFilter: &structs::param::CSearchFilter) {
    let replacer = CReplace::new();
    walk::walk_one_fn(root, &mut |path: &str, name: &str, t: walk::Type| -> bool {
        match t {
            walk::Type::Dir => {
            },
            walk::Type::File => {
                if name == keyword_cmake_file {
                    replacer.replace(path, libRoot, cbbStoreRoot, searchFilter);
                }
            }
        }
        true
    });
}

fn main() {
    let keyword_cbb_store_root = "-cbb-store-root";
    let keyword_parent_notincludes = "-parent-notincludes";
    let mut cmdRegister = CCmd::new();
    let root = cmdRegister.register_with_desc("-root", ".", "cmake search root");
    let libRoot = cmdRegister.register_with_desc("-lib-root", ".", "lib search root");
    let cbbStoreRoot = cmdRegister.register_with_desc(keyword_cbb_store_root, ".", "dbb store root, default == lib-root");
    let parentPathIsnotIncluded = cmdRegister.register_with_desc(keyword_parent_notincludes, "cbb,third,.git", "Parent path is not included");
    cmdRegister.parse();

    if !cmdRegister.has(keyword_cbb_store_root) {
        *cbbStoreRoot.borrow_mut() = libRoot.borrow().to_string();
    }

    let root = root.borrow();
    let libRoot = libRoot.borrow();
    let cbbStoreRoot = cbbStoreRoot.borrow();
    let parentPathIsnotIncluded = parentPathIsnotIncluded.borrow();

    let v: Vec<&str> = parentPathIsnotIncluded.split(",").collect();
    let mut vs = Vec::new();
    for item in v.iter() {
        vs.push(item.to_string());
    }
    let mut parentNotIncluded = None;
    if vs.len() > 0 {
        parentNotIncluded = Some(vs);
    }
    println!("{:?}", &parentNotIncluded);

    start(&*root, &*libRoot, &*cbbStoreRoot, &structs::param::CSearchFilter{
        parentPathIsnotIncluded: parentNotIncluded
    });
}
