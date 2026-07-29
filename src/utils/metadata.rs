use std::fs;
use std::path::Path;
use exif::{In, Reader, Tag, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct ImageMetadata {
    pub file_name: String,
    pub file_path: String,
    pub dimensions: (u32, u32),
    pub size_bytes: u64,
    pub mime_type: String,
    pub camera_model: Option<String>,
    pub iso: Option<u32>,
    pub f_number: Option<f32>,
}

pub fn extract_metadata(path: &Path) -> ImageMetadata {
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Desconocido")
        .to_string();

    let file_path = path.to_string_lossy().to_string();

    let size_bytes = fs::metadata(path).map(|m| m.len()).unwrap_or(0);

    let mime_type = match path.extension().and_then(|ext| ext.to_str()).map(|e| e.to_lowercase()).as_deref() {
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("bmp") => "image/bmp",
        _ => "image/unknown",
    }
    .to_string();

    let dimensions = image::image_dimensions(path).unwrap_or((0, 0));

    let mut camera_model = None;
    let mut iso = None;
    let mut f_number = None;

    if let Ok(file) = fs::File::open(path) {
        let mut bufreader = std::io::BufReader::new(file);
        let exif_reader = Reader::new();
        if let Ok(exif_data) = exif_reader.read_from_container(&mut bufreader) {
            if let Some(field) = exif_data.get_field(Tag::Model, In::PRIMARY) {
                camera_model = Some(field.display_value().with_unit(&exif_data).to_string().trim_matches('"').to_string());
            }
            if let Some(field) = exif_data.get_field(Tag::PhotographicSensitivity, In::PRIMARY) {
                if let Some(val) = field.value.get_uint(0) {
                    iso = Some(val);
                }
            }
            if let Some(field) = exif_data.get_field(Tag::FNumber, In::PRIMARY) {
                if let Value::Rational(ref rationals) = field.value {
                    if let Some(r) = rationals.first() {
                        if r.denom != 0 {
                            f_number = Some(r.num as f32 / r.denom as f32);
                        }
                    }
                }
            }
        }
    }

    ImageMetadata {
        file_name,
        file_path,
        dimensions,
        size_bytes,
        mime_type,
        camera_model,
        iso,
        f_number,
    }
}

pub fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(2048), "2.00 KB");
        assert_eq!(format_size(1048576), "1.00 MB");
    }

    #[test]
    fn test_mime_type_detection() {
        let meta = extract_metadata(Path::new("test_image.jpg"));
        assert_eq!(meta.mime_type, "image/jpeg");

        let meta_png = extract_metadata(Path::new("test_image.png"));
        assert_eq!(meta_png.mime_type, "image/png");
    }
}
