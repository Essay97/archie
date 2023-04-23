use directories::ProjectDirs;
use serde::Deserialize;
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
};

use crate::error;

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
    /// WARNING: this function creates the template in the current working directory. It's responsibility of the caller
    ///  to make sure that the current working directory is set correctly BEFORE this funciton call.
    fn build(&self) -> error::Result<()> {
        match &self.kind {
            // if it's a folder, create it recursively
            // Create this node
            NodeKind::Folder(children) => fs::create_dir(&self.name)
                .map_err(|_| error::Error::OnCreateFolder(PathBuf::from(&self.name)))
                .and_then(|_| {
                    // set cwd to newly created folder
                    env::set_current_dir(&self.name)
                        .map_err(|_| error::Error::OnChangeFolder(PathBuf::from(&self.name)))?;
                    // iterate over children nodes
                    if let Some(nodes) = children {
                        for node in nodes {
                            node.build()?;
                        }
                    }
                    // back to upper folder
                    env::set_current_dir("..")
                        .map_err(|_| error::Error::OnChangeFolder(PathBuf::from("..")))?;
                    Ok(())
                }),
            NodeKind::File => File::create(&self.name)
                .map(|_| ())
                .map_err(|_| error::Error::OnCreateFile(PathBuf::from(&self.name))),
        }
    }

    fn print(&self, indentation: &mut Vec<bool>, level: &mut usize, last: bool) {
        match &self.kind {
            NodeKind::Folder(children) => {
                self.render_line(indentation, last);
                if let Some(nodes) = children {
                    *level += 1;
                    if indentation.len() > *level {
                        indentation[*level] = true;
                    } else {
                        indentation.push(true)
                    }

                    let mut nodes = nodes.iter().peekable();
                    while let Some(node) = nodes.next() {
                        node.print(indentation, level, nodes.peek().is_none());
                    }

                    indentation[*level] = false;
                    *level -= 1;
                }

                if last {
                    indentation[*level] = false;
                }
            }
            NodeKind::File => {
                self.render_line(indentation, last);
            }
        }
    }

    fn render_line(&self, indentation: &[bool], last: bool) {
        let mut line = String::new();
        let mut first = true;

        //println!("{:?}", indentation);

        for i in indentation.iter().rev() {
            if *i {
                if first {
                    let glyph = if last { "└──" } else { "├──" };
                    line = glyph.to_owned() + &line;
                    first = false;
                } else {
                    line = "│   ".to_owned() + &line;
                }
            }
        }

        println!("{line} {}", self.name);
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

    /// WARNING: this function creates the template in the current working directory. It's responsibility of the caller
    ///  to make sure that the current working directory is set correctly BEFORE this funciton call.
    pub fn build(&self) -> error::Result<()> {
        for node in &self.structure {
            node.build()?;
        }

        Ok(())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn print(&self) {
        let mut indentation: Vec<bool> = vec![true];
        let mut level = 0usize;
        let mut structure = self.structure.iter().peekable();
        while let Some(node) = structure.next() {
            node.print(&mut indentation, &mut level, structure.peek().is_none());
            /* if structure.peek().is_none() {
                indentation[level] = false;
            } */
        }
    }
}

const DEFAULT_CONFIG_FILE_NAMES: [&str; 4] = [
    ".archierc.yaml",
    ".archierc.yml",
    ".archierc.YAML",
    ".archierc.YML",
];
const MAIN_CONFIG_FILE_NAME: &str = "archie.yaml";

pub fn get_file_by_priority(from_cli: &Option<PathBuf>) -> error::Result<File> {
    match from_cli {
        Some(path) => {
            File::open(path).map_err(|_| error::Error::FileNotAccessible(path.to_owned()))
        }
        None => {
            // Check if working directory contains a config file
            for filename in DEFAULT_CONFIG_FILE_NAMES {
                let local_file = &env::current_dir()
                    .map_err(|_| error::Error::CurrentDirectoryUnavailable)?
                    .join(filename);
                if local_file.exists() {
                    return File::open(local_file)
                        .map_err(|_| error::Error::FileNotAccessible(local_file.to_owned()));
                }
            }

            // Try to open default config file in config directory
            match ProjectDirs::from("", "", "archie") {
                Some(proj_dirs) => {
                    let config_dir_path = proj_dirs.config_dir();

                    if !crate::path_exists(config_dir_path)? {
                        print!("Seems like config directory {} is missing. Would you like to create it? [Y/n] ", config_dir_path.display());
                        io::stdout().flush().map_err(|_| error::Error::OnInput)?;
                        let mut response = String::new();
                        let stdin = std::io::stdin();

                        stdin
                            .read_line(&mut response)
                            .map_err(|_| error::Error::OnInput)?;

                        response = response.trim().to_owned();

                        if response == "y" || response == "Y" {
                            fs::create_dir_all(config_dir_path).map_err(|_| {
                                error::Error::OnCreateFolder(config_dir_path.to_owned())
                            })?;
                            println!(
                                "Created {} folder. Create a {MAIN_CONFIG_FILE_NAME} there",
                                config_dir_path.display()
                            );
                        }
                        Err(error::Error::Dummy)
                    } else {
                        let path = &config_dir_path.join(MAIN_CONFIG_FILE_NAME);
                        File::open(path).map_err(|e| {
                            dbg!(e);
                            error::Error::FileNotAccessible(path.to_owned())
                        })
                    }
                }
                None => Err(error::Error::NoHomeFolder),
            }
        }
    }
}

pub struct Config {
    templates: Vec<Template>,
}

impl Config {
    pub fn from_file(file: &mut File) -> error::Result<Self> {
        let mut config_file = String::new();
        file.read_to_string(&mut config_file)
            .map_err(|_| error::Error::WrongFileEncoding)?;

        let config_data: ConfigData =
            serde_yaml::from_str(&config_file).map_err(|_| error::Error::OnDeserialize)?;

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

    pub fn templates(&self) -> &Vec<Template> {
        &self.templates
    }
}
