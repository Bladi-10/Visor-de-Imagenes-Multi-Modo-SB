use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct DeletedImageRecord {
    pub original_path: PathBuf,
    pub staged_path: PathBuf,
    pub original_index: usize,
}

pub struct TrashManager;

impl TrashManager {
    fn staging_dir() -> PathBuf {
        let dir = std::env::temp_dir().join("visor_imagenes_trash_staging");
        let _ = fs::create_dir_all(&dir);
        dir
    }

    pub fn send_to_trash_staging(path: &Path, index: usize) -> Result<DeletedImageRecord, String> {
        let staging_dir = Self::staging_dir();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);

        let filename = path.file_name().unwrap_or_default().to_string_lossy();
        let staged_name = format!("{}_{}", timestamp, filename);
        let staged_path = staging_dir.join(staged_name);

        fs::rename(path, &staged_path)
            .or_else(|_| {
                fs::copy(path, &staged_path).and_then(|_| fs::remove_file(path))
            })
            .map_err(|e| format!("Error al mover a papelera de retención: {}", e))?;

        Ok(DeletedImageRecord {
            original_path: path.to_path_buf(),
            staged_path,
            original_index: index,
        })
    }

    pub fn restore_record(record: &DeletedImageRecord) -> Result<(), String> {
        if record.staged_path.exists() {
            if let Some(parent) = record.original_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            fs::rename(&record.staged_path, &record.original_path)
                .or_else(|_| {
                    fs::copy(&record.staged_path, &record.original_path)
                        .and_then(|_| fs::remove_file(&record.staged_path))
                })
                .map_err(|e| format!("Error al restaurar archivo: {}", e))?;
        }
        Ok(())
    }

    pub fn commit_trash() {
        let staging_dir = Self::staging_dir();
        if let Ok(entries) = fs::read_dir(&staging_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file() {
                    let _ = trash::delete(&p);
                }
            }
        }
        let _ = fs::remove_dir_all(&staging_dir);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_staging_trash_and_restore() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("visor_test_staging_orig.txt");

        let mut file = fs::File::create(&test_file).expect("No se pudo crear archivo temporal");
        writeln!(file, "Prueba de retencion de papelera").unwrap();
        assert!(test_file.exists());

        let record_res = TrashManager::send_to_trash_staging(&test_file, 0);
        assert!(record_res.is_ok());
        let record = record_res.unwrap();
        assert!(!test_file.exists());
        assert!(record.staged_path.exists());

        let restore_res = TrashManager::restore_record(&record);
        assert!(restore_res.is_ok());
        assert!(test_file.exists());
        assert!(!record.staged_path.exists());

        let _ = fs::remove_file(&test_file);
    }
}


