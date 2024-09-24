// Code: constants.rs

pub mod constants {
    /// Constants for SAS7BDAT files
    #[derive(Debug)]
    pub enum SasCatalog {
        /// The first index page of the SAS7BDAT file
        FirstIndexPage = 1,

        /// The number of useless pages in the SAS7BDAT file
        UselessPages = 3,
    }
}
