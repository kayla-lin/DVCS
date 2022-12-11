use std::collections::HashMap;

struct RepositoryController {
    // vector to store the commit history for the repository (could be a hashSet as well)
    commit_history: Vec<String>,
    // map to store the head commits for each branch in the repository
    branch_heads: HashMap<String, String>,

    // map to store the commit history for each file in the repository
    file_history: HashMap<String, Vec<String>>,
}

impl RepositoryController {
    pub fn new(dvcs_hidden: &str, working_directory: &str) -> Result<Staging, String> {
        // * Check if index file exists, otherwise create it
        match Path::new(&dvcs_hidden).try_exists() {
            // * Repository exist has error
            Err(_) => return Err("Could not read DVCS hidden file".to_string()),
            Ok(false) => return Err("Could not find working directory".to_string()),
            Ok(true) => {
                // * Creating index file if it doesn't exist already
                let index_path = &(dvcs_hidden.to_owned() + "/index.json");
                match Path::new(&index_path).try_exists() {
                    Err(_) => return Err("Could not read DVCS hidden file".to_string()),
                    // * Create new index file if not created already
                    Ok(false) => {
                        let update = Self::recreate_index_file(dvcs_hidden);
                        if update.is_err() {
                            return Err(update.unwrap_err());
                        }
                    }
                    // * Index file exists, ignore
                    _ => {}
                }

                // * Read index file and load the staging index structure
                // * Returning successful staging structure if read succussfully, otherwise returns new staging
                return Self::read_from_staging_file(dvcs_hidden.to_string())
                    .and_then(|retrieved_file| {
                        return Ok(Staging {
                            dvcs_hidden: dvcs_hidden.to_string(),
                            working_directory: working_directory.to_string(),
                            index: retrieved_file,
                        });
                    })
                    .or_else(|err| Err(err));
            }
        };
    }

    // commits the current state of the repository to storage
    fn commit(&mut self, branch: &str, commit_message: String, files: Vec<(String, String)>) {
        // updates the head commit for the specified branch
        self.branch_heads
            .insert(branch.to_string(), commit_message.clone());

        // adds the commit message to the commit history
        self.commit_history.push(commit_message.clone());

        // updates the file history with the new commit
        for (file_path, file_content) in files {
            self.file_history
                .entry(file_path)
                .or_insert_with(|| vec![])
                .push(format!("{}:{}", commit_message, file_content));
        }
    }

    // return the commit log for the repository
    fn log(&self) -> Vec<String> {
        self.commit_history.clone()
    }

    // return the head commits for all branches in the repository
    fn heads(&self) -> Vec<String> {
        self.branch_heads.values().map(|s| s.clone()).collect()
    }

    // retrieves the full commit history for the repository
    fn retrieve_all_history(&self) -> Vec<String> {
        self.commit_history.clone()
    }

    // retrieves the commit history for a specific file in the repository
    fn retrieve_commit_history(&self, path: &str) -> Vec<String> {
        self.file_history
            .get(path)
            .map(|s| s.clone())
            .unwrap_or_else(|| vec![])
    }

    // concatenates two or more commits together
    fn concatenate(&mut self, branch: &str, commit_hashes: Vec<String>) {
        let mut new_commit_message = String::new();
        for commit_hash in commit_hashes {
            let commit_message = self
                .commit_history
                .iter()
                .find(|m| **m == commit_hash)
                .map(|s| s.clone())
                .unwrap_or_else(|| String::new());
            new_commit_message.push_str(&commit_message);
            new_commit_message.push('\n');
        }

        // create a new commit with the concatenated messages
        self.commit(branch, new_commit_message, vec![])
    }
    // checks out a specific commit from the repository
    fn checkout(&mut self, branch: &str, commit_hash: &str) {
        let commit_message = self
            .commit_history
            .iter()
            .find(|m| **m == commit_hash)
            .map(|s| s.clone())
            .unwrap_or_else(|| String::new());

        // check out the commit by creating a new commit with the same message
        self.commit(branch, commit_message, vec![]);
    }
    // PS. This function takes the name of the branch to checkout as an argument
    // and retrieves the head commit for that branch from the branch_heads map.
    // It then creates a new commit with the same message as the head commit,
    // checking out that branch in the repository. Please let me know if there is
    // anything I should've done differently in this function.
}

// tests for the repository controller
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit() {
        let mut repo = RepositoryController {
            commit_history: Vec::new(),
            branch_heads: HashMap::new(),
            file_history: HashMap::new(),
        };
        repo.commit("master", "Initial commit".to_string(), vec![]);
        assert_eq!(repo.log(), vec!["Initial commit"]);
    }

    #[test]
    fn test_log() {
        let mut repo = RepositoryController {
            commit_history: Vec::new(),
            branch_heads: HashMap::new(),
            file_history: HashMap::new(),
        };
        repo.commit("master", "Initial commit".to_string(), vec![]);
        repo.commit("master", "Second commit".to_string(), vec![]);
        assert_eq!(repo.log(), vec!["Initial commit", "Second commit"]);
    }

    #[test]
    fn test_heads() {
        let mut repo = RepositoryController {
            commit_history: Vec::new(),
            branch_heads: HashMap::new(),
            file_history: HashMap::new(),
        };
        repo.commit("master", "Initial commit".to_string(), vec![]);
        repo.commit("master", "Second commit".to_string(), vec![]);
        assert_eq!(repo.heads(), vec!["Second commit"]);
    }

    #[test]
    fn test_retrieve_all_history() {
        let mut repo = RepositoryController {
            commit_history: Vec::new(),
            branch_heads: HashMap::new(),
            file_history: HashMap::new(),
        };
        repo.commit("master", "Initial commit".to_string(), vec![]);
        repo.commit("master", "Second commit".to_string(), vec![]);
        assert_eq!(
            repo.retrieve_all_history(),
            vec!["Initial commit", "Second commit"]
        );
    }

    #[test]
    fn test_retrieve_commit_history() {
        let mut repo = RepositoryController {
            commit_history: Vec::new(),
            branch_heads: HashMap::new(),
            file_history: HashMap::new(),
        };

        // create a new commit
        repo.commit(
            "master",
            "Initial commit".to_string(),
            vec![("README.md".to_string(), "Hello, world!".to_string())],
        );

        // check if the function returns the correct commit history for the file
        assert_eq!(
            repo.retrieve_commit_history("README.md"),
            vec!["Initial commit:Hello, world!"]
        );
    }

    #[test]
    fn test_concatenate_empty_commit_hashes() {
        let mut repo = RepositoryController {
            commit_history: Vec::new(),
            branch_heads: HashMap::new(),
            file_history: HashMap::new(),
        };

        // create new commits
        repo.commit(
            "master",
            "Initial commit".to_string(),
            vec![("README.md".to_string(), "Hello, world!".to_string())],
        );
        repo.commit(
            "master",
            "Second commit".to_string(),
            vec![("README.md".to_string(), "Another change".to_string())],
        );

        // concatenate the two commits
        repo.concatenate("master", vec![]);

        // check if the branch head was correctly updated
        assert_eq!(repo.branch_heads.get("master"), Some(&"".to_string()));

        // check if the commit history was correctly updated
        assert_eq!(
            repo.commit_history,
            vec![
                "Initial commit".to_string(),
                "Second commit".to_string(),
                "".to_string()
            ]
        );
    }

    #[test]
    fn test_checkout() {
        let mut repo = RepositoryController {
            commit_history: Vec::new(),
            branch_heads: HashMap::new(),
            file_history: HashMap::new(),
        };

        // create a new commit
        repo.commit(
            "master",
            "Initial commit".to_string(),
            vec![("README.md".to_string(), "Hello, world!".to_string())],
        );

        // check out the commit
        repo.checkout("master", "Initial commit");

        // check if the branch head was correctly updated
        assert_eq!(
            repo.branch_heads.get("master"),
            Some(&"Initial commit".to_string())
        );

        // check if the commit history was correctly updated
        assert_eq!(
            repo.commit_history,
            vec!["Initial commit".to_string(), "Initial commit".to_string()]
        );
    }
}
