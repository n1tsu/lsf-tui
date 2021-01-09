# TUI program to learn LSF.

[![asciicast](https://asciinema.org/a/hbrZt3imcFYkFgzNX1EJ7n9tj.svg)](https://asciinema.org/a/hbrZt3imcFYkFgzNX1EJ7n9tj)

## Modes

1. *Dictionary* : TUI to navigate between words.
2. *Learning*   : TUI trial mode on words from a category.
3. *Background* : Desktop notifications every **X** seconds to challenge ourselves.

---

## Words file

Words are loaded from `yaml` file with the following structure :

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
    -q, --video <video>           Search video for a word in Elix dictionary
    -c, --yaml <YAML>             YAML file containing words [default: LSF.yaml]
```

---

## Keys

* `1` : Enter dictionary mode
  * `h` : Focus left
  * `j` : Focus down
  * `k` : Focus up
  * `l` : Focus right
  * `v` : Try to search for a video of the word

* `2` : Enter trial mode
  * `n` : Next word
  * `h` : Display help
  
* `q` : Quit TUI

---
