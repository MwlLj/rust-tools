use searcher::keyword::finder;

use rust_parse::cmd;

const mode_keyword: &str = "keyword";

fn main() {
    let mut cmdHandler = cmd::CCmd::new();
    let root = cmdHandler.register_with_desc("-root", ".", "root path");
    let mode = cmdHandler.register_with_desc("-mode", "", "mode: keyword");
    let keyword = cmdHandler.register_with_desc("-keyword", "", "keyword");
    cmdHandler.parse();

    let mode = mode.borrow();
    if *mode == "" {
        println!("mode is null");
        return;
    }
    match mode.as_str() {
        mode_keyword => {
            let f = finder::CFinder::new();
            let root = root.borrow();
            let keyword = keyword.borrow();
            if *keyword == "" {
                println!("keyword is empty");
                return;
            }
            f.find(&*root, &*keyword);
        },
        _ => {
        }
    }
}
