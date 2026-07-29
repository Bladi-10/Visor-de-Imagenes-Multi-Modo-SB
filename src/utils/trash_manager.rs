use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DeletedImageRecord {
    pub original_path: PathBuf,
    pub original_index: usize,
    pub was_trashed: bool,
}

pub struct TrashManager;

impl TrashManager {
    pub fn send_to_trash(path: &Path) -> Result<(), String> {
        trash::delete(path).map_err(|e| format!("Error al mover a la papelera: {}", e))
    }

    pub fn delete_permanently(path: &Path) -> Result<(), String> {
        fs::remove_file(path).map_err(|e| format!("Error al eliminar permanentemente: {}", e))
    }

    pub fn create_record(path: PathBuf, index: usize, trashed: bool) -> DeletedImageRecord {
        DeletedImageRecord {
            original_path: path,
            original_index: index,
            was_trashed: trashed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_record_creation() {
        let path = PathBuf::from("/tmp/test_img.jpg");
        let record = TrashManager::create_record(path.clone(), 2, true);
        assert_eq!(record.original_path, path);
        assert_eq!(record.original_index, 2);
        assert!(record.was_trashed);
    }

    #[test]
    fn test_permanent_delete() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("visor_test_delete_temp.txt");
        
        let mut file = fs::File::create(&test_file).expect("No se pudo crear archivo temporal");
        writeln!(file, "Prueba de borrado permanente").unwrap();
        assert!(test_file.exists());

        let res = TrashManager::delete_permanently(&test_file);
        assert!(res.is_ok());
        assert!(!test_file.exists());
    }
}
