use std::collections::HashMap;

struct RepositoryController {
    // vector to store the commit history for the repository
    commit_history: Vec<String>,
    // map to store the head commits for each branch in the repository
    branch_heads: HashMap<String, String>,

    // map to store the commit history for each file in the repository
    file_history: HashMap<String, Vec<String>>,
}

impl RepositoryController {
    // commits the current state of the repository to storage
    fn commit(&mut self, branch: &str, commit_message: String) {
        // updates  the head commit for the specified branch
        self.branch_heads
            .insert(branch.to_string(), commit_message.clone());

        // adds the commit message to the commit history
        self.commit_history.push(commit_message);

        // updatse the file history with the new commit
        for (file_path, file_commits) in &mut self.file_history {
            file_commits.push(format!("{}:{}", commit_message, file_path));
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
        self.commit(branch, new_commit_message);
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
        self.commit(branch, commit_message);
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
        repo.commit("Initial commit".to_string());
        assert_eq!(repo.log(), vec!["Initial commit"]);
    }

    #[test]
    fn test_log() {
        let mut repo = RepositoryController {
            commit_history: Vec::new(),
            branch_heads: HashMap::new(),
            file_history: HashMap::new(),
        };
        repo.commit("Initial commit".to_string());
        repo.commit("Second commit".to_string());
        assert_eq!(repo.log(), vec!["Initial commit", "Second commit"]);
    }

    #[test]
    fn test_heads() {
        let mut repo = RepositoryController {
            commit_history: Vec::new(),
            branch_heads: HashMap::new(),
            file_history: HashMap::new(),
        };
        repo.commit("Initial commit".to_string());
        repo.commit("Second commit".to_string());
        assert_eq!(repo.heads(), vec!["Second commit"]);
    }

    #[test]
    fn test_retrieve_all_history() {
        let mut repo = RepositoryController {
            commit_history: Vec::new(),
            branch_heads: HashMap::new(),
            file_history: HashMap::new(),
        };
        repo.commit("Initial commit".to_string());
        repo.commit("Second commit".to_string());
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
        repo.commit("Initial commit".to_string());
        repo.commit("Second commit".to_string());
        assert_eq!(
            repo.retrieve_commit_history("test_file.txt"),
            vec![
                "Initial commit:test_file.txt",
                "Second commit:test_file.txt"
            ]
        );
    }

    #[test]
    fn test_concatenate() {
        let mut repo = RepositoryController {
            commit_history: Vec::new(),
            branch_heads: HashMap::new(),
            file_history: HashMap::new(),
        };
        repo.commit("Initial commit".to_string());
        repo.commit("Second commit".to_string());
        repo.concatenate(vec![
            "Initial commit".to_string(),
            "Second commit".to_string(),
        ]);
        assert_eq!(
            repo.log(),
            vec![
                "Initial commit",
                "Second commit",
                "Initial commit\nSecond commit"
            ]
        );
    }
}
