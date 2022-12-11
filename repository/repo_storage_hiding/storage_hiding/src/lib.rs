#[warn(unused_imports)]
use std::collections::HashMap;

mod repository_storage {

    use std::collections::HashMap;

    pub struct RepositoryStorage {
        // map to store the repository structure (keys are file paths and values are file contents)
        structure: HashMap<String, String>,
    }

    impl RepositoryStorage {
        // creates a new instance of the RepositoryStorage
        pub fn new() -> RepositoryStorage {
            RepositoryStorage {
                structure: HashMap::new(),
            }
        }

        // sees the differences between the current repository structure and a previous snapshot
        pub fn see_diff(&self, snapshot: &HashMap<String, String>) -> HashMap<String, String> {
            let mut diff = HashMap::new();

            // iterate over the current repository structure
            for (path, content) in &self.structure {
                // check if the file was present in the previous snapshot
                let prev_content = snapshot.get(path);
                if let Some(prev_content) = prev_content {
                    // if the file was present, check if its content has changed
                    if prev_content != content {
                        // if the content has changed, add it to the diff
                        diff.insert(path.clone(), content.clone());
                    }
                } else {
                    // if the file was not present in the previous snapshot, add it to the diff
                    diff.insert(path.clone(), content.clone());
                }
            }

            diff
        }

        // gets the current repository structure
        pub fn get_repo_structure(&self) -> HashMap<String, String> {
            self.structure.clone()
        }

        // sets the current repository structure to the given snapshot
        pub fn set_repo_snapshot(&mut self, snapshot: HashMap<String, String>) {
            self.structure = snapshot;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository_storage::RepositoryStorage;

    #[test]
    fn test_see_diff() {
        // create a new instance of the RepositoryStorage
        let mut repo_storage = RepositoryStorage::new();

        // create the current repository structure
        let mut current_structure = HashMap::new();
        current_structure.insert("README.md".to_string(), "Hello, world!".to_string());
        current_structure.insert("src/main.rs".to_string(), "fn main() {}".to_string());
        current_structure.insert("src/lib.rs".to_string(), "pub fn hello() {}".to_string());
        repo_storage.set_repo_snapshot(current_structure);

        // create the previous snapshot of the repository structure
        let mut prev_snapshot = HashMap::new();
        prev_snapshot.insert("README.md".to_string(), "Hello, world!".to_string());
        prev_snapshot.insert("src/main.rs".to_string(), "fn main() {}".to_string());

        // see the differences between the current repository structure and the previous snapshot
        let diff = repo_storage.see_diff(&prev_snapshot);

        // check if the diff is correct
        assert_eq!(
            diff.get("src/lib.rs"),
            Some(&"pub fn hello() {}".to_string())
        );
    }

    #[test]
    fn test_get_repo_structure() {
        // create a new instance of the RepositoryStorage
        let mut repo_storage = RepositoryStorage::new();

        // create the current repository structure
        let mut current_structure = HashMap::new();
        current_structure.insert("README.md".to_string(), "Hello, world!".to_string());
        current_structure.insert("src/main.rs".to_string(), "fn main() {}".to_string());
        repo_storage.set_repo_snapshot(current_structure);

        // get the current repository structure
        let structure = repo_storage.get_repo_structure();

        // check if the structure is correct
        assert_eq!(
            structure.get("src/main.rs"),
            Some(&"fn main() {}".to_string())
        );
    }

    #[test]
    fn test_set_repo_snapshot() {
        // create a new instance of the RepositoryStorage
        let mut repo_storage = RepositoryStorage::new();

        // create the previous snapshot of the repository structure
        let mut prev_snapshot = HashMap::new();
        prev_snapshot.insert("README.md".to_string(), "Hello, world!".to_string());
        prev_snapshot.insert("src/main.rs".to_string(), "fn main() {}".to_string());

        // set the repository structure to the previous snapshot
        repo_storage.set_repo_snapshot(prev_snapshot);

        // get the current repository structure
        let structure = repo_storage.get_repo_structure();

        // check if the structure is correct
        assert_eq!(
            structure.get("src/main.rs"),
            Some(&"fn main() {}".to_string())
        );
    }

    #[test]
    fn test_get_repo_structure_empty_repo() {
        // create a new instance of the RepositoryStorage
        let repo_storage = RepositoryStorage::new();

        // get the current repository structure
        let structure = repo_storage.get_repo_structure();

        // check if the structure is empty
        assert!(structure.is_empty());
    }
}
