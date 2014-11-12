use toml;
use util;

pub fn allowed_extensions(dest: &Path) -> Vec<&'static str> {
    let toml = util::parse_file(&dest.join("config.toml"));
    // TODO: handle errors, don't use unwrap..
    let config = toml::Parser::new(toml.as_slice()).parse().unwrap();
    let value = config.get(&"cobalt".to_string()).unwrap();

    let mut extensions: Vec<&'static str> = vec![];
    for val in value.lookup("extensions").unwrap().as_slice().unwrap().iter() {
        println!("{}", val);
        extensions.push(val.unwrap());
    }

    vec!["tpl"]
}
