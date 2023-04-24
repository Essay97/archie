# Archie
Archie is a command line utility that helps building folder structures based on a template.
You can provide templates based on a configuration file passed in one of the following ways, sorted by priority:
1. `--config <path/to/file>`option
2. `.archierc.yaml` file in current directory
3. `archie.yaml` in global configuration directory
    - `$XDG_CONFIG_HOME/archie` or `$HOME/.config/archie` on Linux
    - `%USERPROFILE%\AppData\Roaming\archie` on Windows
    - `$HOME/Library/Application Support/archie` on macOS
    
## Installation
## Installation
You can install Adventure Engine via [Homebrew](https://brew.sh/), you just have to add my tap:
```bash
brew tap essay97/harrysthings
brew update
brew install archie
```
    
## Configuration
Whatever method you choose to pass the configuration file, the format is always the same: a YAML file with a single root object `templates`. 

You define a new template by adding a key to the `templates` object. 
A template can contain 2 types of nodes:
- a **folder** node: name ends with a `/`
- a **file** node: no trailing `/`

Compose and nest these 2 types of nodes to shape your templates.

### Example
Let's say I want to define a template called "example":
```YAML
templates:
  example:            # creates the "example" template
    foo/:             # creates the "foo" folder
      hello.txt:      # creates the "hello.txt" 
      my_folder/:     # creates the empty folder "my_folder"
    bar/:             # creates the "bar" folder at the same level of "foo"
    file:             # creates the "file" file at the same level of "foo" and "bar"
```

Notice that Archie takes into account only the keys of the configuration file, so even files have to be objects, just without body (i.e. with a `null` body). 

As a rule of thumb, every YAML element that you want to turn into a file or a folder, has to be followed by a colon.
