pub mod user_interaction{
    use crate::stager; 

    pub fn diff(file_path: String, head: String) -> Result<String, Vec<String>> {
        return stager::diff(file_path, head);
    }
    pub fn remove(file_path: String) -> Result<bool, Vec<String>> {
        return stager::remove(file_path).map_err(|e| vec![e]);
    }
}