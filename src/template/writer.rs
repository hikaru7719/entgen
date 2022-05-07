use std::{fs::File, io::Write, path::Path};

use crate::error::EntgenError;

pub fn write(output_dir: &String, name: &String, body: &String) -> Result<(), EntgenError> {
    let dir = Path::new(&output_dir);
    let path = dir.join(format!("{}.rs", name));
    let mut file: File;

    if dir.exists() && path.exists() {
        file = File::open(path)
            .or_else(|err| Err(EntgenError::TemplateFileOpenError(Box::new(err))))?;
    } else if dir.exists() {
        file = File::create(path)
            .or_else(|err| Err(EntgenError::TemplateFileOpenError(Box::new(err))))?;
    } else {
        std::fs::create_dir_all(dir)
            .or_else(|err| Err(EntgenError::TemplateDirCreateError(Box::new(err))))?;

        file = File::create(path)
            .or_else(|err| Err(EntgenError::TemplateFileOpenError(Box::new(err))))?;
    }

    file.write(body.as_bytes())
        .or_else(|err| Err(EntgenError::TemplateFileWriteError(Box::new(err))))?;
    Ok(())
}
