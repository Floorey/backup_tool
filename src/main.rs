use std::fs::File;
use std::io::{self, Write, BufWriter};
use walkdir::{WalkDir, DirEntry};
use zip::write::{FileOptions, ZipWriter};
use std::path::Path;
use std::ffi::OsStr;
use zip::CompressionMethod;

fn main() -> io::Result<()> {
    let source_dir = Path::new("/home/lenderle/Dokumente");
    let target_file = File::create("/run/media/lenderle/eaf0c453-897a-4f14-bd51-8028ab3b65e2/backup.zip")?;
    let writer = BufWriter::new(target_file);
    
    let walkdir = WalkDir::new(&source_dir);
    let mut zip = ZipWriter::new(writer);

    let options = FileOptions::<()>::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o755);

    for entry in walkdir.into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() && should_backup(path) {
            let name = path.strip_prefix(Path::new(&source_dir))
                .unwrap()
                .to_str()
                .unwrap();
            zip.start_file(name, options)?;
            let mut f = File::open(path)?;
            io::copy(&mut f, &mut zip)?;
        } else if path.is_dir() {
            let name = path.strip_prefix(Path::new(&source_dir))
                .unwrap()
                .to_str()
                .unwrap();
            zip.add_directory(name, options)?;
        }
    }
    
    zip.finish()?;
    println!("Backup created successfully");
    
    Ok(())
}

fn should_backup(path: &Path) -> bool {
    if path.is_dir() {
        return true;
    }
    match path.extension().and_then(OsStr::to_str) {
        Some("txt") | Some("pdf") => true,
        _ => false,
    }
}
