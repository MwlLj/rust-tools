use json::JsonValue;

pub enum ValueType {
    Object,
    Array,
    Char,
    Var
}

pub trait IJoin {
    fn on_ch(&mut self, c: char) {}
    fn on_var(&mut self, var: &str) {}
    fn on_field(&mut self, var: &str, field: &str) {}
    fn on_arr(&mut self, var: &str, index: &u32) {}
    fn on_value(&mut self, var: &str, t: &str, valueType: ValueType) {}
    fn json_value(&self, var: &str) -> &JsonValue;
    fn on_json_value(&mut self, jsonValue: &JsonValue) {}
    fn on_json_end(&mut self, jsonValue: &JsonValue) {}
}

enum SymbolMode {
    Normal,
    // $
    Dollar,
    // '
    SingleQuote,
    // .
    Point,
    // [
    Bracket
}

enum ExtractMode {
    Normal,
    // $xxx
    Var,
    // $xxx.yyy -> yyy
    Field,
    // []
    Array
}

enum Mode {
    // char mode
    Normal,
    // `` mode
    Block
}

enum BlockMode {
    Normal,
    Json,
    Str,
    Judge,
    Var
}

#[derive(Clone)]
enum InnerMode {
    Normal,
    Object,
    Array
}

const block_mode_json: &str = "json:";
const block_mode_str: &str = "str:";
const block_mode_judge: &str = "judge:";
const block_mode_var: &str = "var:";

pub struct CJoinParser {
}

impl CJoinParser {
    pub fn parse<F: IJoin>(&self, content: &str, f: &mut F) -> Result<(), &str> {
        let chars = content.chars();
        let mut mode = Mode::Normal;
        let mut blockMode = BlockMode::Normal;
        let mut word = String::new();
        let mut jsonValue = &JsonValue::Null;
        let mut innerWord = String::new();
        let mut innerMode = InnerMode::Normal;
        for c in chars {
            match mode {
                Mode::Normal => {
                    if c == '`' {
                        mode = Mode::Block;
                    }
                },
                Mode::Block => {
                    match blockMode {
                        BlockMode::Normal => {
                            if c == '`' {
                                mode = Mode::Normal;
                            // } else if c == ' ' || c == '\r' || c == '\n' || c == '\t' {
                            } else if c == '"' {
                                if word.len() > 0 {
                                    let w = word.trim();
                                    if w == block_mode_str {
                                        blockMode = BlockMode::Str;
                                    } else if w == block_mode_json {
                                        blockMode = BlockMode::Json;
                                    } else if w == block_mode_judge {
                                        blockMode = BlockMode::Judge;
                                    } else if w == block_mode_var {
                                        blockMode = BlockMode::Var;
                                    }
                                    word.clear();
                                }
                            } else {
                                word.push(c);
                            }
                        },
                        BlockMode::Json => {
                            if c == '"' {
                                jsonValue = &jsonValue[&innerWord];
                                f.on_json_end(jsonValue);
                                blockMode = BlockMode::Normal;
                                innerMode = InnerMode::Normal;
                                innerWord.clear();
                            } else {
                                /*
                                ** parse json values:
                                ** configs[0] / param.configs[0] / param.net
                                */
                                match innerMode {
                                    InnerMode::Normal => {
                                        if c == '[' {
                                            jsonValue = f.json_value(&innerWord);
                                            innerMode = InnerMode::Array;
                                        } else if c == '.' {
                                            jsonValue = f.json_value(&innerWord);
                                            innerMode = InnerMode::Object;
                                        } else {
                                            innerWord.push(c);
                                        }
                                    },
                                    InnerMode::Object => {
                                        if c == '.' {
                                            jsonValue = &jsonValue[&innerWord];
                                            f.on_json_value(jsonValue);
                                        } else if c == '[' {
                                            innerMode = InnerMode::Array;
                                        } else {
                                            innerWord.push(c);
                                        }
                                    },
                                    InnerMode::Array => {
                                        if c == ']' {
                                            let index = match innerWord.parse::<usize>() {
                                                Ok(i) => i,
                                                Err(err) => {
                                                    return Err("index is invalid");
                                                }
                                            };
                                            jsonValue = &jsonValue[index];
                                            f.on_json_value(jsonValue);
                                            innerMode = InnerMode::Object;
                                        } else {
                                            innerWord.push(c);
                                        }
                                    }
                                }
                                // self.parse_json(c, innerMode, &mut innerWord, f);
                            }
                        },
                        BlockMode::Str => {
                            if c == '"' {
                                blockMode = BlockMode::Normal;
                                innerMode = InnerMode::Normal;
                                innerWord.clear();
                            } else {
                                // handle string
                            }
                        },
                        BlockMode::Judge => {
                            if c == '"' {
                                blockMode = BlockMode::Normal;
                                innerMode = InnerMode::Normal;
                                innerWord.clear();
                            } else {
                                // parse judge grammar
                                // self.parse_judge(c, &mut innerMode, &mut innerWord, f);
                            }
                        },
                        BlockMode::Var => {
                            if c == '"' {
                                blockMode = BlockMode::Normal;
                                innerMode = InnerMode::Normal;
                                innerWord.clear();
                            } else {
                                // handle var
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl CJoinParser {
    fn parse_json<F: IJoin>(&self, c: char, innerMode: &mut InnerMode, innerWord: &mut String, f: &mut F) {
        /*
        ** example:
        ** configs[0] / param.configs[0] / param.net
        */
        // match innerMode {
        //     InnerMode::Normal => {
        //         if c == '[' || c == '.' {
        //             innerMode = InnerMode::NotFirst(innerWord);
        //         }
        //     },
        //     InnerMode::NotFirst(_) => {
        //     }
        // }
    }

    fn parse_judge<F: IJoin>(&self, c: char, innerMode: &mut InnerMode, innerWord: &mut String, f: &mut F) {
    }
}

impl CJoinParser {
    pub fn new() -> CJoinParser {
        CJoinParser{}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct CJoin {
        jsonValue: JsonValue,
        firstKey: String
    }

    impl IJoin for CJoin {
        fn on_ch(&mut self, c: char) {
            println!("on_ch, {}", c);
        }
        fn on_var(&mut self, var: &str) {
            println!("on_var, {}", var);
        }
        fn on_field(&mut self, var: &str, field: &str) {
            println!("on_field, {}, {}", var, field);
        }
        fn on_arr(&mut self, var: &str, index: &u32) {
            println!("on_arr, {}, {}", var, index);
        }
        fn json_value(&self, var: &str) -> &JsonValue {
            &self.jsonValue
        }
        fn on_json_value(&mut self, jsonValue: &JsonValue) {
        }
        fn on_json_end(&mut self, jsonValue: &JsonValue) {
        }
    }

    #[test]
    // #[ignore]
    fn joinParserTest() {
        // let s = "xxx.$platform.$extra[0].'$extra.name'.'$extra.objs[0].name.tests[1]'.'$extra.objs[0].name.tests[1]'.xxx.$version.yyy";
        let s = "`json:\"extra.name\"``json:\"extra.objs[0]\"`";
        println!("{:?}", s);
        let parser = CJoinParser::new();
        parser.parse(s, &mut CJoin{
            jsonValue: json::parse(r#"
            {
                "extra": {
                    "name": "jake",
                    "objs": ["one", "two", "third"]
                }
            }
                "#).unwrap(),
            firstKey: String::new()
        });
    }
}
