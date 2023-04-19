// Structures needed to deserialize config file

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConfigData {
    pub templates: HashMap<String, TemplateData>,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct TemplateData {
    structure: HashMap<String, Option<TemplateData>>,
}

// Structures needed to represent configuration

#[derive(Debug)]
struct Node {
    name: String,
    kind: NodeKind,
}

impl Node {
    fn create_file(name: &str) -> Self {
        Self {
            name: name.to_string(),
            kind: NodeKind::File,
        }
    }
}

#[derive(Debug)]
enum NodeKind {
    Folder(Option<Vec<Node>>),
    File,
}

#[derive(Debug)]
pub struct Template {
    name: String,
    structure: Vec<Node>,
}

impl Template {
    fn new_with_name(name: &str) -> Self {
        Self {
            name: String::from(name),
            structure: Vec::new(),
        }
    }

    pub fn from_template_data(name: &str, data: &TemplateData) -> Self {
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

                template.structure.push(folder_node);
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
                        // it's a folder
                        let mut node_name = String::from(node_name);
                        node_name.pop(); // remove trailing slash

                        let folder_node = Node {
                            name: node_name.clone(),
                            kind: NodeKind::Folder(Self::folder_from_template_data(content)),
                        };

                        nodes.push(folder_node);
                    } else {
                        nodes.push(Node::create_file(node_name))
                    }
                }
            }
        };

        Some(nodes)
    }
}
