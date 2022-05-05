use std::fmt;

#[derive(Debug)]
pub enum EntgenError {
    ConfigFilePathError(Box<dyn std::error::Error>),
    ConfigFileParseError(Box<dyn std::error::Error>),
    ConfigEnvError(Box<dyn std::error::Error>),
    DBConnectionError(Box<dyn std::error::Error>),
    DBQueryError(Box<dyn std::error::Error>),
    TemplateBuildFailed(Box<dyn std::error::Error>),
    TemplateFileOpenError(Box<dyn std::error::Error>),
    TemplateFileWriteError(Box<dyn std::error::Error>),
    TemplateDirCreateError(Box<dyn std::error::Error>),
}

impl fmt::Display for EntgenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for EntgenError {}
