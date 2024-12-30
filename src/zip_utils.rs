use std::{
    fs::File,
    io::{Read, Seek, Write},
    path::Path,
};
use walkdir::{DirEntry, WalkDir};

use zip::{result::ZipError, write::SimpleFileOptions};

pub struct ZipUtils {}

impl ZipUtils {
    pub fn zip_dir_to_file(dir: &str, filename: &str) {
        match Self::doit(dir, filename, zip::CompressionMethod::Bzip2) {
            Ok(_) => println!("done: {dir} written to {filename}"),
            Err(e) => println!("Error: {e:?}"),
        }
    }

    fn zip_dir<T>(
        it: &mut dyn Iterator<Item = DirEntry>,
        prefix: &str,
        writer: T,
        method: zip::CompressionMethod,
    ) -> zip::result::ZipResult<()>
    where
        T: Write + Seek,
    {
        let mut zip = zip::ZipWriter::new(writer);
        let options = SimpleFileOptions::default()
            .compression_method(method)
            .unix_permissions(0o755);

        let mut buffer = Vec::new();
        for entry in it {
            let path = entry.path();
            let name = path.strip_prefix(Path::new(prefix)).unwrap();

            if path.is_file() {
                #[allow(deprecated)]
                zip.start_file_from_path(name, options)?;
                let mut f = File::open(path)?;

                f.read_to_end(&mut buffer)?;
                zip.write_all(&buffer)?;
                buffer.clear();
            } else if !name.as_os_str().is_empty() {
                #[allow(deprecated)]
                zip.add_directory_from_path(name, options)?;
            }
        }
        zip.finish()?;
        Result::Ok(())
    }

    fn doit(
        src_dir: &str,
        dst_file: &str,
        method: zip::CompressionMethod,
    ) -> zip::result::ZipResult<()> {
        if !Path::new(src_dir).is_dir() {
            return Err(ZipError::FileNotFound);
        }

        let path = Path::new(dst_file);
        let file = File::create(path).unwrap();

        let walkdir = WalkDir::new(src_dir);
        let it = walkdir.into_iter();

        Self::zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;

        Ok(())
    }
}
