pub mod parse;
pub struct Config {
    templates: Vec<Template>,
}

struct Template {
    name: String,
    structure: Vec<Node>,
}

enum Node {
    File,
    Folder(Vec<Node>),
}
