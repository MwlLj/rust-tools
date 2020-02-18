use json::JsonValue;

const keyword_if: &str = "if";
const keyword_elseif: &str = "elseif";
const keyword_else: &str = "else";

#[derive(Clone, Debug)]
pub enum ParseMode {
    Normal,
    JudgeSub
}

#[derive(Clone, Debug)]
pub enum CondType {
    Json,
    Str,
    Judge,
    Var,
    Symbol,
    Else
}

#[derive(Clone, Debug)]
pub enum ValueType {
    Object,
    Array,
    Char,
    Var,
    Start,
    EndAfterArray,
    End,
    Str,
    JudgeBody,
    Condition(CondType)
}

#[derive(Clone, Debug)]
pub enum ValueCode {
    Normal,
    ContinueToJudge,
    DonotContinueJudge
}

pub enum ValueError {
    Unknow
}

pub trait IJoin {
    fn on_ch(&mut self, c: char, parseMode: &ParseMode) {}
    fn on_var(&mut self, var: &str) {}
    fn on_field(&mut self, var: &str, field: &str) {}
    fn on_arr(&mut self, var: &str, index: &u32) {}
    fn on_value(&mut self, t: &str, parseMode: &ParseMode, valueType: ValueType) -> Result<ValueCode, ValueError> {
        Ok(ValueCode::Normal)
    }
}

enum Mode {
    // char mode
    Normal,
    // `` mode
    Block
}

#[derive(Clone)]
enum BlockMode {
    Normal,
    Json,
    Str,
    Judge,
    Var,
    Symbol
}

#[derive(Clone)]
enum InnerMode {
    Normal,
    Object,
    Array,
    If,
    ElseIf,
    Else,
    JudgeBody,
    JudgeEnd
}

const block_mode_json: &str = "json:";
const block_mode_str: &str = "str:";
const block_mode_judge: &str = "judge:";
const block_mode_var: &str = "var:";

pub struct CJoinParser {
}

impl CJoinParser {
    pub fn parse<F: IJoin>(&self, content: &str, parseMode: &ParseMode, f: &mut F) -> Result<(), &str> {
        let chars = content.chars();
        let mut mode = Mode::Normal;
        let mut blockMode = BlockMode::Normal;
        let mut blockModePhotograph = blockMode.clone();
        let mut word = String::new();
        let mut blockChar: char = '"';
        let mut blockCharPhotograph: char = blockChar;
        let mut valueCode = ValueCode::Normal;
        // let mut jsonValue = &JsonValue::Null;
        let mut innerWord = String::new();
        let mut innerLastMode = InnerMode::Normal;
        let mut innerMode = InnerMode::Normal;
        let mut innerWordPhotograph = innerWord.clone();
        let mut innerLastModePhotograph = innerLastMode.clone();
        let mut innerModePhotograph = innerMode.clone();
        let mut end = '`';
        for c in chars {
            match mode {
                Mode::Normal => {
                    if c == '`' || c == '~' {
                        mode = Mode::Block;
                        end = c;
                    } else {
                        if !c.is_ascii_whitespace() {
                            f.on_ch(c, parseMode);
                        }
                    }
                },
                Mode::Block => {
                    match blockMode {
                        BlockMode::Normal => {
                            if c == end {
                                mode = Mode::Normal;
                            } else if c == '"' || c == '\'' || c == '^' {
                                blockChar = c;
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
                            if c == blockChar {
                                // jsonValue = &jsonValue[&innerWord];
                                // f.on_json_end(jsonValue);
                                match innerLastMode {
                                    InnerMode::Array => {
                                        f.on_value("", parseMode, ValueType::EndAfterArray);
                                    },
                                    _ => {
                                        f.on_value(&innerWord, parseMode, ValueType::End);
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
                                            f.on_value(&innerWord, parseMode, ValueType::Start);
                                            innerWord.clear();
                                            innerMode = InnerMode::Array;
                                        } else if c == '.' {
                                            // jsonValue = f.json_value(&innerWord);
                                            f.on_value(&innerWord, parseMode, ValueType::Start);
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
                                                    f.on_value(&innerWord, parseMode, ValueType::Object);
                                                }
                                            }
                                            innerWord.clear();
                                            innerLastMode = InnerMode::Object;
                                        } else if c == '[' {
                                            f.on_value(&innerWord, parseMode, ValueType::Object);
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
                                            f.on_value(&index.to_string(), parseMode, ValueType::Array);
                                            innerWord.clear();
                                            innerMode = InnerMode::Object;
                                            innerLastMode = InnerMode::Array;
                                        } else {
                                            innerWord.push(c);
                                        }
                                    },
                                    _ => {}
                                }
                                // self.parse_json(c, innerMode, &mut innerWord, f);
                            }
                        },
                        BlockMode::Str => {
                            if c == blockChar {
                                f.on_value(&innerWord, parseMode, ValueType::Str);
                                blockMode = BlockMode::Normal;
                                innerMode = InnerMode::Normal;
                                innerWord.clear();
                            } else {
                                // handle string
                                innerWord.push(c);
                            }
                        },
                        BlockMode::Judge => {
                            if c == blockChar {
                                blockMode = BlockMode::Normal;
                                innerMode = InnerMode::Normal;
                                valueCode = ValueCode::Normal;
                                innerWord.clear();
                            } else {
                                // parse judge grammar
                                match innerMode {
                                    InnerMode::Normal => {
                                        if c.is_ascii_whitespace() {
                                            if innerWord == keyword_if {
                                                innerMode = InnerMode::If;
                                            } else if innerWord == keyword_elseif {
                                                innerMode = InnerMode::ElseIf;
                                            } else if innerWord == keyword_else {
                                                innerMode = InnerMode::Else;
                                            }
                                            innerWord.clear();
                                        } else {
                                            innerWord.push(c);
                                        }
                                    },
                                    InnerMode::If
                                        | InnerMode::ElseIf => {
                                        if c == '{' {
                                            innerMode = InnerMode::JudgeBody;
                                        } else if c.is_ascii_whitespace() || c == '{' {
                                            let (b, cm, bc) = self.start_is_keyword(&innerWord);
                                            if b {
                                                innerWord.insert(0, '`');
                                                innerWord.push('`');
                                                if let Err(e) = self.parse(&innerWord, &ParseMode::JudgeSub, f) {
                                                    return Err(e);
                                                };
                                                valueCode = match f.on_value("", parseMode, ValueType::Condition(cm)) {
                                                    Ok(c) => c,
                                                    Err(err) => {
                                                        return Err("on value error");
                                                    }
                                                };
                                                // println!("innerWord: {}, {:?}", &innerWord, valueCode);
                                            } else {
                                                f.on_value(&innerWord, parseMode, ValueType::Condition(CondType::Symbol));
                                            }
                                            innerWord.clear();
                                        } else {
                                            innerWord.push(c);
                                        }
                                    },
                                    InnerMode::Else => {
                                        if c == '{' {
                                            f.on_value("", parseMode, ValueType::Condition(CondType::Else));
                                            innerMode = InnerMode::JudgeBody;
                                            valueCode = ValueCode::DonotContinueJudge;
                                        } else {
                                        }
                                    },
                                    InnerMode::JudgeBody => {
                                        if c == '}' {
                                            match valueCode {
                                                ValueCode::DonotContinueJudge => {
                                                    innerMode = InnerMode::JudgeEnd;
                                                },
                                                _ => {
                                                    innerMode = InnerMode::Normal;
                                                }
                                            }
                                        } else if c.is_ascii_whitespace() {
                                            match valueCode {
                                                ValueCode::DonotContinueJudge => {
                                                    if innerWord.len() > 0 {
                                                        f.on_value(&innerWord, parseMode, ValueType::JudgeBody);
                                                    }
                                                },
                                                _ => {}
                                            }
                                            innerWord.clear();
                                        } else {
                                            if !c.is_ascii_whitespace() {
                                                innerWord.push(c);
                                            }
                                        }
                                    },
                                    InnerMode::JudgeEnd => {
                                    },
                                    _ => {}
                                }
                                // self.parse_judge(c, &mut innerMode, &mut innerWord, f);
                            }
                        },
                        BlockMode::Var => {
                            if c == blockChar {
                                f.on_value(&innerWord, parseMode, ValueType::Var);
                                blockMode = BlockMode::Normal;
                                innerMode = InnerMode::Normal;
                                innerWord.clear();
                            } else {
                                // handle var
                                innerWord.push(c);
                            }
                        },
                        _ => {}
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

    fn start_is_keyword(&self, word: &str) -> (bool, CondType, char) {
        let mut condType = CondType::Str;
        if word.starts_with(block_mode_var) {
            condType = CondType::Var;
            if word.len() > block_mode_var.len() {
                (true, condType, word.as_bytes()[block_mode_var.len()] as char)
            } else {
                (false, condType, '"')
            }
        } else if word.starts_with(block_mode_str) {
            condType = CondType::Str;
            if word.len() > block_mode_var.len() {
                (true, condType, word.as_bytes()[block_mode_str.len()] as char)
            } else {
                (false, condType, '"')
            }
        } else if word.starts_with(block_mode_judge) {
            condType = CondType::Judge;
            if word.len() > block_mode_var.len() {
                (true, condType, word.as_bytes()[block_mode_judge.len()] as char)
            } else {
                (false, condType, '"')
            }
        } else if word.starts_with(block_mode_json) {
            condType = CondType::Json;
            if word.len() > block_mode_var.len() {
                (true, condType, word.as_bytes()[block_mode_json.len()] as char)
            } else {
                (false, condType, '"')
            }
        } else {
            (false, condType, '"')
        }
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
    where ValueF: FnMut(&str, &ParseMode, ValueType) -> Result<ValueCode, ValueError> {
    fn on_ch(&mut self, c: char, parseMode: &ParseMode) {
        (self.valueF)(&c.to_string(), parseMode, ValueType::Char);
    }
    fn on_value(&mut self, t: &str, parseMode: &ParseMode, valueType: ValueType) -> Result<ValueCode, ValueError> {
        (self.valueF)(t, parseMode, valueType)
    }
}

pub fn parse<'a, ValueF>(content: &'a str, valueF: &mut ValueF) -> Result<(), &'a str>
    where ValueF: FnMut(&str, &ParseMode, ValueType) -> Result<ValueCode, ValueError> {
    let parser = CJoinParser::new();
    if let Err(err) = parser.parse(content, &ParseMode::Normal, &mut CJsonJoin{
        valueF: valueF
    }) {
        println!("parse error, err: {}", err);
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
        fn on_ch(&mut self, c: char, parseMode: &ParseMode) {
            // println!("on_ch, {}", c);
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
        fn on_value(&mut self, t: &str, parseMode: &ParseMode, valueType: ValueType) -> Result<ValueCode, ValueError> {
            println!("t: {}", t);
            Ok(ValueCode::Normal)
        }
    }

    #[test]
    #[ignore]
    fn jsonJoinParserTest() {
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
        // let s = "`var:'config'``var:'version'`";
        let s = "abcd.`json:'extra.name'`.`json:'extra.objs[0]'`";
        println!("{:?}", s);
        let parser = CJoinParser::new();
        parser.parse(s, &ParseMode::Normal, &mut CJoin{
            jsonValue: json::parse(r#"
            {
                "extra": {
                    "name": "jake",
                    "objs": ["one", "two", "third"],
                    "dr": "debug"
                }
            }
                "#).unwrap(),
            firstKey: String::new()
        });
    }

    #[test]
    #[ignore]
    fn judgeJoinParserTest() {
        let s = r#"
        `judge:"
        if json:'extra.name' == str:'win64' {
            64
        } elseif json:'extra.dr' == str:'debug' {
            _d
        } else {
            _
        }
        "`
        "#;
        println!("{:?}", s);
        parse(s, &mut |t: &str, parseMode: &ParseMode, valueType: ValueType| -> Result<ValueCode, ValueError> {
            println!("{:?}, {:?}", t, valueType);
            Ok(ValueCode::Normal)
        });
        /*
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
        */
    }
}
