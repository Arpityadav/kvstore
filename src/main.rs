use std::collections::HashMap;


fn main() {
    // println!("Hello, world!");
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("key was not there");
    let value = arguments.next().unwrap();

    println!("The key is '{}' and the value is '{}'", key, value);

    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kb.db", contents).unwrap();

    let database = Database::new();
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // let contents = match std::fs::read_to_string("kb.db") {
        //     Ok(contents) => contents,
        //     Err(error) => {
        //         return Err(error);
        //     }
        // };
        
        let contents = std::fs::read_to_string("kb.db")?;

        for line in contents.lines() {
            let pair = line.split_once('\t').expect("Corrupted file");
        }
        Database {
            map: HashMap::new();
        }
    } 
}