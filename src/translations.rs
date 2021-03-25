use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::collections::HashMap;



#[derive(Debug, Deserialize, Clone)]
pub struct TranslationTable {
    pub culture: String,
    pub translations: HashMap<String, String>
}

#[derive(Debug, Default)]
pub struct Translations {
    tables: Vec<TranslationTable>
}



impl Translations {
    pub fn new(path: &str) -> Result<Translations, Error> {
        if !Path::new(path).exists() {
            return Err(Error::new(ErrorKind::NotFound, format!("Unable to find i18n path {}", path)));
        }

        let mut tables = Vec::new();
        let paths = std::fs::read_dir(path)?;

        for file in paths {
            let p = file?.path();
            let filename = p.to_str().unwrap();
            let table = Translations::load_file(filename)?;
            tables.push(table);
        }

        Ok(Translations {
            tables
        })
    }

    pub fn add_translation_table(&mut self, table: TranslationTable) {
        self.tables.push(table);
    }

    pub fn translate(&self, culture: &str, key: &str) -> &str {
        for table in &self.tables {
            if table.culture != culture {
                continue;
            }

            if table.translations.contains_key(key) {
                return &table.translations[&key.to_string()];
            }
            panic!("Translation key {} for culture {} is missing", key, culture);
        }
        self.translate("en", key)
    }


    fn load_file(path: &str) -> Result<TranslationTable, Error> {
        let data = Translations::read_config(path)?;
             
        let table: TranslationTable = serde_json::from_str(&data).expect("Unable to parse translation table");
        Ok(table)
    }

    fn read_config(path: &str) -> Result<String, Error> {
        if !Path::new(path).exists() {
            return Err(Error::new(ErrorKind::NotFound, format!("Unable to find config file {}", path)));
        }
       
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;    
        println!("Using {} configuration", path);            
        Ok(contents)     
    }
}