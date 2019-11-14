pub enum ValueType {
    Var,
    Char
}

pub trait IRule {
    fn on_field(&mut self, field: &str) {}
    fn on_str(&mut self, c: char) {}
    fn on_value(&mut self, t: &str, valueType: ValueType) {}
}

enum WordMode {
    Normal,
    // $
    Dollar,
    // string
    Str
}

pub struct CRuleParser {
}

impl CRuleParser {
    pub fn parse<F: IRule>(&self, content: &str, f: &mut F) {
        let chars = content.chars();
        let mut lastWordMode = WordMode::Normal;
        let mut wordMode = WordMode::Normal;
        let mut word = String::new();
        for c in chars {
            match wordMode {
                WordMode::Normal => {
                    if c == '$' {
                        if word.len() > 0 {
                            // call on_field
                            // println!("on_field, {}", word.trim());
                            let wordTrim = word.trim();
                            f.on_field(wordTrim);
                            f.on_value(wordTrim, ValueType::Var);
                        }
                        wordMode = WordMode::Dollar;
                        word.clear();
                    } else if !((c >= 'A' && c <= 'B') || (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') || (c == '$') || c == '_') {
                        f.on_str(c);
                        f.on_value(&c.to_string(), ValueType::Char);
                        wordMode = WordMode::Str;
                    } else {
                        word.push(c);
                        f.on_value(&c.to_string(), ValueType::Char);
                    }
                },
                WordMode::Dollar => {
                    if c == '$' {
                        // call on_field
                        // println!("on_field, {}", word.trim());
                        let wordTrim = word.trim();
                        f.on_field(wordTrim);
                        f.on_value(wordTrim, ValueType::Var);
                        word.clear();
                        /*
                        wordMode = WordMode::Normal;
                        match lastWordMode {
                            WordMode::Str => {
                                // call on_field
                                // println!("on_field, {}", word.trim());
                                let wordTrim = word.trim();
                                f.on_field(wordTrim);
                                f.on_value(wordTrim, ValueType::Var);
                            },
                            _ => {
                                let wordTrim = word.trim();
                                f.on_field(wordTrim);
                                f.on_value(wordTrim, ValueType::Var);
                            }
                        }
                        word.clear();
                        */
                    } else if !((c >= 'A' && c <= 'B') || (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') || (c == '$') || c == '_') {
                        let wordTrim = word.trim();
                        f.on_field(wordTrim);
                        f.on_value(wordTrim, ValueType::Var);
                        f.on_str(c);
                        f.on_value(&c.to_string(), ValueType::Char);
                        wordMode = WordMode::Str;
                    } else {
                        word.push(c);
                    }
                },
                WordMode::Str => {
                    if c == '$' {
                        // lastWordMode = WordMode::Str;
                        wordMode = WordMode::Dollar;
                    } else {
                        f.on_str(c);
                        f.on_value(&c.to_string(), ValueType::Char);
                    }
                    word.clear();
                }
            }
        }
        // call on_field
        // println!("on_field, {}", word.trim());
        match wordMode {
            WordMode::Str => {
            },
            _ => {
                let wordTrim = word.trim();
                f.on_field(wordTrim);
                f.on_value(wordTrim, ValueType::Var);
            }
        }
    }
}

impl CRuleParser {
    pub fn new() -> CRuleParser {
        CRuleParser{}
    }
}

struct CDefaultRule<'a, ValueF> {
    valueF: &'a mut ValueF
}

impl<'a, ValueF> IRule for CDefaultRule<'a, ValueF>
    where ValueF: FnMut(&str, ValueType) {
    fn on_value(&mut self, t: &str, valueType: ValueType) {
        (self.valueF)(t, valueType);
    }
}

pub fn parse<ValueF>(content: &str, valueF: &mut ValueF)
    where ValueF: FnMut(&str, ValueType) {
    let parser = CRuleParser::new();
    parser.parse(content, &mut CDefaultRule{
        valueF: valueF
    });
}

#[cfg(test)]
mod test {
    use super::*;
    struct CRule {
    }
    impl IRule for CRule {
        fn on_field(&mut self, field: &str) {
            println!("field: {}", field);
        }
        fn on_str(&mut self, c: char) {
            println!("on_str: {}", c);
        }
    }
    #[test]
    #[ignore]
    fn ruleParserTest() {
        let parser = CRuleParser::new();
        let mut rule = CRule{};
        // parser.parse("--$name-.-$version-$platform.$d_r..$d_r..", &mut rule);
        // parser.parse("$name$version$platform$d_r", &mut rule);
        parser.parse("anywhere_net64$d_r", &mut rule);
    }
}
