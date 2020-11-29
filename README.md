# Terminal UI program to help learn LSF.

[![asciicast](https://asciinema.org/a/hbrZt3imcFYkFgzNX1EJ7n9tj.svg)](https://asciinema.org/a/hbrZt3imcFYkFgzNX1EJ7n9tj)

## Modes

There is 3 modes :

1. *Dictionary* : TUI to navigate between words.
2. *Learning*   : TUI trial mode on words from a category.
3. *Background* : Desktop notifications every **X** seconds to challenge ourselves.

---

## Words file

Words are load from `yaml` file with the following structure :

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
Display help with `h`  

*Press `q` to quit TUI*  

---
