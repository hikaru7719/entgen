use std::{fs::File, io::Write, path::Path};

use crate::error::EntgenError;

pub fn write(output_dir: &String, name: &String, body: &String) -> Result<(), EntgenError> {
    let dir = Path::new(&output_dir);

    if !dir.exists() {
        std::fs::create_dir_all(dir)
            .or_else(|err| Err(EntgenError::TemplateDirCreateError(Box::new(err))))?;
    }

    let path = dir.join(format!("{}.rs", name));
    let mut file = File::options()
        .create(true)
        .write(true)
        .open(path)
        .or_else(|err| Err(EntgenError::TemplateFileOpenError(Box::new(err))))?;

    file.write(body.as_bytes())
        .or_else(|err| Err(EntgenError::TemplateFileWriteError(Box::new(err))))?;
    Ok(())
}
