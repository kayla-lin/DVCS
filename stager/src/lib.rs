pub mod stager {
    use staging::staging_storage::{StagedComparison, Staging};
    use std::fs;
    use std::fs::File;
    use std::io;

    const DVCS_HIDDEN: &str = "/tmp/dvcs_team";

    pub struct Repo {
        root_path: String,
        head: String,
        modified: bool,
    }
    impl Repo {}

    #[derive(Clone, Debug)]
    pub struct Stager {
        staging: Staging,
    }

    impl Stager {
        pub fn new(dvcs_hidden: &str, working_directory: &str) -> Result<Stager, String> {
            let staging_ = Staging::new(dvcs_hidden, working_directory);
            if staging_.is_err() {
                return Err(staging_.err().unwrap());
            }
            return Ok(Stager {
                staging: staging_.unwrap(),
            });
        }

        fn show_diff(current_path: String, orig: String) -> String {
            return String::from("");
        }

        fn is_repo(current_path: String) -> bool {
            return true;
        }

        fn is_changed(current_path: String) -> bool {
            return true;
        }

        pub fn diff(self, file_path: String, head: String) -> Result<String, String> {
            let mut path: String;
            if head.is_empty() {
                //find path to head origin or error
                return Ok(String::from(""));
            } else {
                //find path to revision or error
                path = String::from("/tmp/1");
            }
            return Ok(Self::show_diff(file_path, path));
        }

        pub fn status(&mut self, file_path: String) -> Result<String, String> {
            if file_path.is_empty() {
                return Err(String::from("empty path"));
            } else {
                self.staging.update_staged_files();
                // let ddd = &self.staging;

                let values: Vec<StagedComparison> = self
                    .staging
                    .clone()
                    .get_index()
                    .clone()
                    .into_values()
                    .collect();
                let update: String =
                    values
                        .iter()
                        .fold(String::from("changed:\n"), |mut acc, val| {
                            let staging_ = val.staging.clone();
                            let other_ = val.working_directory.clone();
                            if staging_.is_none() || other_.is_none() {
                                return acc;
                            }
                            let sta = staging_.clone().unwrap();
                            let oth = other_.unwrap();
                            if sta.modified != oth.modified
                                || sta.created != oth.created
                                || sta.mode != oth.mode
                                || sta.sha1 != oth.sha1
                            {
                                acc.push_str(staging_.clone().unwrap().path.as_str());
                            }
                            return acc;
                        });
                return Ok(update);
            }
        }

        pub fn add(mut self, file_path: String) -> Result<(), String> {
            if file_path.is_empty() {
                return Err(String::from("No path specified"));
            } else {
                // let staging = Staging::new(DVCS_HIDDEN, file_path.as_str());
                let res = self.staging.add_file_to_staging(file_path.as_str());
                if res.is_ok() {
                    return Ok(());
                } else {
                    return Err(res.err().unwrap());
                }
            }
        }

        pub fn remove(mut self, file_path: String) -> Result<(), String> {
            if file_path.is_empty() {
                return Err(String::from("No path specified"));
            } else {
                let res = self.staging.remove_file_from_staging(file_path.as_str());
                if res.is_ok() {
                    return Ok(());
                } else {
                    return Err(res.err().unwrap());
                }
            }
        }

        pub fn init(mut self, file_path: String) -> Result<(), String> {
            if file_path.is_empty() {
                return Err(String::from("No path specified"));
            } else {
                let contents: Result<fs::ReadDir, io::Error> = fs::read_dir(&file_path);
                if contents.unwrap().next().is_some() {
                    return Ok(());
                }
                self.staging.set_staging_snapshot(1); // 1 = working directory
                return Ok(());
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::stager;

        use super::*;

        #[test]
        // * Adding a file to be stored in the staging storage successfully
        fn success_diff() {
            let stager_i = Stager::new(DVCS_HIDDEN, "/tmp/one").unwrap();
            let a = stager_i.diff(String::from("/tmp/one"), String::from(""));

            assert_eq!(a, Ok(String::from("")));
        }

        #[test]
        // * Adding a file to be stored in the staging storage successfully
        fn all_status() {
            //               let stager_ = Stager {};
            //               let a = stager_.status(String::from("/tmp/one"));
            fs::create_dir(DVCS_HIDDEN);

            fs::remove_dir_all("/tmp/dvcs_test/");
            fs::create_dir("/tmp/dvcs_test/");

            let mut stager_i = Stager::new(DVCS_HIDDEN, "/tmp/one").unwrap();

            let a = stager_i.init(String::from("/tmp/dvcs_test/"));

            let file = File::create("/tmp/dvcs_test/one.txt");
            // stager_i.add(String::from("/tmp/dvcs_test/one.txt"));

            //            let a = stager_i.clone().status(String::from("/tmp/dvcs_test/"));

            assert_eq!(a.is_ok(), true);
            //            assert_eq!(stager_i.clone().status(String::from("")), Err(String::from("empty path")));
        }

        #[test]
        // * Adding a file to be stored in the staging storage successfully
        fn all_add() {
            let stager_i = Stager::new(DVCS_HIDDEN, "/tmp/one").unwrap();
            let a = stager_i.add(String::from("/tmp/one"));

            assert_eq!(a.is_ok(), false);
        }

        #[test]
        // * Adding a file to be stored in the staging storage successfully
        fn all_remove() {
            let stager_i = Stager::new(DVCS_HIDDEN, "/tmp/one").unwrap();
            let a = stager_i.remove(String::from("/tmp/one"));

            assert_eq!(a.is_ok(), false);
        }
        #[test]
        // * Adding a file to be stored in the staging storage successfully
        fn all_init() {
            fs::create_dir(DVCS_HIDDEN);

            fs::remove_dir_all("/tmp/dvcs_test/");
            fs::create_dir("/tmp/dvcs_test/");
            let stager_i = Stager::new(DVCS_HIDDEN, "/tmp/one").unwrap();

            let b = stager_i.clone().init(String::from("/tmp/dvcs_test/"));
            assert_eq!(b.is_ok(), true);

            let file = File::create("/tmp/dvcs_test/one.txt");
            let b = stager_i.clone().init(String::from("/tmp/dvcs_test/"));
            assert_eq!(b.is_ok(), true);
        }
    }
}
