use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

pub(crate) struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    pub fn new() -> Result<Todo, std::io::Error> {
        let mut db = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("todo.db")?;

        let mut map_str = String::new();
        db.read_to_string(&mut map_str);

        let map: HashMap<String, bool> = map_str
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<_>>())
            .map(|v| (String::from(v[0]), bool::from_str(v[1]).unwrap()))
            .collect();

        Ok(Todo { map })
    }

    pub fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    // save takes self (and not a reference to self)
    // this stops user from changing things after saving
    pub fn save(self) -> Result<(), std::io::Error> {
        let mut map_str = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            map_str.push_str(&record);
        }
        std::fs::write("todo.db", map_str)
    }

    pub fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}
