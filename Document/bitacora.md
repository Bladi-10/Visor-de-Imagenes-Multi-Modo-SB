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

### Entrada 8: Creación de Manifiesto .gitignore y Commit Inicial
- **Fecha**: 28 de Julio de 2026
- **Detalles**:
  - Se creó el archivo `.gitignore` configurando el descarte de artefactos de compilación en `/target/` y de la carpeta `Retro/`.
  - Se realizó el commit inicial en el repositorio git con todos los componentes del sistema y la documentación.

### Entrada 9: Refactorización de Interfaz según Retroalimentación Visual (Retro 1.png)
- **Fecha**: 5 de Agosto de 2026
- **Detalles**:
  - Se verificó la omisión en `.gitignore` de la carpeta `Retro/` y artefactos de compilación.
  - Se actualizó el componente `src/components/header.rs` según las observaciones de `Retro/Retro 1.png`.

### Entrada 10: Optimización de Rendimiento y Mejoras de UI (Retro 2.png)
- **Fecha**: 6 de Agosto de 2026
- **Detalles**:
  - Se implementó la ventana deslizante de memoria de 6 imágenes en `src/app/model.rs`.

### Entrada 11: Correcciones de UI, Gestor de Arrastre con Mouse y Zoom Dinámico (Retro 3.png)
- **Fecha**: 6 de Agosto de 2026
- **Detalles**:
  - Se incorporó `gtk::GestureDrag` y zoom dinámico.

### Entrada 12: Atención a Retroalimentación Retro 4.txt
- **Fecha**: 6 de Agosto de 2026
- **Detalles**:
  - **Liberación de RAM**: Ventana de memoria dinámica (3 imágenes en modo Single, hasta 6 en modos múltiples) y desvinculación explícita de `Picture.set_paintable(None)` en `ViewportComponent` antes de eliminar widgets.
  - **Desacoplamiento de Ultra Limpio y Pantalla Completa**: `Ctrl+R` alterará únicamente la interfaz visible sin forzar modo pantalla completa. `F11` gestiona pantalla completa. Ocultamiento automático de botones minimizar/maximizar en la barra flotante de título en pantalla completa.
  - **Papelera Diferida (Staging Trash)**: Implementación de retención temporal en `/tmp/visor_imagenes_trash_staging/`. Al presionar `Delete` la imagen se mueve a la retención. Si se presiona `Ctrl+Z`, se restaura a su ruta original sin dejar duplicados en la papelera del SO. Al cerrar la app se procesa el envío a la papelera del SO (`trash::delete`).
  - **Eliminación de Borrado Permanente**: Eliminación completa de `Ctrl+Delete`, diálogos y funciones asociadas.
  - **Unificación de Apertura por Archivo**: Se eliminó el botón e ícono "Abrir Carpeta". Al abrir una imagen individual, se escanea y carga automáticamente el directorio completo contenedor.
  - **Diálogo Nativo Ágil**: Cambio de filtros MIME a patrones de extensión (`add_pattern("*.jpg")`, etc.) para navegación instantánea sin lectura de disco en el selector de archivos.
  - **Ordenación**: Confirmado y documentado el ordenamiento alfabético por nombre de archivo.
  - **Verificación**: Pasaron con éxito las 9 pruebas unitarias automatizadas (`cargo test`).
