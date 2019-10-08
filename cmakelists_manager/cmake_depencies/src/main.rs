use cmake_depencies::replace::CReplace;
use rust_parse::cmd::CCmd;
use path::walk;

const keyword_cmake_file: &str = "CMakelists.config";

fn start(root: &str, libRoot: &str) {
    let replacer = CReplace::new();
    walk::walk_one_fn(root, &mut |path: &str, name: &str, t: walk::Type| -> bool {
        match t {
            walk::Type::Dir => {
            },
            walk::Type::File => {
                if name == keyword_cmake_file {
                    replacer.replace(path, libRoot);
                }
            }
        }
        true
    });
}

fn main() {
    let mut cmdRegister = CCmd::new();
    let root = cmdRegister.register_with_desc("-root", ".", "cmake search root");
    let libRoot = cmdRegister.register_with_desc("-lib-root", ".", "lib search root");
    cmdRegister.parse();

    let root = root.borrow();
    let libRoot = libRoot.borrow();

    start(&*root, &*libRoot);
}
