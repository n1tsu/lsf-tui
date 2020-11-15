# Terminal UI program to help learn LSF.

## Words file

Provide a `yaml` file with the following structure :

```yaml
categories:
 - categorie: "Categorie name" 
   mots:
     - mot: "Word"
       description: "Translation or description"
       lien: "Some link providing more information"
      ...
  ...
```

---

## Modes

There is 3 modes :

1. *Dictionary* : TUI to navigate between words.
2. *Learning*   : TUI random set of word to challenge ourselves.
3. *Background* : Desktop notifications every **X** seconds to challenge ourselves.

[![asciicast](https://asciinema.org/a/372952.svg)](https://asciinema.org/a/372952)

---

## Usage

```sh
USAGE:
    lsf_tui [FLAGS] [OPTIONS]

FLAGS:
    -d, --description    Show word description in notifications
    -h, --help           Prints help information
    -V, --version        Prints version information

OPTIONS:
    -b, --background <SECONDS>    Background mode with notifications
    -c, --yaml <YAML>             YAML file containing words [default: LSF.yaml]
```

*Press `1` to enter in dictionary mode*  
Navigate using `h,j,k,l`  

*Press `2` to enter in learn mode*  
Next word using `n`  
Display help with `m`  

*Press `q` to quit TUI*  

---
