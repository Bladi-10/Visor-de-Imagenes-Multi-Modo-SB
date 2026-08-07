# Reporte de Pruebas Unitarias - Visor de Imágenes

## 1. Resumen de Ejecución de Pruebas

- **Herramienta de Pruebas**: Cargo Test Framework (`cargo test`)
- **Fecha de Ejecución**: 7 de Agosto de 2026
- **Resultado General**: 10 pasadas / 0 fallidas / 0 ignoradas

```
running 10 tests
test app::model::tests::test_view_mode_count ... ok
test app::model::tests::test_window_indices_quad_mode ... ok
test utils::image_loader::tests::test_is_supported_image ... ok
test app::model::tests::test_visible_indices_row_layout ... ok
test utils::metadata::tests::test_format_size ... ok
test app::model::tests::test_window_indices_single_mode ... ok
test utils::metadata::tests::test_mime_type_detection ... ok
test utils::trash_manager::tests::test_commit_trash_and_verify ... ok
test utils::trash_manager::tests::test_staging_trash_and_restore ... ok
test app::model::tests::test_circular_navigation ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
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

### 2.4 Ventana de Memoria en Modo Single (`test_window_indices_single_mode`)
- **Ubicación**: `./src/app/model.rs`
- **Objetivo**: Garantizar que en modo Single solo se mantienen cargados 3 elementos en la ventana de memoria (anterior, actual, siguiente).
- **Estado**: Exitoso.

### 2.5 Ventana de Memoria en Modo Multi-Vista (`test_window_indices_quad_mode`)
- **Ubicación**: `./src/app/model.rs`
- **Objetivo**: Validar que en modos múltiples (Quad) la ventana de precarga sostiene hasta 6 imágenes para fluidez inmediata.
- **Estado**: Exitoso.

### 2.6 Retención y Restauración de Papelera (`test_staging_trash_and_restore`)
- **Ubicación**: `./src/utils/trash_manager.rs`
- **Objetivo**: Validar la retención en carpeta temporal al eliminar y la posterior restitución idéntica a su ruta original con verificación de existencia en disco.
- **Estado**: Exitoso.

### 2.7 Envío Verificado a Papelera del SO (`test_commit_trash_and_verify`)
- **Ubicación**: `./src/utils/trash_manager.rs`
- **Objetivo**: Validar la transferencia de archivos de retención a la papelera del SO (`trash::delete`), verificando que no queden remanentes en el directorio temporal.
- **Estado**: Exitoso.

### 2.8 Detección de Formatos Soportados (`test_is_supported_image`)
- **Ubicación**: `./src/utils/image_loader.rs`
- **Objetivo**: Garantizar el filtrado adecuado de extensiones válidas de imágenes (jpg, png, webp, bmp, gif, tiff, ico).
- **Estado**: Exitoso.

### 2.9 Formateo de Tamaños de Archivo (`test_format_size`)
- **Ubicación**: `./src/utils/metadata.rs`
- **Objetivo**: Verificar la conversión correcta de bytes a cadenas legibles (B, KB, MB).
- **Estado**: Exitoso.

### 2.10 Detección de Tipos MIME (`test_mime_type_detection`)
- **Ubicación**: `./src/utils/metadata.rs`
- **Objetivo**: Validar la asignación de MIME types según la extensión del archivo.
- **Estado**: Exitoso.

