use rust_parse::cmd::CCmd;
use path::walk;

use std::fs;

const keyword_cmake_file: &str = "CMakeLists.txt";

fn start(root: &str) {
    walk::walk_one_fn(root, &mut |path: &str, name: &str, t: walk::Type| -> bool {
        match t {
            walk::Type::Dir => {
            },
            walk::Type::File => {
                if name == keyword_cmake_file {
                    println!("remove: {:?}", path);
                    fs::remove_file(path);
                }
            }
        }
        true
    });
}

fn main() {
    let mut cmdRegister = CCmd::new();
    let root = cmdRegister.register_with_desc("-root", ".", "root");
    cmdRegister.parse();

    let root = root.borrow();

    start(&*root);
}
