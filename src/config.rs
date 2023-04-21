use directories::ProjectDirs;
use serde::Deserialize;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

// Structures needed to deserialize config file
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

    fn from_template_data(name: &str, data: &TemplateData) -> Self {
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

const DEFAULT_CONFIG_FILE_NAMES: [&str; 4] = [
    ".archierc.yaml",
    ".archierc.yml",
    ".archierc.YAML",
    ".archierc.YML",
];
const MAIN_CONFIG_FILE_NAME: &str = "archie.yaml";

pub fn get_file_by_priority(from_cli: &Option<PathBuf>) -> io::Result<File> {
    match from_cli {
        Some(path) => File::open(path),
        None => {
            // Check if working directory contains a config file
            for filename in DEFAULT_CONFIG_FILE_NAMES {
                let local_file = env::current_dir()?.join(filename);
                dbg!(&local_file);
                if local_file.exists() {
                    dbg!("Opening ", &local_file);
                    return File::open(local_file);
                }
            }

            // Try to open default config file in config directory
            match ProjectDirs::from("", "", "archie") {
                Some(proj_dirs) => File::open(proj_dirs.config_dir().join(MAIN_CONFIG_FILE_NAME)),
                None => Err(io::ErrorKind::NotFound.into()),
            }
        }
    }
}

pub struct Config {
    templates: Vec<Template>,
}

impl Config {
    pub fn from_file(file: &mut File) -> Result<Self, crate::error::Error> {
        let mut config_file = String::new();
        file.read_to_string(&mut config_file)?;

        let config_data: ConfigData = serde_yaml::from_str(&config_file)?;

        let mut config = Config {
            templates: Vec::new(),
        };

        for (name, data) in config_data.templates {
            config
                .templates
                .push(Template::from_template_data(&name, &data))
        }

        Ok(config)
    }

    pub fn template_by_name(&self, name: &str) -> Option<&Template> {
        self.templates.iter().find(|t| t.name == name)
    }
}
