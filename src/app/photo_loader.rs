use std::path::PathBuf;
use walkdir::WalkDir;
use std::fs;

#[derive(Clone, Debug)]
pub struct Photo {
    pub path: PathBuf,
    pub name: String,
}

pub async fn load_photos() -> Vec<Photo> {
    let mut photos = Vec::new();
    let pictures_dir = fs::canonicalize(dirs::picture_dir().unwrap_or_else(|| PathBuf::from("~/Pictures"))).unwrap();

    for entry in WalkDir::new(pictures_dir).max_depth(3) {
        if let Ok(entry) = entry {
            let path = entry.path().to_path_buf();

            if path.is_file() && is_image(&path) {
                let name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                photos.push(Photo { path, name });
            }
        }
    }

    // Sort photos by name for consistent ordering
    photos.sort_by(|a, b| a.name.cmp(&b.name));
    photos
}

pub fn is_image(path: &PathBuf) -> bool {
    let extension = path.extension().and_then(|e| e.to_str());
    matches!(
        extension,
        Some("jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "webp" | "JPG" | "JPEG" | "PNG")
    )
}
