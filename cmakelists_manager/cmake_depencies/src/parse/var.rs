pub trait IVar {
    fn on_var(&mut self, var: &str) {}
    fn on_ch(&mut self, c: char) {}
}

pub struct CVarParse {
}

enum Mode {
    Normal,
    Dollar,
    Var
}

impl CVarParse {
    pub fn parse<F: IVar>(&self, content: &str, f: &mut F) {
        let chars = content.chars();
        let mut mode = Mode::Normal;
        let mut var = String::new();
        for c in chars {
            match mode {
                Mode::Normal => {
                    if c == '@' {
                        mode = Mode::Dollar;
                    } else {
                        f.on_ch(c);
                    }
                },
                Mode::Dollar => {
                    if c == '{' {
                        mode = Mode::Var;
                        var.clear();
                    } else {
                        mode = Mode::Normal;
                    }
                },
                Mode::Var => {
                    if c == '}' {
                        f.on_var(&var);
                        mode = Mode::Normal;
                        var.clear();
                    } else {
                        var.push(c);
                    }
                }
            }
        }
    }
}

impl CVarParse {
    pub fn new() -> Self {
        Self{
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    struct CVar {
    }
    impl IVar for CVar {
        fn on_var(&mut self, var: &str) {
            println!("var: {}", var);
        }
        fn on_ch(&mut self, c: char) {
            println!("char: {}", c);
        }
    }
    #[test]
    #[ignore]
    fn varParseTest() {
        let parser = CVarParse::new();
        parser.parse(r#"hello@{name}good morning@{age}"#, &mut CVar{});
    }
}

