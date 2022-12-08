
pub mod user_interaction{
    use stager; 

    pub fn remove_in(file_path: String) -> Result<bool, Vec<String>> {
        return Ok(stager::stager::remove(file_path)); 
    }

    pub fn status_in(file_path: String) -> Result<String, Vec<String>> {
        return stager::stager::status(file_path).map_err(|e| vec![e]);
    }
    pub fn add_in(file_path: String) -> Result<bool, Vec<String>> {
        return Ok(stager::stager::add(file_path));
    }

    


}
