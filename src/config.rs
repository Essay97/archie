use anyhow::{anyhow, Context};
use directories::ProjectDirs;
use serde::Deserialize;
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

// Structures needed to deserialize config file
#[derive(Debug, Deserialize)]
pub struct ConfigData {
    templates: HashMap<String, TemplateData>,
    favorite: Option<String>,
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
    fn new_file(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            kind: NodeKind::File,
        }
    }

    /// WARNING: this function creates the template in the current working directory. It's responsibility of the caller
    ///  to make sure that the current working directory is set correctly BEFORE this funciton call.
    fn build(&self) -> anyhow::Result<()> {
        match &self.kind {
            // if it's a folder, create it recursively
            // Create this node
            NodeKind::Folder(children) => {
                fs::create_dir(&self.name)
                    .with_context(|| format!("could not create folder {}", &self.name))
                    .and_then(|_| {
                        // set cwd to newly created folder
                        env::set_current_dir(&self.name)
                            .with_context(|| format!("could not move to {} folder", &self.name))?;
                        // iterate over children nodes
                        if let Some(nodes) = children {
                            for node in nodes {
                                node.build()?;
                            }
                        }
                        // back to upper folder
                        env::set_current_dir("..").with_context(|| {
                            format!("could not move to parent folder of {}", &self.name)
                        })?;
                        Ok(())
                    })
            }
            NodeKind::File => File::create(&self.name)
                .map(|_| ())
                .with_context(|| format!("could not create file {}", &self.name)),
        }
    }

    fn print(&self) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();

        // If node is a folder and has children => node is a non-terminal folder, continue printing
        if let NodeKind::Folder(Some(nodes)) = &self.kind {
            let mut nodes = nodes.iter().peekable();

            while let Some(node) = nodes.next() {
                let is_last = nodes.peek().is_none();
                let glyph = if is_last { "└── " } else { "├── " };

                lines.push(format!("{glyph}{}", node.name));
                let sub_lines = node.print();

                for line in sub_lines {
                    let pre_glyph = if is_last { "    " } else { "│   " };
                    lines.push(format!("{pre_glyph}{line}"));
                }
            }
        }

        lines
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
                        nodes.push(Node::new_file(node_name))
                    }
                }
            }
        };

        Some(nodes)
    }

    /// WARNING: this function creates the template in the current working directory. It's responsibility of the caller
    ///  to make sure that the current working directory is set correctly BEFORE this funciton call.
    pub fn build(&self) -> anyhow::Result<()> {
        for node in &self.structure {
            node.build()?;
        }

        Ok(())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn print(&self) {
        let nodes = &mut self.structure.iter().peekable();

        while let Some(node) = nodes.next() {
            let is_last = nodes.peek().is_none();
            let glyph = if is_last { "└── " } else { "├── " };

            // Print current node
            println!("{glyph}{}", node.name);

            let lines = node.print();
            let lines = lines.iter().peekable();

            for line in lines {
                let pre_glyph = if is_last { "    " } else { "│   " };
                println!("{pre_glyph}{line}");
            }
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

pub fn get_file_by_priority(from_cli: &Option<PathBuf>) -> anyhow::Result<File> {
    match from_cli {
        Some(path) => {
            File::open(path).with_context(|| format!("could not access file {}", path.display()))
        }
        None => {
            // Check if working directory contains a config file
            for filename in DEFAULT_CONFIG_FILE_NAMES {
                let local_file = &env::current_dir()
                    .with_context(|| "could not get current directory")?
                    .join(filename);
                if local_file.exists() {
                    return File::open(local_file).with_context(|| {
                        format!("could not access file {}", local_file.display())
                    });
                }
            }

            // Try to open default config file in config directory
            match ProjectDirs::from("", "", "archie") {
                Some(proj_dirs) => {
                    let config_dir_path = proj_dirs.config_dir();
                    let path = &config_dir_path.join(MAIN_CONFIG_FILE_NAME);

                    match File::open(path) {
                        Ok(file) => Ok(file),
                        Err(e) => {
                            if let io::ErrorKind::NotFound = e.kind() {
                                create_config_dir(config_dir_path)
                            } else {
                                Err(e).with_context(|| {
                                    format!("could not access file {}", path.display())
                                })
                            }
                        }
                    }
                }
                None => Err(anyhow!("could not access home directory")),
            }
        }
    }
}

// "File" result type just satisfies the compiler but it never really gets reached. The "success" branch exits with exit code 0
fn create_config_dir(path: &Path) -> anyhow::Result<File> {
    print!(
        "Seems like config directory {} is missing. Would you like to create it? [Y/n] ",
        path.display()
    );
    io::stdout()
        .flush()
        .with_context(|| "a problem occurred while reading user input")?;
    let mut response = String::new();
    let stdin = std::io::stdin();

    stdin
        .read_line(&mut response)
        .with_context(|| "a problem occurred while reading user input")?;

    response = response.trim().to_owned();

    if response == "y" || response == "Y" {
        fs::create_dir_all(path)
            .with_context(|| format!("could not create folder {}", path.display()))?;
        println!(
            "Created {} folder. Create a {MAIN_CONFIG_FILE_NAME} there",
            path.display()
        );
        std::process::exit(0);
    }

    Err(anyhow!("could not create config folder"))
}

#[derive(Debug)]
pub struct Config {
    templates: Vec<Template>,
    favorite: Option<String>,
}

impl Config {
    pub fn from_file(file: &mut File) -> anyhow::Result<Self> {
        let mut config_file = String::new();
        file.read_to_string(&mut config_file)
            .with_context(|| "could not read config file")?;

        let config_data: ConfigData = serde_yaml::from_str(&config_file)
            .with_context(|| "could not deserialize config file")?;

        let mut config = Config {
            templates: Vec::new(),
            favorite: config_data.favorite,
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

    pub fn favorite(&self) -> &Option<String> {
        &self.favorite
    }
}
