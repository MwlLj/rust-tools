use serde_derive::{Serialize, Deserialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct CPackage {
    pub name: String,
    pub authors: Option<String>,
    pub edition: Option<String>,
    pub platform: Option<String>,
    pub target: Option<String>,
    pub debug: Option<String>,
    pub release: Option<String>,
    pub rule: Option<String>,
    pub libpathRule: Option<String>,
    pub includeRule: Option<String>,
    pub map: Option<HashMap<String, String>>
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct CAttributes {
    pub platform: Option<String>,
    pub target: Option<String>,
    pub debug: Option<String>,
    pub release: Option<String>,
    pub rule: Option<String>,
    pub libpathRule: Option<String>,
    pub includeRule: Option<String>,
    pub map: Option<HashMap<String, String>>
}

/*
impl CAttributes {
    pub fn new() -> CAttributes {
        CAttributes{
            platform: Some("${FILE_PREFIX}".to_string()),
            debug: Some("_d".to_string()),
            release: Some("".to_string()),
            rule: Some("$name$version$platform$d_r".to_string())
        }
    }
}
*/

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct CLib {
    pub version: String,
    pub subs: Option<String>,
    pub root: Option<String>,
    pub no: u32
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct CVersion {
    pub attributes: Option<CAttributes>,
    pub dependencies: Option<HashMap<String, CLib>>
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct CLibConfig {
    pub package: CPackage,
    pub versions: HashMap<String, CVersion>,
}

pub fn parse<'a>(content: &'a [u8]) -> Result<CLibConfig, &'a str> {
    let config = match toml::de::from_slice(content) {
        Ok(c) => c,
        Err(err) => {
            println!("toml parse error, err: {}", err);
            return Err("toml parse error");
        }
    };
    Ok(config)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    #[test]
    #[ignore]
    fn parseTest() {
        let content = fs::read("./doc/lib_config/test.library.config.toml").unwrap();
        let config = parse(&content).unwrap();
        println!("{:?}", config);
    }
}
