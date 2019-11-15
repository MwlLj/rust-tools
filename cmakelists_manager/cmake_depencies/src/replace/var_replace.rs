use crate::parse;
use parse::var;

use std::collections::HashMap;

struct CVar<'a> {
    content: &'a mut String,
    vars: &'a HashMap<String, Vec<String>>
}

impl<'a> var::IVar for CVar<'a> {
    fn on_var(&mut self, var: &str) {
        match self.vars.get(var) {
            Some(values) => {
                for value in values.iter() {
                    self.content.push_str(value);
                }
            },
            None => {
                self.content.push_str("${");
                self.content.push_str(var);
                self.content.push('}');
            }
        }
    }

    fn on_ch(&mut self, c: char) {
        self.content.push(c);
    }
}

pub struct CVarReplace {
}

impl CVarReplace {
    pub fn replace(&self, c: &str, vars: &HashMap<String, Vec<String>>) -> String {
        let mut content = String::new();
        let mut v = CVar{
            content: &mut content,
            vars: vars
        };
        let parser = var::CVarParse::new();
        parser.parse(c, &mut v);
        content
    }
}

impl CVarReplace {
    pub fn new() -> Self {
        Self {
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[ignore]
    fn varReplaceTest() {
        let replacer = CVarReplace::new();
        let mut vars = HashMap::new();
        vars.insert(String::from("name"), vec![String::from("jake")]);
        vars.insert(String::from("age"), vec![String::from("20")]);
        let c = replacer.replace("hello ${name}, myage is ${age}", &vars);
        println!("{:?}", c);
    }
}

