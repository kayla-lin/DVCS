use repository_storage::RepositoryStorage;
#[warn(unused_imports)]
use std::collections::HashMap;

fn main() {
    // create a new instance of the RepositoryStorage
    let mut repo_storage = RepositoryStorage::new();

    // create the current repository structure
    let mut current_structure = HashMap::new();
    current_structure.insert("README.md".to_string(), "Hello, world!".to_string());
    current_structure.insert("src/main.rs".to_string(), "fn main() {}".to_string());
    repo_storage.set_repo_snapshot(current_structure);

    // create the previous snapshot of the repository structure
    let mut prev_snapshot = HashMap::new();
    prev_snapshot.insert("README.md".to_string(), "Hello, world!".to_string());
    prev_snapshot.insert("src/main.rs".to_string(), "fn main() {}".to_string());

    // see the differences between the current repository structure and the previous snapshot
    let diff = repo_storage.see_diff(&prev_snapshot);

    // print the diff
    for (path, content) in diff {
        println!("{}: {}", path, content);
    }
}
