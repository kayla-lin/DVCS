pub mod staging_storage {
    use serde::{Deserialize, Serialize};
    use serde_json;
    use sha1::{Digest, Sha1};
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
    #[derive(Serialize, Deserialize)]
    pub struct Staging {
        dvcs_hidden: String,
        working_directory: String,
        // * HashMap<Path, Metadata of working directory, staging, and repository version>
        index: HashMap<String, StagedComparison>,
    }

    impl Staging {
        /**
         * Staging constructor creates and loads staging file given the dvcs hidden folder and the working directory
         */
        pub fn new(dvcs_hidden: String, working_directory: String) -> Result<Staging, String> {
            // * Check if index file exists, otherwise create it
            match Path::new(&dvcs_hidden).exists() {
                // * Repository cannot be found with given dvcs hidden path
                false => panic!("DVCS Repository cannot be found, cannot create staging file"),
                true => {
                    // * Creating index file if it doesn't exist already
                    if !Path::new(&(dvcs_hidden.to_owned() + "/index.json")).exists() {
                        match File::create(&(dvcs_hidden.to_owned() + "/index.json")) {
                            Ok(x) => x,
                            Err(error) => panic!("Problem creating index file: {:?}", error),
                        };
                    }
                    // * Read index file and load the staging index structure
                    let saved_index = Self::read_from_staging_file(dvcs_hidden.clone());

                    // * Returning successful staging structure if found
                    if (saved_index.is_some()) {
                        return Ok(Staging {
                            dvcs_hidden: dvcs_hidden,
                            working_directory: working_directory,
                            index: saved_index.unwrap(),
                        });
                    }

                    // * Creating new file, return new struct
                    return Ok(Staging {
                        dvcs_hidden: dvcs_hidden,
                        working_directory: working_directory,
                        index: HashMap::new(),
                    });
                }
            };
        }

        /**
         * Add file to staging struct
         */
        pub fn add_file_to_staging(&mut self, file_path: String) -> Option<StagedData> {
            let attrib_result = fs::metadata(file_path.clone());
            if attrib_result.is_ok() {
                let attributes = attrib_result.unwrap();
                let file = self.add_staged_data(attributes, &file_path, 0);
                if file.is_some() {
                    // * Save struct in file
                    self.write_to_staging_file();
                    return file;
                }
                return None;
            }
            None
        }

        /**
         * Remove file to staging struct
         */
        pub fn remove_file_from_staging(&mut self, file_path: String) -> Option<String> {
            let file = self.get_file_from_staging(file_path.to_string()).to_owned();
            if file.is_some() {
                self.index.entry(file_path.to_string()).and_modify(|e| {
                    e.staging = None; // * Save struct in file
                });
                self.write_to_staging_file();
                return Some(file_path);
            }
            None
        }

        /**
         * Remove file comparison from staging struct
         */
        pub fn get_file_from_staging(
            &mut self,
            file_path: String,
        ) -> Option<(&String, &StagedComparison)> {
            let file = self.index.get_key_value(&file_path);
            if file.is_some() {
                return file;
            }
            return None;
        }

        /**
         * Sets the current version of the repository as the snapshot version for comparison in the index file
         */
        pub fn set_staging_snapshot(&mut self, kind: i32) {
            self.recursive_file_traversal(self.working_directory.to_owned(), kind);
            self.write_to_staging_file();
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
        ) -> Option<StagedData> {
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
                    _ => panic!("Wrong kind given"),
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
                        _ => panic!("Wrong kind given"),
                    }
                });
            return Some(Self::create_staged_data_struct(
                metadata.clone(),
                sha1_hex_encode.clone(),
                file_path.to_string(),
            ));
        }

        // *  Private helper function to go through all files in repository minus the DVCS hidden folder
        fn recursive_file_traversal(&mut self, starting_directory: String, kind: i32) {
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
        fn write_to_staging_file(&self) {
            let j = serde_json::to_string(&self.index);
            let file = fs::File::create(self.dvcs_hidden.to_owned() + "/index.json").unwrap();
            let res = serde_json::to_writer(file, &self.index);
        }

        // * Private helper function to read index struct from file
        fn read_from_staging_file(path: String) -> Option<HashMap<String, StagedComparison>> {
            // Open the json file
            let mut file_content = match File::open(path + "/index.json") {
                Ok(file) => file,
                Err(_) => panic!("Could not read the json file"),
            };

            // Transform content of the file into a string
            let mut contents = String::new();
            match file_content.read_to_string(&mut contents) {
                Ok(_) => {}
                Err(err) => panic!("Could not deserialize the file, error code: {}", err),
            };

            // * No content in file
            if contents.as_str().len() == 0 {
                return None;
            }

            let deserialized: HashMap<String, StagedComparison> =
                serde_json::from_str(&contents.as_str()).unwrap();

            Some(deserialized)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        // * Adding a file to be stored in the staging storage successfully
        fn test_add_file_to_staging_success() {
            let mut staging = Staging::new(
                String::from("./src/repo"),
                String::from("./src/working-directory"),
            )
            .unwrap();
            //staging.print_staging_snapshot();
            let file = staging
                .add_file_to_staging("./src/working-directory/folder 1/test2.txt".to_string());
            assert_eq!(file.is_some(), true);
        }

        #[test]
        // * Adding file to be stored in staging storage unsuccessfully because it doesn’t exist
        fn test_add_file_to_staging_fail() {
            let mut staging = Staging::new(
                String::from("./src/repo"),
                String::from("./src/working-directory"),
            )
            .unwrap();
            let file = staging
                .add_file_to_staging("./src/working-directory/folder 1/test2.tt".to_string());

            assert_eq!(file.is_none(), true);
        }

        #[test]
        // *  Remove file from the staging successfully
        fn test_remove_file_from_staging_success() {
            let mut staging = Staging::new(
                String::from("./src/repo"),
                String::from("./src/working-directory"),
            )
            .unwrap();
            staging.add_file_to_staging("./src/working-directory/folder 1/test2.txt".to_string());
            let file = staging
                .remove_file_from_staging("./src/working-directory/folder 1/test2.txt".to_string());
            //staging.print_staging_snapshot();
            assert_eq!(file.is_some(), true);
        }

        #[test]
        // *  Successfully set the repository version of the snapshot
        fn test_set_staging_snapshot_repository() {
            let mut staging = Staging::new(
                String::from("./src/repo"),
                String::from("./src/working-directory"),
            );
            staging.as_mut().unwrap().set_staging_snapshot(2); // 2 = repository version
            assert_eq!(staging.as_ref().is_ok(), true);
        }

        #[test]
        // *  Successfully set the working directory version of the snapshot
        fn test_set_staging_snapshot_working_directory() {
            let mut staging = Staging::new(
                String::from("./src/repo"),
                String::from("./src/working-directory"),
            );
            staging.as_mut().unwrap().set_staging_snapshot(1); // 1 = working directory
            assert_eq!(staging.as_ref().is_ok(), true);
        }

        #[test]
        // * Get staging structure
        fn test_get_staging_struct() {
            let staging = Staging::new(
                String::from("./src/repo"),
                String::from("./src/working-directory"),
            );
            assert_eq!(staging.as_ref().is_ok(), true);
        }

        #[test]
        // * Get file from staging structure
        fn get_file_from_staging() {
            let mut staging = Staging::new(
                String::from("./src/repo"),
                String::from("./src/working-directory"),
            )
            .unwrap();
            staging.add_file_to_staging("./src/working-directory/folder 1/test2.txt".to_string());
            let file = staging
                .get_file_from_staging("./src/working-directory/folder 1/test2.txt".to_string());
            assert_eq!(file.is_some(), true);
        }
    }
}
