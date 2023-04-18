use std::{collections::HashMap, fs};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ConfigData {
    templates: HashMap<String, TemplateData>,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
struct TemplateData {
    structure: HashMap<String, Option<TemplateData>>,
}

struct Node {
    name: String,
    kind: NodeKind,
}

impl Node {
    fn file(name: &str) -> Self {
        let mut name = String::from(name);

        if name.ends_with('/') {
            name.pop();
        }

        Self {
            name,
            kind: NodeKind::File,
        }
    }
}

enum NodeKind {
    Folder(Option<Vec<Node>>),
    File,
}

struct Template {
    name: String,
    structure: Vec<Node>,
}

impl Template {
    fn new_with_name(name: String) -> Self {
        Self {
            name: name,
            structure: Vec::new(),
        }
    }

    fn from_template_data(name: String, data: &TemplateData) -> Self {
        let mut template = Self::new_with_name(name);

        for (node_name, content) in &data.structure {
            if node_name.ends_with('/') {
                // it's a folder
                let mut node_name = String::from(node_name);
                node_name.pop(); // remove trailing slash

                let folder_node = Node {
                    name: node_name.clone(),
                    kind: NodeKind::Folder(Self::folder_from_template_data(content)),
                };
            } else {
                // it's a file
                if content.is_some() {
                    panic!("Files cannot have sub-nodes");
                }

                template.structure.push(Node {
                    name: String::from(node_name),
                    kind: NodeKind::File,
                })
            }
        }

        template
    }

    fn folder_from_template_data(template_data: &Option<TemplateData>) -> Option<Vec<Node>> {
        let mut nodes = Vec::new();

        match template_data {
            None => return None,
            Some(data) => {
                for (node_name, content) in &data.structure {
                    if node_name.ends_with('/') {
                    } else {
                        nodes.push(Node::file(node_name))
                    }
                }
            }
        };

        Some(nodes)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_file = fs::read_to_string("examples/config/.archierc.yaml")?;
    let config: ConfigData = serde_yaml::from_str(&config_file)?;

    match config.templates.get("template1") {
        None => panic!("template not found"),
        Some(template) => {}
    }

    println!("{config:#?}");

    Ok(())
}
