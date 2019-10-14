use path::walk;
use rust_parse::cmd::CCmd;

use std::path::Path;
use std::fs;

const cargo_toml: &str = "Cargo.toml";
const cargo_lock: &str = "Cargo.lock";
const target: &str = "target";

fn walk(root: &str) {
    walk::walk_one_fn(root, &mut |path: &str, name: &str, t: walk::Type| -> bool {
        match t {
            walk::Type::Dir => {
                true
            },
            walk::Type::File => {
                // file
                if name != cargo_toml {
                    return true;
                }
                let dir = match Path::new(path).parent() {
                    Some(d) => d,
                    None => {
                        return true;
                    }
                };
                // remove Cargo.lock
                fs::remove_file(dir.join(cargo_lock));
                // remove target
                fs::remove_dir_all(dir.join(target));
                true
            }
        }
    });
}

fn main() {
    let mut cmdHandler = CCmd::new();
    let root = cmdHandler.register_with_desc("-root", ".", "root");
    cmdHandler.parse();

    let root = root.borrow();

    walk(&*root);
}
