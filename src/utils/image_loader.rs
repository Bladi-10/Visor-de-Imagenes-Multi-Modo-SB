use std::path::{Path, PathBuf};
use rayon::prelude::*;

pub struct ImageLoader;

impl ImageLoader {
    pub fn is_supported_image(path: &Path) -> bool {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()).map(|s| s.to_lowercase()) {
            matches!(
                ext.as_str(),
                "jpg" | "jpeg" | "png" | "bmp" | "webp" | "gif" | "tiff" | "ico"
            )
        } else {
            false
        }
    }

    pub fn scan_directory(dir_path: &Path) -> Vec<PathBuf> {
        let mut entries = Vec::new();
        if let Ok(dir_entries) = std::fs::read_dir(dir_path) {
            let paths: Vec<PathBuf> = dir_entries
                .filter_map(|res| res.ok().map(|e| e.path()))
                .collect();

            entries = paths
                .into_par_iter()
                .filter(|p| p.is_file() && Self::is_supported_image(p))
                .collect();

            entries.sort();
        }
        entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_supported_image() {
        assert!(ImageLoader::is_supported_image(Path::new("foto.jpg")));
        assert!(ImageLoader::is_supported_image(Path::new("imagen.PNG")));
        assert!(ImageLoader::is_supported_image(Path::new("grafico.webp")));
        assert!(!ImageLoader::is_supported_image(Path::new("documento.pdf")));
        assert!(!ImageLoader::is_supported_image(Path::new("script.rs")));
    }
}
