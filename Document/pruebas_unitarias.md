# Reporte de Pruebas Unitarias - Visor de Imágenes

## 1. Resumen de Ejecución de Pruebas

- **Herramienta de Pruebas**: Cargo Test Framework (`cargo test`)
- **Fecha de Ejecución**: 28 de Julio de 2026
- **Resultado General**: 8 pasadas / 0 fallidas / 0 ignoradas

```
running 8 tests
test app::model::tests::test_circular_navigation ... ok
test utils::metadata::tests::test_format_size ... ok
test app::model::tests::test_visible_indices_row_layout ... ok
test utils::image_loader::tests::test_is_supported_image ... ok
test app::model::tests::test_view_mode_count ... ok
test utils::metadata::tests::test_mime_type_detection ... ok
test utils::trash_manager::tests::test_permanent_delete ... ok
test utils::trash_manager::tests::test_record_creation ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

## 2. Detalle de Casos de Prueba Evaluados

### 2.1 Navegación Circular (`test_circular_navigation`)
- **Ubicación**: `./src/app/model.rs`
- **Objetivo**: Verificar que al avanzar más allá de la última imagen del catálogo, el índice retorna al primer elemento (0), y al retroceder desde el elemento 0, retorna al último elemento.
- **Estado**: Exitoso.

### 2.2 Índices Visibles en Fila (`test_visible_indices_row_layout`)
- **Ubicación**: `./src/app/model.rs`
- **Objetivo**: Confirmar que en modo Quad (4 imágenes), la lista de índices visibles calcula adecuadamente la secuencia continua de 4 imágenes dispuestas en fila.
- **Estado**: Exitoso.

### 2.3 Cantidad por Modo de Vista (`test_view_mode_count`)
- **Ubicación**: `./src/app/model.rs`
- **Objetivo**: Validar los métodos `count()` de los enumeradores `ViewMode` (Single = 1, Dual = 2, Triple = 3, Quad = 4).
- **Estado**: Exitoso.

### 2.4 Detección de Formatos Soportados (`test_is_supported_image`)
- **Ubicación**: `./src/utils/image_loader.rs`
- **Objetivo**: Garantizar el filtrado adecuado de extensiones válidas de imágenes (jpg, png, webp, bmp, gif) y rechazo de archivos no soportados.
- **Estado**: Exitoso.

### 2.5 Formateo de Tamaños de Archivo (`test_format_size`)
- **Ubicación**: `./src/utils/metadata.rs`
- **Objetivo**: Verificar la conversión correcta de bytes a cadenas legibles (B, KB, MB).
- **Estado**: Exitoso.

### 2.6 Detección de Tipos MIME (`test_mime_type_detection`)
- **Ubicación**: `./src/utils/metadata.rs`
- **Objetivo**: Validar la asignación de MIME types según la extensión del archivo.
- **Estado**: Exitoso.

### 2.7 Eliminación Permanente de Archivos (`test_permanent_delete`)
- **Ubicación**: `./src/utils/trash_manager.rs`
- **Objetivo**: Verificar que el borrado permanente elimina físicamente el archivo del disco.
- **Estado**: Exitoso.

### 2.8 Registro de Historial de Borrado (`test_record_creation`)
- **Ubicación**: `./src/utils/trash_manager.rs`
- **Objetivo**: Validar la creación adecuada del registro de deshacer conteniendo ruta original e índice.
- **Estado**: Exitoso.
