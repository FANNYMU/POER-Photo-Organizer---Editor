use std::path::PathBuf;
use image;
use walkdir::WalkDir;
use std::fs::metadata;

#[derive(Clone, Debug)]
pub struct Photo {
    pub path: PathBuf,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub size: u64,
}

pub async fn load_photos() -> Vec<Photo> {
    let mut photos = Vec::new();
    if let Some(pictures_dir) = dirs::picture_dir() {
        let mut entries = WalkDir::new(pictures_dir)
            .into_iter()
            .filter_map(|e| e.ok());
        
        while let Some(entry) = entries.next() {
            let path = entry.path().to_path_buf();
            if path.extension().and_then(|e| e.to_str()).map_or(false, |ext| {
                ["jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp"].contains(&ext.to_lowercase().as_str())
            }) {
                if let Ok(img) = image::open(&path) {
                    let (width, height) = (img.width(), img.height());
                    let size = match metadata(&path) {
                        Ok(metadata) => metadata.len(),
                        Err(_) => 0,
                    };
                    let name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or_default()
                        .to_string();
                    
                    photos.push(Photo {
                        path,
                        name,
                        width,
                        height,
                        size,
                    });
                }
            }
        }
    }
    photos
}

