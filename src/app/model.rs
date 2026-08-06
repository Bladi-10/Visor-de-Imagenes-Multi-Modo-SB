use std::path::PathBuf;
use gtk::gdk;
use crate::utils::metadata::{extract_metadata, ImageMetadata};
use crate::utils::trash_manager::{DeletedImageRecord, TrashManager};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Single = 1,
    Dual = 2,
    Triple = 3,
    Quad = 4,
}

impl ViewMode {
    pub fn count(&self) -> usize {
        *self as usize
    }
}

#[derive(Debug, Clone)]
pub struct ImageItem {
    pub path: PathBuf,
    pub texture: Option<gdk::Texture>,
    pub metadata: Option<ImageMetadata>,
}

impl ImageItem {
    #[allow(dead_code)]
    pub fn new(path: PathBuf) -> Self {
        let meta = extract_metadata(&path);
        let texture = gdk::Texture::from_filename(&path).ok();
        Self {
            path,
            texture,
            metadata: Some(meta),
        }
    }

    pub fn new_lazy(path: PathBuf) -> Self {
        Self {
            path,
            texture: None,
            metadata: None,
        }
    }

    pub fn load_texture(&mut self) {
        if self.texture.is_none() {
            self.texture = gdk::Texture::from_filename(&self.path).ok();
        }
        if self.metadata.is_none() {
            self.metadata = Some(extract_metadata(&self.path));
        }
    }

    pub fn unload_texture(&mut self) {
        self.texture = None;
    }
}

pub struct AppModel {
    pub images: Vec<ImageItem>,
    pub current_index: usize,
    pub view_mode: ViewMode,
    pub zoom_level: f64,
    pub is_fit_mode: bool,
    pub is_fullscreen: bool,
    pub is_clean_ui: bool,
    pub is_sidebar_open: bool,
    pub undo_stack: Vec<DeletedImageRecord>,
}

impl Default for AppModel {
    fn default() -> Self {
        Self {
            images: Vec::new(),
            current_index: 0,
            view_mode: ViewMode::Single,
            zoom_level: 1.0,
            is_fit_mode: true,
            is_fullscreen: false,
            is_clean_ui: false,
            is_sidebar_open: false,
            undo_stack: Vec::new(),
        }
    }
}

impl AppModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_window_indices(&self) -> Vec<usize> {
        let n = self.images.len();
        if n == 0 {
            return Vec::new();
        }
        if n <= 6 {
            return (0..n).collect();
        }

        let mut indices = Vec::with_capacity(6);
        for offset in -1isize..=4isize {
            let idx = (self.current_index as isize + offset).rem_euclid(n as isize) as usize;
            if !indices.contains(&idx) {
                indices.push(idx);
            }
        }
        indices
    }

    pub fn update_loaded_window(&mut self) {
        if self.images.is_empty() {
            return;
        }
        let window_indices = self.get_window_indices();
        for (i, item) in self.images.iter_mut().enumerate() {
            if window_indices.contains(&i) {
                item.load_texture();
            } else {
                item.unload_texture();
            }
        }
    }

    pub fn next_image(&mut self) {
        if self.images.is_empty() {
            return;
        }
        self.current_index = (self.current_index + 1) % self.images.len();
        self.update_loaded_window();
    }

    pub fn prev_image(&mut self) {
        if self.images.is_empty() {
            return;
        }
        if self.current_index == 0 {
            self.current_index = self.images.len() - 1;
        } else {
            self.current_index -= 1;
        }
        self.update_loaded_window();
    }

    pub fn visible_indices(&self) -> Vec<usize> {
        if self.images.is_empty() {
            return Vec::new();
        }
        let count = self.view_mode.count();
        let mut indices = Vec::with_capacity(count);
        for i in 0..count {
            let idx = (self.current_index + i) % self.images.len();
            indices.push(idx);
        }
        indices
    }

    pub fn active_image(&self) -> Option<&ImageItem> {
        self.images.get(self.current_index)
    }

    pub fn remove_current_and_trash(&mut self) -> Option<DeletedImageRecord> {
        if self.images.is_empty() {
            return None;
        }

        let item = self.images.remove(self.current_index);
        let trashed = TrashManager::send_to_trash(&item.path).is_ok();
        let record = TrashManager::create_record(item.path, self.current_index, trashed);

        if self.current_index >= self.images.len() && !self.images.is_empty() {
            self.current_index = self.images.len() - 1;
        }

        self.update_loaded_window();
        self.undo_stack.push(record.clone());
        Some(record)
    }

    pub fn remove_current_permanently(&mut self) -> Option<PathBuf> {
        if self.images.is_empty() {
            return None;
        }

        let item = self.images.remove(self.current_index);
        let _ = TrashManager::delete_permanently(&item.path);

        if self.current_index >= self.images.len() && !self.images.is_empty() {
            self.current_index = self.images.len() - 1;
        }

        self.update_loaded_window();
        Some(item.path)
    }

    pub fn undo_last_delete(&mut self) -> bool {
        if let Some(record) = self.undo_stack.pop() {
            let restored_item = ImageItem::new_lazy(record.original_path);
            let target_index = record.original_index.min(self.images.len());
            self.images.insert(target_index, restored_item);
            self.current_index = target_index;
            self.update_loaded_window();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_mode_count() {
        assert_eq!(ViewMode::Single.count(), 1);
        assert_eq!(ViewMode::Dual.count(), 2);
        assert_eq!(ViewMode::Triple.count(), 3);
        assert_eq!(ViewMode::Quad.count(), 4);
    }

    #[test]
    fn test_circular_navigation() {
        let mut model = AppModel::new();
        model.images = vec![
            ImageItem { path: PathBuf::from("a.jpg"), texture: None, metadata: None },
            ImageItem { path: PathBuf::from("b.jpg"), texture: None, metadata: None },
            ImageItem { path: PathBuf::from("c.jpg"), texture: None, metadata: None },
        ];

        assert_eq!(model.current_index, 0);
        model.next_image();
        assert_eq!(model.current_index, 1);
        model.next_image();
        assert_eq!(model.current_index, 2);
        model.next_image();
        assert_eq!(model.current_index, 0);

        model.prev_image();
        assert_eq!(model.current_index, 2);
    }

    #[test]
    fn test_visible_indices_row_layout() {
        let mut model = AppModel::new();
        model.images = vec![
            ImageItem { path: PathBuf::from("1.jpg"), texture: None, metadata: None },
            ImageItem { path: PathBuf::from("2.jpg"), texture: None, metadata: None },
            ImageItem { path: PathBuf::from("3.jpg"), texture: None, metadata: None },
            ImageItem { path: PathBuf::from("4.jpg"), texture: None, metadata: None },
            ImageItem { path: PathBuf::from("5.jpg"), texture: None, metadata: None },
        ];

        model.current_index = 3;
        model.view_mode = ViewMode::Quad;
        assert_eq!(model.visible_indices(), vec![3, 4, 0, 1]);
    }

    #[test]
    fn test_window_indices_6_images() {
        let mut model = AppModel::new();
        for i in 0..10 {
            model.images.push(ImageItem::new_lazy(PathBuf::from(format!("{}.jpg", i))));
        }

        // current_index = 0 -> -1(9), 0(0), +1(1), +2(2), +3(3), +4(4)
        model.current_index = 0;
        assert_eq!(model.get_window_indices(), vec![9, 0, 1, 2, 3, 4]);

        // current_index = 5 -> -1(4), 0(5), +1(6), +2(7), +3(8), +4(9)
        model.current_index = 5;
        assert_eq!(model.get_window_indices(), vec![4, 5, 6, 7, 8, 9]);

        // current_index = 9 -> -1(8), 0(9), +1(0), +2(1), +3(2), +4(3)
        model.current_index = 9;
        assert_eq!(model.get_window_indices(), vec![8, 9, 0, 1, 2, 3]);
    }
}
