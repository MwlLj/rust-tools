use cmake_depencies::replace::CReplace;
use rust_parse::cmd::CCmd;
use path::walk;

const keyword_cmake_file: &str = "CMakeLists.config";

fn start(root: &str, libRoot: &str, cbbStoreRoot: &str) {
    let replacer = CReplace::new();
    walk::walk_one_fn(root, &mut |path: &str, name: &str, t: walk::Type| -> bool {
        match t {
            walk::Type::Dir => {
            },
            walk::Type::File => {
                if name == keyword_cmake_file {
                    replacer.replace(path, libRoot, cbbStoreRoot);
                }
            }
        }
        true
    });
}

fn main() {
    let keyword_cbb_store_root = "-cbb-store-root";
    let mut cmdRegister = CCmd::new();
    let root = cmdRegister.register_with_desc("-root", ".", "cmake search root");
    let libRoot = cmdRegister.register_with_desc("-lib-root", ".", "lib search root");
    let cbbStoreRoot = cmdRegister.register_with_desc(keyword_cbb_store_root, ".", "dbb store root, default == lib-root");
    cmdRegister.parse();

    if !cmdRegister.has(keyword_cbb_store_root) {
        *cbbStoreRoot.borrow_mut() = libRoot.borrow().to_string();
    }

    let root = root.borrow();
    let libRoot = libRoot.borrow();
    let cbbStoreRoot = cbbStoreRoot.borrow();

    start(&*root, &*libRoot, &*cbbStoreRoot);
}
