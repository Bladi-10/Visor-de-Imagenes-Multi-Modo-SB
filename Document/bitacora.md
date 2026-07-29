# Bitácora de Desarrollo - Visor de Imágenes Relm4

## Registro de Eventos y Decisiones de Diseño

### Entrada 1: Definición de la Especificación Técnica Inicial
- **Fecha**: 28 de Julio de 2026
- **Detalles**:
  - Se analizó la especificación técnica general (`./Especificación-Tecnica-Visor-de-Imágenes-Relm4.md`) para la construcción del visor de imágenes con Rust, GTK4, Libadwaita y Relm4.
  - Se verificó la disponibilidad de herramientas y dependencias en el sistema objetivo (Linux Mint): Rustc 1.94.1, Cargo 1.94.1, GTK4 4.14.5, Libadwaita 1.5.0.

### Entrada 2: Ajuste en la Disposición del Modo de 4 Imágenes
- **Fecha**: 28 de Julio de 2026
- **Detalles**:
  - Por requerimiento explícito del usuario, se modificó la especificación del modo de 4 imágenes para sustituir la distribución en retícula/cuadrícula por una disposición horizontal en una sola fila continua.
  - Se actualizó la especificación técnica en `./Especificación-Tecnica-Visor-de-Imágenes-Relm4.md` reflejando la disposición en fila en el esquema ASCII de la interfaz y en los requerimientos funcionales.

### Entrada 3: Estructuración de la Documentación del Proyecto
- **Fecha**: 28 de Julio de 2026
- **Detalles**:
  - Se creó la carpeta de documentación `./Document/` con los archivos requeridos:
    - `./Document/manual_usuario.md`: Guía de atajos de teclado y funcionamiento del visor.
    - `./Document/arquitectura.md`: Explicación del modelo MVU, arquitectura de módulos y mapa de código fuente.
    - `./Document/tasks.md`: Lista de verificación de tareas completadas y pendientes.
    - `./Document/bitacora.md`: Historial cronológico de cambios y decisiones arquitectónicas.
  - Se aplicaron las reglas de estilo del proyecto (`./.gemini/rules/reglas_ia.md`): comunicación en español, ausencia total de emoticones y uso exclusivo de rutas relativas.

### Entrada 4: Inclusión de Atajos Numéricos para Cambio de Modo Multi-Imagen
- **Fecha**: 28 de Julio de 2026
- **Detalles**:
  - Se incorporaron las teclas numéricas `1`, `2`, `3` y `4` a la matriz de atajos de teclado para alternar directamente entre los modos de visualización de 1, 2, 3 y 4 imágenes en fila.
  - Se actualizaron la especificación técnica en `./Especificación-Tecnica-Visor-de-Imágenes-Relm4.md` y el manual de usuario en `./Document/manual_usuario.md`.

### Entrada 5: Implementación Completa de Código Fuente
- **Fecha**: 28 de Julio de 2026
- **Detalles**:
  - Se implementó todo el árbol de módulos del proyecto (`./src/main.rs`, `./src/app/`, `./src/components/`, `./src/utils/`).
  - Se resolvió la integración entre Relm4, GTK4, Libadwaita y Rayon para decodificación y renderizado en fila de 1 a 4 imágenes.

### Entrada 6: Ejecución de Pruebas Unitarias y Compilación Final
- **Fecha**: 28 de Julio de 2026
- **Detalles**:
  - Se ejecutó la suite de pruebas unitarias (`cargo test`), obteniendo 8 pruebas pasadas con 0 errores.
  - Se generó el documento de reporte de pruebas unitarias en `./Document/pruebas_unitarias.md`.
  - Se ejecutó la compilación final del ejecutable binario del sistema (`cargo build`) saliendo con código 0 de éxito.

### Entrada 7: Corrección de Integración de AdwApplicationWindow y Diálogo Modal
- **Fecha**: 28 de Julio de 2026
- **Detalles**:
  - Se resolvió el error en tiempo de ejecución de Libadwaita (`gtk_window_set_titlebar() is not supported for AdwApplicationWindow`) reestructurando el contenedor raíz en un `gtk::Box` vertical que empaqueta `adw::HeaderBar` en la parte superior y el contenedor de contenido en la parte inferior.
  - Se implementó el diálogo modal de confirmación de borrado permanente con `gtk::MessageDialog` y botones de acción Destructiva/Cancelar.
  - Se conectaron los controladores de botones de minimizar (`win.minimize()`) y maximizar/restaurar (`win.maximize()` / `win.unmaximize()`).
  - Se limpiaron todas las advertencias de código no utilizado mediante directivas `#[allow(dead_code)]`. Compilación y pruebas de nuevo verificadas con 0 advertencias y 0 errores.
