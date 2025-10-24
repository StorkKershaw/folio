use actix_files::Directory;
use image::ImageReader;
use std::{fs::DirEntry, io, path::Path};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Resource {
    pub(crate) title: String,
    pub(crate) href: String,
}

impl Resource {
    pub(crate) fn parent(base_url: &Path) -> Option<Self> {
        Some(Self {
            title: "..".to_owned(),
            href: base_url.parent()?.to_string_lossy().replace('\\', "/"),
        })
    }

    pub(crate) fn from_read_result(
        read_result: io::Result<DirEntry>,
        base_url: &Path,
        directory: &Directory,
    ) -> io::Result<(Self, bool)> {
        let dir_entry = read_result?;
        let is_dir = dir_entry.metadata()?.is_dir();

        let path = dir_entry.path();
        if !is_dir {
            ImageReader::open(&path)?
                .with_guessed_format()?
                .format()
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::Other, "File is not a valid image format")
                })?;
        }

        match path.strip_prefix(&directory.path) {
            Ok(p) => {
                let title = dir_entry.file_name().to_string_lossy().into_owned();
                let href = base_url.join(p).to_string_lossy().replace('\\', "/");
                Ok((Self { title, href }, is_dir))
            }
            Err(_) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Path is not a subdirectory of the directory",
            )),
        }
    }
}
