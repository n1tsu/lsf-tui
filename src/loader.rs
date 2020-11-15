use std::io::prelude::*;
use std::fs::File;
use yaml_rust::YamlLoader;

/*
 * Categorie and Word structure are clonable because
 * we want to clone the vector of categories loaded
 * by `load_file` to generate a new list containing
 * all the words
 */

#[derive(Clone)]
pub struct Categorie {
    pub name : String,
    pub words : Vec<Word>,
}

#[derive(Clone)]
pub struct Word {
    pub name : String,
    pub description : String,
    pub link : String,
}

// Load yaml entries into a vector of categories
pub fn load_file(file: &str) -> Vec<Categorie> {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();

    let mut res = Vec::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0];

    for i in doc["categories"].as_vec().unwrap() {
        let mut words = Vec::new();
        for y in i["mots"].as_vec().unwrap() {
            let word = Word {
                name : String::from(y["mot"].as_str().unwrap()),
                description : String::from(y["description"].as_str().unwrap()),
                link : String::from(y["lien"].as_str().unwrap()),
            };
            words.push(word);
        }
        let categorie = Categorie {
            name : String::from(i["categorie"].as_str().unwrap()),
            words,
        };
        res.push(categorie);
    }
    res
}
