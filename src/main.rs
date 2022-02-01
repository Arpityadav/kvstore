use std::collections::HashMap;


fn main() {
    // println!("Hello, world!");
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("key was not there");
    let value = arguments.next().unwrap();

    println!("The key is '{}' and the value is '{}'", key, value);

    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kb.db", contents).unwrap();

    let database = Database::new().expect("Creating DB failed.");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();

        let contents = std::fs::read_to_string("kb.db")?;

        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No Key");
            let value = chunks.next().expect("No Value");

            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database {
            map: map
        })
    }
    
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

}