use json::JsonValue;

pub enum ValueType {
    Object,
    Array,
    Char,
    Var,
    Start,
    End,
    Str
}

pub trait IJoin {
    fn on_ch(&mut self, c: char) {}
    fn on_var(&mut self, var: &str) {}
    fn on_field(&mut self, var: &str, field: &str) {}
    fn on_arr(&mut self, var: &str, index: &u32) {}
    fn on_value(&mut self, t: &str, valueType: ValueType) {}
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
        // let mut jsonValue = &JsonValue::Null;
        let mut innerWord = String::new();
        let mut innerLastMode = InnerMode::Normal;
        let mut innerMode = InnerMode::Normal;
        for c in chars {
            match mode {
                Mode::Normal => {
                    if c == '`' {
                        mode = Mode::Block;
                    } else {
                        f.on_ch(c);
                    }
                },
                Mode::Block => {
                    match blockMode {
                        BlockMode::Normal => {
                            if c == '`' {
                                mode = Mode::Normal;
                            // } else if c == ' ' || c == '\r' || c == '\n' || c == '\t' {
                            } else if c == '"' || c == '\'' {
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
                                } else {
                                    blockMode = BlockMode::Str;
                                }
                            } else {
                                word.push(c);
                            }
                        },
                        BlockMode::Json => {
                            if c == '"' || c == '\'' {
                                // jsonValue = &jsonValue[&innerWord];
                                // f.on_json_end(jsonValue);
                                match innerLastMode {
                                    InnerMode::Array => {},
                                    _ => {
                                        f.on_value(&innerWord, ValueType::End);
                                    }
                                }
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
                                            // jsonValue = f.json_value(&innerWord);
                                            f.on_value(&innerWord, ValueType::Start);
                                            innerWord.clear();
                                            innerMode = InnerMode::Array;
                                        } else if c == '.' {
                                            // jsonValue = f.json_value(&innerWord);
                                            f.on_value(&innerWord, ValueType::Start);
                                            innerWord.clear();
                                            innerMode = InnerMode::Object;
                                        } else {
                                            innerWord.push(c);
                                        }
                                    },
                                    InnerMode::Object => {
                                        if c == '.' {
                                            // jsonValue = &jsonValue[&innerWord];
                                            // f.on_json_value(jsonValue);
                                            match innerLastMode {
                                                InnerMode::Array => {},
                                                _ => {
                                                    f.on_value(&innerWord, ValueType::Object);
                                                }
                                            }
                                            innerWord.clear();
                                            innerLastMode = InnerMode::Object;
                                        } else if c == '[' {
                                            f.on_value(&innerWord, ValueType::Start);
                                            innerWord.clear();
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
                                            // jsonValue = &jsonValue[index];
                                            // f.on_json_value(jsonValue);
                                            f.on_value(&index.to_string(), ValueType::Array);
                                            innerWord.clear();
                                            innerMode = InnerMode::Object;
                                            innerLastMode = InnerMode::Array;
                                        } else {
                                            innerWord.push(c);
                                        }
                                    }
                                }
                                // self.parse_json(c, innerMode, &mut innerWord, f);
                            }
                        },
                        BlockMode::Str => {
                            if c == '"' || c == '\'' {
                                f.on_value(&innerWord, ValueType::Str);
                                blockMode = BlockMode::Normal;
                                innerMode = InnerMode::Normal;
                                innerWord.clear();
                            } else {
                                // handle string
                                innerWord.push(c);
                            }
                        },
                        BlockMode::Judge => {
                            if c == '"' || c == '\'' {
                                blockMode = BlockMode::Normal;
                                innerMode = InnerMode::Normal;
                                innerWord.clear();
                            } else {
                                // parse judge grammar
                                // self.parse_judge(c, &mut innerMode, &mut innerWord, f);
                            }
                        },
                        BlockMode::Var => {
                            if c == '"' || c == '\'' {
                                f.on_value(&innerWord, ValueType::Var);
                                blockMode = BlockMode::Normal;
                                innerMode = InnerMode::Normal;
                                innerWord.clear();
                            } else {
                                // handle var
                                innerWord.push(c);
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

struct CJsonJoin<'a, ValueF> {
    valueF: &'a mut ValueF,
}

impl<'a, ValueF> IJoin for CJsonJoin<'a, ValueF>
    where ValueF: FnMut(&str, ValueType) {
    fn on_ch(&mut self, c: char) {
        (self.valueF)(&c.to_string(), ValueType::Char);
    }
    fn on_value(&mut self, t: &str, valueType: ValueType) {
        (self.valueF)(t, valueType);
    }
}

pub fn parse<'a, ValueF>(content: &'a str, valueF: &mut ValueF) -> Result<(), &'a str>
    where ValueF: FnMut(&str, ValueType) {
    let parser = CJoinParser::new();
    if let Err(err) = parser.parse(content, &mut CJsonJoin{
        valueF: valueF
    }) {
        return Err("parse error");
    };
    Ok(())
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
        fn on_value(&mut self, t: &str, valueType: ValueType) {
            println!("t: {}", t);
        }
    }

    #[test]
    #[ignore]
    fn joinParserTest() {
        // let s = "xxx.$platform.$extra[0].'$extra.name'.'$extra.objs[0].name.tests[1]'.'$extra.objs[0].name.tests[1]'.xxx.$version.yyy";
        /*
        let s = r#"
        `json:"extra.name"`/
        `json:"extra"`/
        `json:"extra.objs[0].name"`/
        `json:"extra[0].name[0]"`/
        "#;
        */
        // let s = "`str:\"hello\"`";
        let s = "`var:'config'``var:'version'`";
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
