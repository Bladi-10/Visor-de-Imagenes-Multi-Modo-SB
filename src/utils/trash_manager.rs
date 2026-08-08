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
        let base = std::env::var_os("XDG_CACHE_HOME")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache"))
            })
            .unwrap_or_else(std::env::temp_dir);
        let dir = base.join("visor_imagenes_trash_staging");
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

    pub fn has_staged_items() -> bool {
        let staging_dir = Self::staging_dir();
        if let Ok(entries) = fs::read_dir(&staging_dir) {
            entries.flatten().any(|e| e.path().is_file())
        } else {
            false
        }
    }

    pub fn restore_all(records: &[DeletedImageRecord]) -> Result<(), String> {
        let mut errors = Vec::new();
        for record in records {
            if let Err(e) = Self::restore_record(record) {
                errors.push(e);
            } else if !record.original_path.exists() {
                errors.push(format!(
                    "No se pudo verificar la existencia de {:?}",
                    record.original_path
                ));
            }
        }

        if !errors.is_empty() {
            return Err(errors.join("\n"));
        }

        let staging_dir = Self::staging_dir();
        if let Ok(mut entries) = fs::read_dir(&staging_dir) {
            if entries.next().is_none() {
                let _ = fs::remove_dir_all(&staging_dir);
            }
        }
        Ok(())
    }

    pub fn commit_trash_and_verify() -> Result<(), String> {
        let staging_dir = Self::staging_dir();
        let mut delete_errors = Vec::new();

        if let Ok(entries) = fs::read_dir(&staging_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file() {
                    let target_path = p.canonicalize().unwrap_or(p);
                    if let Err(e) = trash::delete(&target_path) {
                        delete_errors.push(format!("Error al enviar {:?} a la papelera: {}", target_path, e));
                    }
                }
            }
        }

        if !delete_errors.is_empty() {
            return Err(delete_errors.join("\n"));
        }

        // Verificar que la carpeta de retención temporal haya quedado limpia
        if let Ok(entries) = fs::read_dir(&staging_dir) {
            if entries.flatten().any(|e| e.path().is_file()) {
                return Err("Archivos pendientes no pudieron ser enviados a la papelera del SO".to_string());
            }
        }

        if let Ok(mut entries) = fs::read_dir(&staging_dir) {
            if entries.next().is_none() {
                let _ = fs::remove_dir_all(&staging_dir);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::sync::Mutex;

    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_staging_trash_and_restore() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
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
        assert!(TrashManager::has_staged_items());

        let restore_res = TrashManager::restore_all(&[record]);
        assert!(restore_res.is_ok());
        assert!(test_file.exists());

        let _ = fs::remove_file(&test_file);
    }

    #[test]
    fn test_commit_trash_and_verify() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let base_dir = std::env::var_os("HOME").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
        let test_file = base_dir.join("visor_test_commit_orig.txt");

        let mut file = fs::File::create(&test_file).expect("No se pudo crear archivo temporal");
        writeln!(file, "Prueba de envio a la papelera del SO").unwrap();
        assert!(test_file.exists());

        let record_res = TrashManager::send_to_trash_staging(&test_file, 0);
        assert!(record_res.is_ok());
        assert!(TrashManager::has_staged_items());

        let commit_res = TrashManager::commit_trash_and_verify();
        if let Err(ref e) = commit_res {
            panic!("commit_trash_and_verify falló con error: {}", e);
        }
        assert!(commit_res.is_ok());
        assert!(!TrashManager::has_staged_items());
    }
}



