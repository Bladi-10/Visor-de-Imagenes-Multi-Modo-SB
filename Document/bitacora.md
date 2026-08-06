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
  - Se actualizó el componente `src/components/header.rs` según las observaciones de `Retro/Retro 1.png`:
    - Reemplazo del icono del botón de información por el icono simbólico monocromático `dialog-information-symbolic` con estilo `flat`.
    - Eliminación de la etiqueta de texto central ("Imagen X de Y") dejando únicamente centrado el selector de modos de vista.
    - Sustitución de las etiquetas de texto de los botones "1:1" y "Fit" por iconos simbólicos `zoom-original-symbolic` y `zoom-fit-best-symbolic`, eliminando el fondo y la sombra cuadrada con la clase CSS `flat`.
    - Eliminación de los botones manuales de minimizar y maximizar de la barra superior para evitar duplicidad con los botones de control nativos de Libadwaita.
  - Se ejecutaron las pruebas unitarias automatizadas (`cargo test`), aprobando los 8 test del sistema sin errores.

### Entrada 10: Optimización de Rendimiento (Ventana de 6 Imágenes) y Mejoras de UI (Retro 2.png)
- **Fecha**: 6 de Agosto de 2026
- **Detalles**:
  - Se implementó la ventana deslizante de memoria de 6 imágenes (`new_lazy`, `load_texture`, `unload_texture`, `get_window_indices` y `update_loaded_window`) en `src/app/model.rs` y `src/app/view.rs`.
  - Al abrir carpetas masivas de 1,000+ imágenes, solo se mantienen cargadas en memoria RAM 6 imágenes (1 atrás, la actual y 4 adelante). Las texturas fuera de ese rango se descargan automáticamente, manteniendo nulo el impacto en RAM/Swap.
  - Se agregó el botón con icono nativo `dark-mode-symbolic` para alternar entre el modo oscuro y claro de Libadwaita.
  - Se trasladó el botón de Pantalla Completa al bloque derecho de la barra de herramientas y se actualizó su atajo global a `Alt+F11`.
  - Se implementó el menú flotante en pantalla completa que aparece cuando el cursor se sitúa cerca del límite superior de la pantalla (`y <= 25px`).
  - Se eliminó el marco `gtk::Frame` alrededor de las imágenes en `src/components/viewport.rs`, descartando el borde blanco rectangular.
  - Se añadió la regla CSS para eliminar la línea negra inferior divisoria de la barra de título en `src/main.rs`.
  - Se añadieron pruebas unitarias automatizadas (`test_window_indices_6_images`), alcanzando 9/9 tests pasados con éxito.

### Entrada 11: Correcciones de UI, Gestor de Arrastre con Mouse y Zoom Dinámico (Retro 3.png)
- **Fecha**: 6 de Agosto de 2026
- **Detalles**:
  - Se solucionó el icono roto de modo oscuro en `src/components/header.rs` empleando el nombre estándar `weather-clear-night-symbolic`.
  - Se corrigió el formateo Pango en `src/components/manual_dialog.rs` escapando los caracteres `<` y `>` (`&lt;` y `&gt;`), eliminando etiquetas HTML literales visibles.
  - Se agruparon los botones de zoom en `box_zoom` dentro de `src/components/header.rs`, configurando su visibilidad dinámica para mostrarse en el modo de 1 imagen y ocultarse automáticamente en los modos múltiples (2, 3, 4 imágenes).
  - Se mejoró la lógica de Zoom Out (`-`) en `src/app/view.rs` para permitir reducciones escalonadas por debajo del tamaño de ajuste/UI inicial.
  - Se incorporó `gtk::GestureDrag` en `src/components/viewport.rs` permitiendo desplazamiento panning al arrastrar la imagen con clic izquierdo.
  - Se restauró el atajo de pantalla completa a `F11` estándar.
  - Se ejecutaron las 9 pruebas unitarias automatizadas del sistema (`cargo test`) con resultado exitoso.




