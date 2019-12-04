struct CReplaceParser {
}

enum Mode {
    Normal,
    Var
}

impl CReplaceParser {
    fn parse(&self, content: &str, datas: &Vec<&str>) -> String {
        let mut data = String::new();
        let mut mode = Mode::Normal;
        let mut index = String::new();
        let chars = content.chars();
        let dataLen = datas.len();
        for c in chars {
            match mode {
                Mode::Normal => {
                    if c == '<' {
                        mode = Mode::Var;
                    } else {
                        data.push(c);
                    }
                },
                Mode::Var => {
                    if c == '>' {
                        match index.parse::<usize>() {
                            Ok(idx) => {
                                match datas.get(idx) {
                                    Some(d) => {
                                        data.push_str(d);
                                    },
                                    None => {
                                        panic!("index overflow");
                                    }
                                }
                            },
                            Err(_) => {
                                continue;
                            }
                        };
                        mode = Mode::Normal;
                        index.clear();
                    } else {
                        if !c.is_ascii_whitespace() {
                            index.push(c);
                        }
                    }
                }
            }
        }
        data
    }
}

impl CReplaceParser {
    pub fn new() -> Self {
        Self{}
    }
}

pub fn parse(content: &str, datas: &Vec<&str>) -> String {
    let replacer = CReplaceParser::new();
    replacer.parse(content, datas)
}

mod test {
    use super::*;

    #[test]
    #[ignore]
    fn repeaceParseTest() {
        let replacer = CReplaceParser::new();
        let value = replacer.parse("hello {0}, hello {1}", &vec!["jake", "mike"]);
        println!("{:?}", value);
    }
}
