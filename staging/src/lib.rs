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

    #[derive(Debug, Serialize, Deserialize)]
    pub struct StagedData {
        path: String,
        modified: String,
        created: String,
        accessed: String,
        mode: String,
        read_only: String,
        is_file: String,
        sha1: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct StagedComparison {
        working_directory: Option<StagedData>,
        staging: Option<StagedData>,
        repository_version: Option<StagedData>,
    }

    /// Staging storage hiding module
    ///
    /// Designed to interact with the index file storing file metadata for the stager module
    ///
    /// #### How to import module
    ///
    /// ```
    /// use staging::staging_storage::*;
    ///
    /// ```
    #[derive(Serialize, Deserialize)]
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
        /// #### Examples
        ///
        /// ```
        /// use staging::*;
        /// let staging_storage = staging::staging_storage::Staging::new("./.dvcs", "./working_directory");
        ///
        /// // Will wrap the staging structure in a result. If the given paths do not exist, will return an error
        /// staging_storage.unwrap();

        /// ```
        pub fn new(dvcs_hidden: &str, working_directory: &str) -> Result<Staging, String> {
            // * Check if index file exists, otherwise create it
            match Path::new(&dvcs_hidden).exists() {
                // * Repository cannot be found with given dvcs hidden path
                false => return Err("Could not find working directory files".to_string()),
                true => {
                    // * Creating index file if it doesn't exist already
                    let index_path = &(dvcs_hidden.to_owned() + "/index.json");
                    if !Path::new(index_path).exists() {
                        if File::create(&(dvcs_hidden.to_owned() + "/index.json")).is_err() {
                            return Err("Could not create index file".to_string());
                        }
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

        /**
         * Add file to staging struct
         */
        pub fn add_file_to_staging(
            &mut self,
            file_path: &str,
        ) -> Result<Option<StagedData>, String> {
            // * Path to file
            match fs::metadata(file_path.clone()) {
                // * Add data given metadata, filepath
                Ok(attributes) => match self.add_staged_data(attributes, file_path, 0) {
                    Ok(file) => {
                        // * Check to see if writing to file was a success
                        if self.write_to_staging_file().is_err() {
                            return Err("Could not write to staging file".to_string());
                        }
                        return Ok(file);
                    }
                    Err(_) => Err("File does not exist to add".to_string()),
                },
                Err(_) => Err("Could not find file to add".to_string()),
            }
        }

        /**
         * Remove file to staging struct
         */
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
        /**
         * Remove file comparison from staging struct
         */
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

        /**
         * Sets the current version of the repository as the snapshot version for comparison in the index file
         */
        pub fn set_staging_snapshot(&mut self, kind: i32) -> Result<(), String> {
            self.recursive_file_traversal(self.working_directory.to_owned(), kind);
            self.write_to_staging_file()
        }

        // * Private helper function to get nano seconds out of SystemTime structure
        fn get_time_from_metadata(time: SystemTime) -> u128 {
            return time
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("File A thinks it was created before Epoch")
                .as_nanos();
        }

        // * Private helper function to get all useful metadata from file for indexing
        fn create_staged_data_struct(metadata: Metadata, sha1: String, path: String) -> StagedData {
            StagedData {
                modified: Self::get_time_from_metadata(metadata.created().unwrap()).to_string(),
                created: Self::get_time_from_metadata(metadata.created().unwrap()).to_string(),
                accessed: Self::get_time_from_metadata(metadata.accessed().unwrap()).to_string(),
                mode: metadata.mode().to_string(),
                read_only: metadata.permissions().readonly().to_string(),
                is_file: metadata.is_file().to_string(),
                sha1: sha1,
                path: path,
            }
        }

        // * Private helper function to convert file contents to sha1 hex hash
        fn create_sha_1_hex(file_path: String) -> String {
            // * Sha1 hash object into bytes
            let mut hasher = Sha1::new();
            let mut file = fs::File::open(file_path.clone()).unwrap();
            let n = io::copy(&mut file, &mut hasher);
            let result = hasher.finalize();
            // * Encode sha1 encryption into hex
            hex::encode(result)
        }

        // * Private helper function that creates the data being stored & puts it in the corresponding kind
        fn add_staged_data(
            &mut self,
            metadata: Metadata,
            file_path: &str,
            kind: i32,
        ) -> Result<Option<StagedData>, String> {
            // * Creating hex sha1 hash of the contents
            let sha1_hex_encode = Self::create_sha_1_hex(file_path.to_string());

            // * Creating the struct with file path, metadata and sha1 hashed contents
            let data = Self::create_staged_data_struct(
                metadata.clone(),
                sha1_hex_encode.clone(),
                file_path.to_string(),
            );

            self.index
                .entry(file_path.to_string())
                .and_modify(|e| match kind {
                    0 => e.staging = Some(data),
                    1 => e.working_directory = Some(data),
                    2 => e.repository_version = Some(data),
                    _ => panic!("Wrong snapshot type given"), // * Function for internal use only
                })
                .or_insert({
                    match kind {
                        0 => StagedComparison {
                            working_directory: None,
                            staging: Some(Self::create_staged_data_struct(
                                metadata.clone(),
                                sha1_hex_encode.clone(),
                                file_path.to_string(),
                            )),
                            repository_version: None,
                        },
                        1 => StagedComparison {
                            working_directory: Some(Self::create_staged_data_struct(
                                metadata.clone(),
                                sha1_hex_encode.clone(),
                                file_path.to_string(),
                            )),
                            staging: None,
                            repository_version: None,
                        },
                        2 => StagedComparison {
                            working_directory: None,
                            staging: None,
                            repository_version: Some(Self::create_staged_data_struct(
                                metadata.clone(),
                                sha1_hex_encode.clone(),
                                file_path.to_string(),
                            )),
                        },
                        _ => return Err("Wrong kind of snapshot given, use (1-3)".to_string()),
                    }
                });
            return Ok(Some(Self::create_staged_data_struct(
                metadata.clone(),
                sha1_hex_encode.clone(),
                file_path.to_string(),
            )));
        }

        // *  Private helper function to go through all files in repository minus the DVCS hidden folder
        fn recursive_file_traversal(
            &mut self,
            starting_directory: String,
            kind: i32,
        ) -> Result<(), String> {
            if !Path::new(&starting_directory).exists() {
                return Err("Starting directory does not exist".to_string());
            }

            // * Check starting_directory is a valid path
            for entry in fs::read_dir(starting_directory).unwrap() {
                let path_buffer = entry.unwrap().path();
                let file_path = path_buffer.display().to_string();
                // * Loops through all files, not the DVCS hidden one
                if !file_path.contains(&self.dvcs_hidden) {
                    let attrib_result = fs::metadata(file_path.clone());
                    if attrib_result.is_ok() {
                        let attributes = attrib_result.unwrap();
                        if attributes.is_dir() {
                            // * Recursive call of all files in directory
                            let new_directory = file_path.clone();
                            self.add_staged_data(attributes, &file_path, kind);
                            // * Adds file to staging struct
                            Self::recursive_file_traversal(self, new_directory, kind);
                        } else if attributes.is_file() {
                            // * Recursive call of all files in directory
                            self.add_staged_data(attributes, &file_path, kind);
                        }
                    }
                }
            }
            return Ok(());
        }

        /**
         * Prints entire staging file
         */
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

        // * Private helper function to write index struct to file
        fn write_to_staging_file(&self) -> Result<(), String> {
            let write = match fs::File::create(self.dvcs_hidden.to_owned() + "/index.json") {
                Ok(file) => match serde_json::to_writer(file, &self.index) {
                    Ok(_) => Ok(()), // * Successfully written to file
                    Err(_) => Err("Could not write to index file".to_string()),
                },
                Err(_) => Err("Could not find index file".to_string()),
            };
            write
        }

        // * Private helper function to read index struct from file
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
                    let deserialized = serde_json::from_str(&contents.to_string()).unwrap();
                    Ok(deserialized)
                }
                Err(_) => {
                    return Err("Could not open index file".to_string());
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        // * Adding a file to be stored in the staging storage successfully
        fn test_add_file_to_staging_success() {
            let mut staging = Staging::new("./src/repo", "./src/working-directory").unwrap();
            //staging.print_staging_snapshot();
            let file = staging.add_file_to_staging("./src/working-directory/folder 1/test2.txt");
            assert_eq!(file.is_ok(), true);
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
            staging.as_mut().unwrap().set_staging_snapshot(2); // 2 = repository version
            assert_eq!(staging.as_ref().is_ok(), true);
        }

        #[test]
        // *  Successfully set the working directory version of the snapshot
        fn test_set_staging_snapshot_working_directory() {
            let mut staging = Staging::new("./src/repo", "./src/working-directory");
            staging.as_mut().unwrap().set_staging_snapshot(1); // 1 = working directory
            assert_eq!(staging.is_ok(), true);
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
