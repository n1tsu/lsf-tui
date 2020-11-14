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
# Run TUI application with mode 1 and 2.
cargo run -yaml LSF.yaml
```

*Press `1` to enter in dictionary mode*  
Navigate using `h,j,k,l`  

*Press `2` to enter in learn mode*  
Next word using `n`  
Display help with `m`  

*Press `q` to quit TUI*  

``` sh
# Run background mode with notifications every 10 seconds.
cargo run -yaml LSF.yaml -background 10
```

---
