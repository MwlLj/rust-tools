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

pub struct CJoinParser {
}

impl CJoinParser {
    pub fn parse<F: IJoin>(&self, content: &str, f: &mut F) -> Result<(), &str> {
        let chars = content.chars();
        let mut symbolModeWhenBracket = SymbolMode::Normal;
        let mut symbolMode = SymbolMode::Normal;
        let mut extractMode = ExtractMode::Normal;
        let mut lastExtractMode = ExtractMode::Normal;
        let mut var = String::new();
        let mut field = String::new();
        let mut arrIndex = String::new();
        for c in chars {
            match symbolMode {
                SymbolMode::Normal => {
                    if c == '\'' {
                        symbolMode = SymbolMode::SingleQuote;
                    } else if c == '$' {
                        extractMode = ExtractMode::Var;
                    } else if c == '[' {
                        match extractMode {
                            ExtractMode::Var => {
                                // call on_var(var: &str)
                                // println!("on_var, {}", &var);
                                f.on_var(&var);
                                f.on_value(&var, "", ValueType::Var);
                                extractMode = ExtractMode::Normal;
                            },
                            ExtractMode::Field => {
                                // call on_field(var: &str)
                                // println!("on_field, {}, {}", &var, &field);
                                f.on_field(&var, &field);
                                f.on_value(&var, &field, ValueType::Object);
                                extractMode = ExtractMode::Normal;
                            },
                            _ => {
                            }
                        }
                        symbolMode = SymbolMode::Bracket;
                        symbolModeWhenBracket = SymbolMode::Normal;
                    } else {
                        match extractMode {
                            ExtractMode::Normal => {
                                // call on_ch(c: char)
                                // println!("on_ch, {}", c);
                                f.on_ch(c);
                                f.on_value(&var, &c.to_string(), ValueType::Char);
                            },
                            ExtractMode::Var => {
                                if c == '.' {
                                    // call on_var(var: &str)
                                    // println!("on_var, {}", &var);
                                    f.on_var(&var);
                                    f.on_value(&var, "", ValueType::Var);
                                    // call on_ch(c: char)
                                    // println!("on_ch, {}", c);
                                    f.on_ch(c);
                                    f.on_value(&var, &c.to_string(), ValueType::Char);
                                    var.clear();
                                    extractMode = ExtractMode::Normal;
                                } else {
                                    if c != ' ' && c != '\t' || c != '\r' || c != '\n' {
                                        var.push(c);
                                    }
                                }
                            },
                            ExtractMode::Field => {
                            },
                            _ => {}
                        }
                    }
                },
                SymbolMode::Dollar => {
                },
                SymbolMode::SingleQuote => {
                    if c == '\'' {
                        match lastExtractMode {
                            ExtractMode::Array => {
                            },
                            _ => {
                                // call on_field(field: &str)
                                // println!("on_field, {}, {}", &var, &field);
                                f.on_field(&var, &field);
                                f.on_value(&var, &field, ValueType::Object);
                            }
                        }
                        var.clear();
                        field.clear();
                        extractMode = ExtractMode::Normal;
                        symbolMode = SymbolMode::Normal;
                    } else if c == '$' {
                        extractMode = ExtractMode::Var;
                    } else if c == '[' {
                        match extractMode {
                            ExtractMode::Var => {
                                // call on_var(var: &str)
                                // println!("on_var, {}", &var);
                                f.on_var(&var);
                                f.on_value(&var, "", ValueType::Var);
                                // extractMode = ExtractMode::Normal;
                            },
                            ExtractMode::Field => {
                                // call on_field(var: &str)
                                // println!("on_field, {}, {}", &var, &field);
                                f.on_field(&var, &field);
                                f.on_value(&var, &field, ValueType::Object);
                                // extractMode = ExtractMode::Normal;
                            },
                            _ => {
                            }
                        }
                        symbolMode = SymbolMode::Bracket;
                        symbolModeWhenBracket = SymbolMode::SingleQuote;
                    } else {
                        match extractMode {
                            ExtractMode::Normal => {
                            },
                            ExtractMode::Var => {
                                if c == '.' {
                                    // call on_var(var: &str)
                                    // println!("on_var, {}", &var);
                                    f.on_var(&var);
                                    f.on_value(&var, "", ValueType::Var);
                                    extractMode = ExtractMode::Field;
                                    // var.clear();
                                } else {
                                    if c != ' ' && c != '\t' || c != '\r' || c != '\n' {
                                        var.push(c);
                                    }
                                }
                            },
                            ExtractMode::Field => {
                                if c == '.' {
                                    match lastExtractMode {
                                        ExtractMode::Array => {
                                            lastExtractMode = ExtractMode::Field;
                                        },
                                        _ => {
                                            // call on_field(field: &str)
                                            // println!("on_field, {}, {}", &var, &field);
                                            f.on_field(&var, &field);
                                            f.on_value(&var, &field, ValueType::Object);
                                        }
                                    }
                                    // var.clear();
                                    field.clear();
                                } else {
                                    if c != ' ' && c != '\t' || c != '\r' || c != '\n' {
                                        field.push(c);
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                },
                SymbolMode::Point => {
                },
                SymbolMode::Bracket => {
                    if c == ']' {
                        let index = match arrIndex.parse::<u32>() {
                            Ok(i) => i,
                            Err(err) => {
                                return Err("arr index error");
                            }
                        };
                        // call on_array(var: &str, index: u32)
                        // println!("on_array, {}, {}", &var, index);
                        f.on_arr(&var, &index);
                        f.on_value(&var, &arrIndex, ValueType::Array);
                        // var.clear();
                        field.clear();
                        arrIndex.clear();
                        match symbolModeWhenBracket {
                            SymbolMode::Normal => {
                                symbolMode = SymbolMode::Normal;
                                var.clear();
                            },
                            SymbolMode::SingleQuote => {
                                symbolMode = SymbolMode::SingleQuote;
                                lastExtractMode = ExtractMode::Array;
                            },
                            _ => {
                            }
                        }
                    } else {
                        arrIndex.push(c);
                    }
                }
            }
        }
        match extractMode {
            ExtractMode::Var => {
                f.on_var(&var);
                f.on_value(&var, "", ValueType::Var);
            },
            _ => {
            }
        }
        Ok(())
    }
}

impl CJoinParser {
    fn varCheck(&self, c: char, extractMode: &mut ExtractMode) {
    }
}

impl CJoinParser {
    pub fn new() -> CJoinParser {
        CJoinParser{}
    }
}

pub fn parse<'a, F: IJoin>(content: &'a str, f: &mut F) -> Result<(), &'a str> {
    let parser = CJoinParser::new();
    if let Err(err) = parser.parse(content, f) {
        return Err("parse error");
    };
    Ok(())
}

struct CDefaultJoin<'a, ValueF> {
    valueF: &'a mut ValueF,
}

impl<'a, ValueF> IJoin for CDefaultJoin<'a, ValueF>
    where ValueF: FnMut(&str, &str, ValueType) {
    fn on_value(&mut self, var: &str, t: &str, valueType: ValueType) {
        (self.valueF)(var, t, valueType);
    }
}

pub fn parse_with_fn<'a, ValueF>(content: &'a str, valueF: &mut ValueF) -> Result<(), &'a str>
    where ValueF: FnMut(&str, &str, ValueType) {
    let parser = CJoinParser::new();
    if let Err(err) = parser.parse(content, &mut CDefaultJoin{
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
    }

    #[test]
    #[ignore]
    fn joinParserTest() {
        // let s = "xxx.$platform.$extra[0].'$extra.name'.'$extra.objs[0].name.tests[1]'.'$extra.objs[0].name.tests[1]'.xxx.$version.yyy";
        let s = "abcd.'$extra.name'.'$extra.objs[0]'";
        println!("{:?}", s);
        let parser = CJoinParser::new();
        parser.parse(s, &mut CJoin{});
    }

    #[test]
    #[ignore]
    fn parseWithFnTest() {
        let s = "$platform";
        // let s = "$extra.objs[0]";
        // let s = "xxx.$platform.$extra[0].'$extra.name'.'$extra.objs[0].name.tests[1]'.'$extra.objs[0].name.tests[1]'.xxx.$version.yyy";
        parse_with_fn(s, &mut |var: &str, t: &str, valueType: ValueType| {
            println!("var: {}, t: {}", var, t);
        });
    }
}
