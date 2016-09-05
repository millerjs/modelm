use yaml_rust;
use regex;

quick_error! {
    #[derive(Debug)]
    pub enum KeyboardError {
        /// Parsing Error
        ScanError(err: yaml_rust::ScanError) { from() }
        /// Config Error
        Config(err: String) { from() }
        Regex(err: regex::Error) { from() }
    }
}
