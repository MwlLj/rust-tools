use super::libconfig;

use std::collections::HashMap;

pub fn default(name: &str, config: &str) -> libconfig::CLibConfig {
    let mut package = libconfig::CPackage::default();
    package.name = name.to_string();
    // package.includeEnable = Some(String::from("false"));
    package.includeRule = Some(String::from(config));
    package.libpathEnable = Some(String::from("false"));
    package.rule = Some(String::from(r#"$name$name_platform$d_r"#));
    let mut map = HashMap::new();
    map.insert(String::from("name_platform"), String::from(r#"
        `judge:"
        if var:'target' == str:'win64' {
            64
        } elseif var:'target' == str:'win32' {
        } elseif var:'target' == str:'centos64' {
            64
        } elseif var:'target' == str:'ubuntu64' {
            64
        } else {
        }
        "`
        "#));
    package.map = Some(map);
    let mut versions: HashMap<String, libconfig::CVersion> = HashMap::new();
    let mut version = libconfig::CVersion::default();
    versions.insert("".to_string(), version);
    libconfig::CLibConfig{
        package: package,
        versions: versions
    }
}
