pub mod stager {
    use staging::staging_storage::Staging;
    use std::fs;
    use std::fs::File;

const DVCS_HIDDEN:&str = "/tmp/dvcs_team";

fn show_diff(current_path: String, orig: String) -> String{
    return String::from("");
}

fn is_repo(current_path: String) -> bool {
    return true;
}

fn is_changed(current_path: String) -> bool {
    return true;
}

pub struct Repo {
    root_path: String,
    head: String,
    modified: bool
}
impl Repo {}

pub fn diff(file_path: String, head: String) -> Result<String, String> {
    let mut path:String;
    if head.is_empty() {
        //find path to head origin or error
        return Ok(String::from(""));
    } else {
        //find path to revision or error
        path = String::from("/tmp/1");
    }
    return Ok(show_diff(file_path, path));
}

pub fn status(file_path: String) -> Result<String, String> {
    if file_path.is_empty() {
        return Err(String::from("empty path"));
    } else {
        return Ok(String::from("state"));
    }
}

pub fn add(file_path: String) -> bool {
    if file_path.is_empty() {
        return true;
    } else {
        return true;
    }
}

pub fn remove(file_path: String) -> bool {
    if file_path.is_empty() {
        return true;
    } else {
        return true;
    }
}

pub fn init(file_path: String) -> bool {
    if file_path.is_empty() {
        return false;
    } else {
        let staging = Staging::new(
            String::from(DVCS_HIDDEN),
            String::from(file_path),
        );
        if staging.is_ok() {
            staging.unwrap().set_staging_snapshot(1); // 1 = working directory
            return true;
        } else {
            return false;
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // * Adding a file to be stored in the staging storage successfully
    fn success_diff() {
       let a = diff(String::from("/tmp/one"), String::from(""));
       
       assert_eq!(a, Ok(String::from("")));
    }


    #[test]
    // * Adding a file to be stored in the staging storage successfully
    fn all_status() {
       let a = status(String::from("/tmp/one"));
       
       assert_eq!(a, Ok(String::from("state")));
       assert_eq!(status(String::from("")), Err(String::from("empty path")));

    }


    #[test]
    // * Adding a file to be stored in the staging storage successfully
    fn all_add() {
       let a = add(String::from("/tmp/one"));
       
       assert_eq!(a, true);

    }


    #[test]
    // * Adding a file to be stored in the staging storage successfully
    fn all_remove() {
       let a = remove(String::from("/tmp/one"));
       
       assert_eq!(a, true);

    }
    #[test]
    // * Adding a file to be stored in the staging storage successfully
    fn all_init() {
        fs::create_dir(DVCS_HIDDEN);

        fs::create_dir("/tmp/dvcs_test/");

       let file = File::create("/tmp/dvcs_test/");
       let b = init(String::from("/tmp/dvcs_test/"));
       assert_eq!(b, true);


    }



}

}