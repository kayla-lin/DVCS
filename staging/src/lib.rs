pub mod staging_storage {
    use serde::{Deserialize, Serialize};
    use serde_json;
    use sha1::{Digest, Sha1};
    use std::borrow::Borrow;
    use std::collections::HashMap;
    use std::fs::{self, File, Metadata};
    use std::io::{self, Read};
    use std::os::unix::prelude::MetadataExt;
    use std::path::Path;
    use std::time::SystemTime;

    /// Structure that keeps relevant metadata and the sha1 hash of the file/folder
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct StagedData {
        pub path: String,
        pub modified: String,
        pub created: String,
        pub accessed: String,
        pub mode: String,
        pub read_only: String,
        pub is_file: String,
        pub sha1: String,
    }

    /// Structure that holds metadata for working directory and repository snapshot as well as the files that are being staged. This structure can be used to compare the versions of the file at different snapshots
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct StagedComparison {
        pub working_directory: Option<StagedData>,
        pub staging: Option<StagedData>,
        pub repository_version: Option<StagedData>,
    }

    /// Staging storage hiding module
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Staging {
        /**
        Path to the hidden folder with the DVCS repository information
         */
        dvcs_hidden: String,
        /**
        Path to working directory
        */
        working_directory: String,
        /**
        HashMap<Path, Metadata of working directory, meta data of staged files, and meta data of current version from repository>
        */
        index: HashMap<String, StagedComparison>,
    }

    impl Staging {
        ///  Staging constructor creates and loads staging file given the dvcs hidden folder and the working directory
        ///
        /// #### Arguments
        ///
        /// * `dvcs_hidden` - A string that holds the path to the DVCS hidden folder (respository folder)
        /// * `working_directory` - A string that holds the path to the working directory folder
        ///
        /// ```
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

/*         pub fn clone(&self) -> Staging {
            return Staging {
            dvcs_hidden : self.dvcs_hidden,
            working_directory: self.working_directory,
            index: self.index.clone(),}
        }*/


        pub fn get_index(self) -> HashMap<String, StagedComparison> {
            self.index
        }

        /// Updates the status of working directory and repository files that are previously added in order to get most recent snapshot of the tracked file - will get most recent metadata/or of both the working directory and staged. If a file is removed from the working directory, will replace that value to None
        pub fn update_staged_files(&mut self) -> Result<(), String> {
            let values: Vec<StagedComparison> = self.index.clone().into_values().collect();
            let update: (Result<(), String>, Option<&StagedComparison>) =
                values.iter().fold((Ok(()), None), |acc, val| {
                    let staging = val.staging.as_ref();
                    // * Will update staged data and working directory (file removed will replace working directory only with None, thus you can check if a staged file has been deleted)
                    if staging.is_some() {
                        if self
                            .add_staged_data(&staging.unwrap().path, 1, true)
                            .is_ok()
//                            && self
//                                .add_staged_data(&staging.unwrap().path, 0, false)
//                                .is_ok()
                        {
                            return (self.write_to_staging_file(), Some(val));
                        }
                    }
                    return (acc.0, Some(val));
                });
            self.write_to_staging_file()
        }

        /// Function that deletes index file and creates new one to reduce writing conflict
        pub fn recreate_index_file(dvcs_hidden: &str) -> Result<(), String> {
            match Path::new(&dvcs_hidden).try_exists() {
                // * Repository cannot be found with given dvcs hidden path
                Err(_) => return Err("Could not find working directory files".to_string()),
                Ok(_) => {
                    // * Creating index file if it doesn't exist already
                    let index_path = &(dvcs_hidden.to_owned() + "/index.json");
                    match Path::new(index_path).try_exists() {
                        Err(_) => return Err("Could not create index file".to_string()),
                        // * If there is an index file, remove it
                        Ok(true) => {
                            if fs::remove_file(index_path).is_err() {
                                return Err("Error creating new index file".to_string());
                            }
                        }
                        // * If there isn't an index file, ignore
                        Ok(false) => {}
                    };
                    // * Create new index file
                    if File::create(index_path).is_err() {
                        return Err("Could not create index file".to_string());
                    }
                    Ok(())
                }
            }
        }

        /// Add file to staging structure
        pub fn add_file_to_staging(&mut self, file_path: &str) -> Result<(), String> {
            // * Path to file
            // * Add data given metadata, filepath
            match self.add_staged_data(file_path, 0, false) {
                Ok(_) => {
                    // * Check to see if writing to file was a success
                    self.write_to_staging_file()
                }
                Err(_) => Err("File does not exist to add".to_string()),
            }
        }

        /// Add file(s) to staging structure starting from directory
        pub fn add_directory_to_staging(
            &mut self,
            kind: i32,
            directory_path: &str,
        ) -> Result<(), String> {
            if let Ok(attributes) = fs::metadata(directory_path) {
                if attributes.is_dir() {
                    self.recursive_file_traversal(directory_path, kind);
                    return self.write_to_staging_file();
                } else if attributes.is_file() {
                    return Err("Path must be to a directory".to_string());
                }
            }
            Err("Directory path does not exist".to_string())
        }

        /// Remove file from staging structure
        pub fn remove_file_from_staging(&mut self, file_path: &str) -> Result<(), String> {
            match self.get_file_from_staging(file_path).to_owned() {
                Ok(_) => {
                    self.index.entry(file_path.to_string()).and_modify(|e| {
                        e.staging = None; // * Save struct in file
                    });
                    self.write_to_staging_file()
                        .or_else(|_| return Err("Cannot find file to remove".to_string()))
                }
                Err(_) => Err("Cannot find file to remove".to_string()),
            }
        }

        /// Gets working directory, repository, and staging version of the file
        pub fn get_file_from_staging(
            &mut self,
            file_path: &str,
        ) -> Result<(&String, &StagedComparison), String> {
            let file = self.index.get_key_value(file_path);
            match file {
                Some(file) => Ok(file),
                None => Err("No file found in staging index".to_string()),
            }
        }

        /// Sets the current version of the repository as the snapshot version for comparison in the index file
        pub fn set_staging_snapshot(&mut self, kind: i32) -> Result<(), String> {
            self.recursive_file_traversal(self.working_directory.clone().as_str(), kind);
            self.write_to_staging_file()
        }

        /// Private helper function to get nano seconds out of SystemTime structure
        fn get_time_from_metadata(time: io::Result<SystemTime>) -> Result<String, String> {
            if let Ok(nano_time) = time {
                return Ok(nano_time
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("File A thinks it was created before Epoch")
                    .as_nanos()
                    .to_string());
            } else {
                return Err("Could not get time from metadata".to_string());
            }
        }

        /// Private helper function to get all useful metadata from file for indexing
        fn create_staged_data_struct(
            metadata: Metadata,
            sha1: String,
            path: String,
        ) -> Result<StagedData, String> {
            let modified = Self::get_time_from_metadata(metadata.modified());
            let created = Self::get_time_from_metadata(metadata.created());
            let accessed = Self::get_time_from_metadata(metadata.accessed());
            if let Ok(accessed) = accessed {
                if let Ok(created) = created {
                    if let Ok(modified) = modified {
                        return Ok(StagedData {
                            mode: metadata.mode().to_string(),
                            read_only: metadata.permissions().readonly().to_string(),
                            is_file: metadata.is_file().to_string(),
                            modified,
                            created,
                            accessed,
                            sha1,
                            path,
                        });
                    }
                }
            }
            return Err("Could not get metadata from file".to_string());
        }

        /// Private helper function to convert file contents to sha1 hex hash
        fn create_sha_1_hex(file_path: &str) -> Result<String, String> {
            // * Sha1 hash object into bytes
            let mut hasher = Sha1::new();
            if let Ok(mut file) = fs::File::open(file_path) {
                io::copy(&mut file, &mut hasher);
                let result = hasher.finalize();
                // * Encode sha1 encryption into hex
                return Ok(hex::encode(result));
            } else {
                return Err("Could not open file to hash".to_string());
            }
        }

        /// Private helper function that creates the data being stored & puts it in the corresponding kind
        fn add_staged_data(
            &mut self,
            file_path: &str,
            kind: i32,
            add_nulls: bool,
        ) -> Result<(), String> {
            // * Get metadata
            let attrib_result = fs::metadata(file_path.clone());
            match attrib_result {
                Ok(metadata) => {
                    // * Creating hex sha1 hash of the contents
                    match Self::create_sha_1_hex(file_path) {
                        Ok(sha1_hex_encode) => {
                            // * Creating the struct with file path, metadata and sha1 hashed contents
                            if let Ok(created_data) = Self::create_staged_data_struct(
                                metadata.clone(),
                                sha1_hex_encode.clone(),
                                file_path.to_string(),
                            ) {
                                self.index
                                    .entry(file_path.to_string())
                                    .and_modify(|e| match kind {
                                        0 => e.staging = Some(created_data),            // 0 - staging
                                        1 => e.working_directory = Some(created_data), // 1 - working directory
                                        _ => e.repository_version = Some(created_data), // 2 - repository version
                                    })
                                    .or_insert({
                                        let data = match Self::create_staged_data_struct(
                                            metadata.clone(),
                                            sha1_hex_encode.clone(),
                                            file_path.to_string(),
                                        ) {
                                            Ok(data) => Some(data),
                                            Err(_) => None,
                                        };
                                        if data.is_none() {
                                            return Err(
                                                "Could not create data from given parameters"
                                                    .to_string(),
                                            );
                                        }
                                        match kind {
                                            0 => StagedComparison {
                                                working_directory: None,
                                                staging: data,
                                                repository_version: None,
                                            },
                                            1 => StagedComparison {
                                                working_directory: data,
                                                staging: None,
                                                repository_version: None,
                                            },
                                            _ => StagedComparison {
                                                working_directory: None,
                                                staging: None,
                                                repository_version: data,
                                            },
                                        }
                                    });
                                return Ok(());
                            }
                        }
                        Err(err) => return Err(err),
                    }
                    return Err("Could not create data from given parameters".to_string());
                }
                // * If file cannot be found set the index as a blank
                Err(_) => {
                    println!("Add to staging");
                    if add_nulls {
                        self.index
                            .entry(file_path.to_string())
                            .and_modify(|e| match kind {
                                0 => e.staging = None,
                                1 => e.working_directory = None,
                                _ => e.repository_version = None,
                            });
                        return Ok(());
                    }
                    return Err("Could not add file to structure".to_string());
                }
            }
        }

        ///  Private helper function to go through all files in repository minus the DVCS hidden folder
        fn recursive_file_traversal(&mut self, starting_directory: &str, kind: i32) {
            // * Check if starting directory exists
            match Path::new(&starting_directory).try_exists() {
                // * Error with opening file
                Err(_) => return,
                // * starting_directory not a valid path
                Ok(false) => return,
                Ok(true) => {
                    // * Check starting_directory is a valid path
                    match fs::read_dir(starting_directory) {
                        Ok(file_entry) => {
                            // * Loop through each entry in the directory
                            file_entry.into_iter().for_each(|entry| {
                                if let Ok(path) = entry {
                                    let path_buffer = path.path();
                                    let file_path = path_buffer.display().to_string();
                                    // * Loops through all files, not the DVCS hidden one
                                    if !file_path.contains(&self.dvcs_hidden) {
                                        if let Ok(attributes) = fs::metadata(file_path.clone()) {
                                            if attributes.is_dir() {
                                                // * Recursive call of all files in directory
                                                let new_directory = file_path.clone();
                                                self.add_staged_data(&new_directory, kind, false);
                                                // * Adds file to staging struct
                                                Self::recursive_file_traversal(
                                                    self,
                                                    &new_directory,
                                                    kind,
                                                );
                                            } else if attributes.is_file() {
                                                // * Recursive call of all files in directory
                                                self.add_staged_data(&file_path, kind, false);
                                            }
                                        }
                                    }
                                }
                            })
                        }
                        Err(_) => return,
                    };
                }
            }
        }

        /// Print staging snapshot (index.json)
        pub fn print_staging_snapshot(&self) {
            self.index.iter().for_each(|x| {
                println!("PATH (key): {:?}", &x.0);
                println!("STAGING:");
                println!("{:?}", &x.1.staging);
                println!("WORKING DIRECTORY:");
                println!("{:?}", &x.1.working_directory);
                println!("REPOSITORY VERSION:");
                println!("{:?}", &x.1.repository_version);
                println!();
            });
        }

        /// Private helper function to delete and create index file
        fn write_to_staging_file(&self) -> Result<(), String> {
            if Self::recreate_index_file(&self.dvcs_hidden).is_ok() {
                return match fs::File::create(self.dvcs_hidden.to_owned() + "/index.json") {
                    Ok(file) => {
                        return match serde_json::to_writer(file, &self.index) {
                            Ok(_) => Ok(()), // * Successfully written to file
                            Err(_) => Err("Could not write to index file".to_string()),
                        };
                    }
                    Err(_) => Err("Could not recreate index file".to_string()),
                };
            }
            return Err("Could not find index file".to_string());
        }

        /// Private helper function to read index struct from file
        fn read_from_staging_file(
            path: String,
        ) -> Result<HashMap<String, StagedComparison>, String> {
            // * Open the json file
            match File::open(path + "/index.json") {
                Ok(index_file) => {
                    let mut contents = String::new();
                    // * Creating string from contents in index file
                    index_file
                        .borrow()
                        .read_to_string(&mut contents)
                        .or_else(|_| {
                            return Err("No content found in staging file".to_string());
                        })
                        .unwrap();
                    // * Check if file is empty, create empty hashmap
                    if contents.to_string().len() == 0 {
                        return Ok(HashMap::new());
                    }
                    // * Otherwise, return structure created from file, TODO: safe way to check if this works?
                    if let Ok(deserialized) = serde_json::from_str(&contents.to_string()) {
                        return Ok(deserialized);
                    } else {
                        return Err("Could not deserialize index file".to_string());
                    }
                }
                Err(_) => {
                    return Err("Could not open index file".to_string());
                }
            }
        }
    }

    /// Note: As these functions are writing to files, parallel testing will overwrite one another and fail
    /// PLEASE USE THIS IF YOU WANT TO TEST MULTIPLE AT ONCE: cargo test -- --test-threads=1
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        // * Update staging file with most recent version
        fn update_staged_files() {
            let mut staging = Staging::new("./src/repo", "./src/working-directory").unwrap();
            let update = staging.update_staged_files();
            assert_eq!(update.is_ok(), true);
        }

        #[test]
        // * Adding a file to be stored in the staging storage successfully
        fn test_add_file_to_staging_success() {
            let mut staging = Staging::new("./src/repo", "./src/working-directory").unwrap();
            let file = staging.add_file_to_staging("./src/working-directory/folder 1/test2.txt");
            assert_eq!(file.is_ok(), true);
        }

        #[test]
        // * Adding a file to be stored in the staging storage successfully
        fn test_add_directory_to_staging_success() {
            let mut staging = Staging::new("./src/repo", "./src/working-directory").unwrap();
            let file = staging.add_directory_to_staging(0, "./src/working-directory/folder 1");
            assert_eq!(file.is_ok(), true);
        }

        #[test]
        // * Adding a file to be stored in the staging storage failure, not a directory given
        fn test_add_directory_to_staging_fail() {
            let mut staging = Staging::new("./src/repo", "./src/working-directory").unwrap();
            // * Path to file given, will fail
            let file = staging.add_directory_to_staging(0, "./src/working-directory/test1.txt");
            assert_eq!(file.is_err(), true);
        }

        #[test]
        // * Adding file to be stored in staging storage unsuccessfully because it doesn’t exist
        fn test_add_file_to_staging_fail() {
            let mut staging = Staging::new("./src/repo", "./src/working-directory").unwrap();
            let file = staging.add_file_to_staging("./src/working-directory/folder 1/test2.xyz123");
            assert_eq!(file.is_err(), true);
        }

        #[test]
        // *  Remove file from the staging successfully
        fn test_remove_file_from_staging_success() {
            let mut staging = Staging::new("./src/repo", "./src/working-directory").unwrap();
            staging.add_file_to_staging("./src/working-directory/folder 1/test2.txt");
            let file =
                staging.remove_file_from_staging("./src/working-directory/folder 1/test2.txt");
            //staging.print_staging_snapshot();
            assert_eq!(file.is_ok(), true);
        }

        #[test]
        // *  Successfully set the repository version of the snapshot
        fn test_set_staging_snapshot_repository() {
            let mut staging = Staging::new("./src/repo", "./src/working-directory");
            let set = staging.as_mut().unwrap().set_staging_snapshot(2); // 2 = repository version
            assert_eq!(set.is_ok(), true);
        }

        #[test]
        // *  Successfully set the working directory version of the snapshot
        fn test_set_staging_snapshot_working_directory() {
            let mut staging = Staging::new("./src/repo", "./src/working-directory");
            let set = staging.as_mut().unwrap().set_staging_snapshot(1); // 1 = working directory
            assert_eq!(set.is_ok(), true);
        }

        #[test]
        // * Get staging structure
        fn test_get_staging_struct() {
            let staging = Staging::new("./src/repo", "./src/working-directory");
            assert_eq!(staging.is_ok(), true);
        }

        #[test]
        // * Get staging structure, failure - repo doesn't exist
        fn test_get_staging_struct_fail() {
            let staging = Staging::new("./src/repo123", "./src/working-directory12312313");
            assert_eq!(staging.is_err(), true);
        }

        #[test]
        // * Get file from staging structure
        fn get_file_from_staging() {
            let mut staging = Staging::new("./src/repo", "./src/working-directory").unwrap();
            staging.add_file_to_staging("./src/working-directory/folder 1/test2.txt");
            let file = staging.get_file_from_staging("./src/working-directory/folder 1/test2.txt");
            assert_eq!(file.is_ok(), true);
        }

        #[test]
        // * Get file from staging structure, fail because file doesn't exist
        fn get_file_from_staging_fail() {
            let mut staging = Staging::new("./src/repo", "./src/working-directory").unwrap();
            let file = staging.get_file_from_staging("./src/working-directory/folder 1/test2.xyz");
            assert_eq!(file.is_err(), true);
        }
    }
}
