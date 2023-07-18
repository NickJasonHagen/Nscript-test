use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use zip::{write::FileOptions, ZipWriter};


// Function to zip a directory
pub fn zip_directory(directory: &str, zip_file: &str) -> String {
    match do_zip_directory(directory, zip_file) {
        Ok(_) => "Directory zipped successfully.".to_string(),
        Err(err) => format!("Error zipping directory: {:?}", err),
    }
}

// Helper function to perform the zip operation
fn do_zip_directory(directory: &str, zip_file: &str) -> io::Result<()> {
    let zip_file = File::create(zip_file)?;
    let mut zip = ZipWriter::new(zip_file);

    add_directory_to_zip(directory, &mut zip, directory)?;

    zip.finish()?;
    Ok(())
}

// Helper function to recursively add files and directories to the zip archive
fn add_directory_to_zip(
    directory: &str,
    zip: &mut ZipWriter<File>,
    root: &str,
) -> io::Result<()> {
    let directory = Path::new(directory);
    let root = Path::new(root);

    for entry in directory.read_dir()? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            add_directory_to_zip(&path.to_string_lossy(), zip, root.to_string_lossy().as_ref())?;
        } else {
            let file_path = path.strip_prefix(root).unwrap();
            let options = FileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated)
                .unix_permissions(0o755); // Set permissions as needed

            zip.start_file(file_path.to_string_lossy().into_owned(), options)?;
            let mut file = File::open(&path)?;
            io::copy(&mut file, zip)?;
        }
    }

    Ok(())
}

// Function to unzip a zip file
pub fn unzip_file(zip_file: &str, target_directory: &str) -> String {
    match do_unzip_file(zip_file, target_directory) {
        Ok(_) => "File unzipped successfully.".to_string(),
        Err(err) => format!("Error unzipping file: {:?}", err),
    }
}

// Helper function to perform the unzip operation
fn do_unzip_file(zip_file: &str, target_directory: &str) -> io::Result<()> {
    let file = File::open(zip_file)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = PathBuf::from(target_directory).join(file.sanitized_name());

        if let Some(parent) = outpath.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        if file.is_dir() {
            std::fs::create_dir_all(&outpath)?;
        } else {
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}
