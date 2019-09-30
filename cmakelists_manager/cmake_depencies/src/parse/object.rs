/*
** parse example:
** { name = awnet_client, version = 0.2.1, platform = ${FILE_PREFIX}, extra = '' }
*/

pub trait IObject {
    fn on_kv(&mut self, key: &str, value: &str) {}
}

enum SymbolMode {
    Normal,
    BigBrackets,
    Equal,
    Comma,
    SingleQuote
}

enum WordMode {
    Normal,
    Key,
    Value
}

pub struct CObjectParser {
}

impl CObjectParser {
    pub fn parse<F: IObject>(&self, content: &str, f: &mut F) {
        let chars = content.chars();
        let mut symbolMode = SymbolMode::Normal;
        let mut wordMode = WordMode::Normal;
        let mut key = String::new();
        let mut value = String::new();
        let mut bigBracketsCount = 0;
        for c in chars {
            match symbolMode {
                SymbolMode::Normal => {
                    if c == '{' {
                        symbolMode = SymbolMode::BigBrackets;
                        wordMode = WordMode::Key;
                        bigBracketsCount += 1;
                    }
                },
                SymbolMode::BigBrackets => {
                    if c == '\'' {
                        symbolMode = SymbolMode::SingleQuote;
                    } else if c == '=' {
                        wordMode = WordMode::Value;
                    } else if c == '}' && bigBracketsCount == 1 {
                        if bigBracketsCount == 1 {
                            // call fn(k, v)
                            f.on_kv(&key, value.trim_end());
                            // println!("k: {}, v: {}", &key, &value);
                            break;
                        }
                    } else if c == ',' {
                        // call fn(k, v)
                        f.on_kv(&key, value.trim_end());
                        // println!("k: {}, v: {}", &key, &value);
                        key.clear();
                        value.clear();
                        wordMode = WordMode::Key;
                    } else {
                        match wordMode {
                            WordMode::Normal => {
                            },
                            WordMode::Key => {
                                if c != ' ' && c != '\n' && c != '\r' && c != '\t' {
                                    key.push(c);
                                }
                            },
                            WordMode::Value => {
                                if value.len() == 0 && (c == ' ' || c == '\n' || c == '\r' || c == '\t') {
                                } else {
                                    if c == '{' {
                                        bigBracketsCount += 1;
                                    } else if c == '}' {
                                        bigBracketsCount -= 1;
                                    }
                                    value.push(c);
                                }
                            }
                        }
                    }
                },
                SymbolMode::Equal => {
                },
                SymbolMode::Comma => {
                },
                SymbolMode::SingleQuote => {
                    if c == '\'' {
                        symbolMode = SymbolMode::BigBrackets;
                    } else {
                        match wordMode {
                            WordMode::Normal => {
                            },
                            WordMode::Key => {
                                if c != ' ' && c != '\n' && c != '\r' && c != '\t' {
                                    key.push(c);
                                }
                            },
                            WordMode::Value => {
                                if value.len() == 0 && (c == ' ' || c == '\n' || c == '\r' || c == '\t') {
                                } else {
                                    if c == '{' {
                                        bigBracketsCount += 1;
                                    } else if c == '}' {
                                        bigBracketsCount -= 1;
                                    }
                                    value.push(c);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl CObjectParser {
    fn popEndSpace(&self, content: &mut str) {
        content.trim_end();
    }
}

impl CObjectParser {
    pub fn new() -> CObjectParser {
        CObjectParser{}
    }
}

#[cfg(test)]
mod test {
    use super::*;
    struct CObject {
    }
    impl IObject for CObject {
        fn on_kv(&mut self, key: &str, value: &str) {
            println!("key: {}, value: {}", key, value);
        }
    }
    #[test]
    #[ignore]
    fn objectParserTest() {
        let parser = CObjectParser::new();
        let mut f = CObject{};
        parser.parse(r#"{
            name = awnet_client,
            version = 0.2.1,
            platform = ${FILE_PREFIX},
            extra = '{
                "name": "jake",
                "age": 20
            }',
            xxx = '{
                "name": "jake",
                "age": 20
            }'
        }"#, &mut f);
    }
}
