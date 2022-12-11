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

    pub struct Stager {
        //    staging: Staging,
    }

    impl Stager {
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

        pub fn status(file_path: String) -> Result<String, String> {
            if file_path.is_empty() {
                return Err(String::from("empty path"));
            } else {
                let mut output: String;
                output = String::from("changed:\n");
                let staging = Staging::new(DVCS_HIDDEN, file_path.as_str());

                let values: Vec<StagedComparison> =
                    staging.unwrap().get_index().clone().into_values().collect();
                let update: String = values.iter().fold(output, |mut acc, val| {
                    let staging_ = val.staging.clone().unwrap();
                    let other_ = val.working_directory.clone().unwrap();
                    if staging_.modified != other_.modified
                        || staging_.created != other_.created
                        || staging_.mode != other_.mode
                        || staging_.sha1 != other_.sha1
                    {
                        acc.push_str(staging_.path.as_str());
                    }
                    return acc;
                });
                return Ok(update);
            }

        }

        pub fn add(file_path: String) -> Result<(), String> {
            if file_path.is_empty() {
                return Err(String::from("No path specified"));
            } else {
                let staging = Staging::new(DVCS_HIDDEN, file_path.as_str());
                if staging.is_ok() {
                    let res = staging.unwrap().add_file_to_staging(file_path.as_str());
                    if res.is_ok() {
                        return Ok(());
                    } else {
                        return Err(res.err().unwrap());
                    }
                } else {
                    return Err(staging.err().unwrap());
                }
            }
        }

        pub fn remove(file_path: String) -> Result<(), String> {
            if file_path.is_empty() {
                return Err(String::from("No path specified"));
            } else {
                let staging = Staging::new(DVCS_HIDDEN, file_path.as_str());
                if staging.is_ok() {
                    let res = staging
                        .unwrap()
                        .remove_file_from_staging(file_path.as_str());
                    if res.is_ok() {
                        return Ok(());
                    } else {
                        return Err(res.err().unwrap());
                    }
                } else {
                    return Err(staging.err().unwrap());
                }
            }
        }

        pub fn init(file_path: String) -> Result<(), String> {
            if file_path.is_empty() {
                return Err(String::from("No path specified"));
            } else {
                let contents: Result<fs::ReadDir, io::Error> = fs::read_dir(&file_path);
                if contents.unwrap().next().is_some() {
                    return Err(String::from("Directory not empty"));
                }
                let staging = Staging::new(DVCS_HIDDEN, file_path.as_str());
                if staging.is_ok() {
                    staging.unwrap().set_staging_snapshot(1); // 1 = working directory
                    return Ok(());
                } else {
                    return Err(staging.err().unwrap());
                }
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
            let stager_i = Stager {};
            let a = stager_i.diff(String::from("/tmp/one"), String::from(""));
            //            let a = Stager::diff(String::from("/tmp/one"), String::from(""));

            //assert_eq!(a, Ok(String::from("")));
        }

        #[test]
        // * Adding a file to be stored in the staging storage successfully
        fn all_status() {
            //               let stager_ = Stager {};
            //               let a = stager_.status(String::from("/tmp/one"));
            fs::create_dir(DVCS_HIDDEN);

            fs::remove_dir_all("/tmp/dvcs_test/");
            fs::create_dir("/tmp/dvcs_test/");

            let a = stager::Stager::init(String::from("/tmp/dvcs_test/"));

            let file = File::create("/tmp/dvcs_test/one.txt");
            stager::Stager::add(String::from("/tmp/dvcs_test/one.txt"));

            let a = stager::Stager::status(String::from("/tmp/dvcs_test/"));

            assert_eq!(a, Ok(String::from("state")));
            assert_eq!(
                stager::Stager::status(String::from("")),
                Err(String::from("empty path"))
            );
        }

        #[test]
        // * Adding a file to be stored in the staging storage successfully
        fn all_add() {
            let a = stager::Stager::add(String::from("/tmp/one"));

            assert_eq!(a.is_ok(), true);
        }

        #[test]
        // * Adding a file to be stored in the staging storage successfully
        fn all_remove() {
            let a = stager::Stager::remove(String::from("/tmp/one"));

            assert_eq!(a.is_ok(), true);
        }
        #[test]
        // * Adding a file to be stored in the staging storage successfully
        fn all_init() {
            fs::create_dir(DVCS_HIDDEN);

            fs::remove_dir_all("/tmp/dvcs_test/");
            fs::create_dir("/tmp/dvcs_test/");

            let b = stager::Stager::init(String::from("/tmp/dvcs_test/"));
            assert_eq!(b.is_ok(), true);

            let file = File::create("/tmp/dvcs_test/one.txt");
            let b = stager::Stager::init(String::from("/tmp/dvcs_test/"));
            assert_eq!(b.is_err(), true);
        }
    }
}
