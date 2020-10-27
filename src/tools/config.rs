extern crate yaml_rust;
use yaml_rust::YamlLoader;

use std::fs;


pub fn read_conf_file(filename: String) -> yaml_rust::Yaml {
    let filecontent = fs::read_to_string(&filename)
        .expect(format!("Couldn't open config file {}", filename).as_str());

    let mut multidoc = YamlLoader::load_from_str(filecontent.as_str()).unwrap();

    // vec has ownership !
    multidoc.remove(0)
}

#[cfg(test)]
mod tests {
    use super::read_conf_file;

    #[test]
    fn test_read_file() {
        let yaml = read_conf_file(String::from("tests/ressources/tests.yaml"));

        assert_eq!(yaml["common"]["delay"].as_i64().unwrap(), 600);
    }
}
