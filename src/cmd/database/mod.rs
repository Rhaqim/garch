// Mapping of commands to run to add different types of databases to the Go project.

use std::collections::HashMap;

fn database_map() -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();

    map.insert(
        "PostgreSQL".to_string(),
        vec![
            "go get github.com/lib/pq".to_string(),
            "go mod tidy".to_string(),
        ],
    );
    map.insert(
        "MySQL".to_string(),
        vec![
            "go get github.com/go-sql-driver/mysql".to_string(),
            "go mod tidy".to_string(),
        ],
    );
    map.insert(
        "SQLite".to_string(), vec!["go mod tidy".to_string()]);
    map.insert(
        "MongoDB".to_string(),
        vec![
            "go get go.mongodb.org/mongo-driver/mongo".to_string(),
            "go mod tidy".to_string(),
        ],
    );

    map
}

pub fn run_database_commands(db_type: &str) -> Vec<String> {
    let map = database_map();
    let commands = map.get(db_type).unwrap();
    commands.to_vec()
}