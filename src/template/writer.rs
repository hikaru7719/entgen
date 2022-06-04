use std::{fs::File, io::Write, path::Path};

use crate::error::EntgenError;

pub fn write(output_dir: &String, name: &String, body: &String) -> Result<(), EntgenError> {
    let dir = Path::new(&output_dir);

    if !dir.exists() {
        std::fs::create_dir_all(dir)
            .or_else(|err| Err(EntgenError::TemplateDirCreateError(Box::new(err))))?;
    }

    File::options()
        .create(true)
        .write(true)
        .open(dir.join(format!("{}.rs", name)))
        .or_else(|err| Err(EntgenError::TemplateFileOpenError(Box::new(err))))?
        .write(body.as_bytes())
        .or_else(|err| Err(EntgenError::TemplateFileWriteError(Box::new(err))))?;

    Ok(())
}
