#[derive(Debug)]
pub struct Error; 

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error searching Anilist")
    }
}

impl std::error::Error for Error {

}