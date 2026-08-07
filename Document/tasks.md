# Registro de Tareas de Desarrollo

## Tareas Completadas

- [x] **Definición de Especificación Técnica**: Creación y actualización de la especificación técnica `./Especificación-Tecnica-Visor-de-Imágenes-Relm4.md` adaptando el modo de 4 imágenes a disposición en fila continua y agregando los atajos numéricos 1, 2, 3 y 4.
- [x] **Plan de Implementación**: Elaboración del plan de arquitectura y módulos en `./plan_implementacion_visor_imagenes.md`.
- [x] **Documentación Técnica Inicial**: Creación de los documentos de arquitectura (`./Document/arquitectura.md`), manual de usuario (`./Document/manual_usuario.md`), registro de tareas (`./Document/tasks.md`) y bitácora de desarrollo (`./Document/bitacora.md`).
- [x] **Configuración de Entorno y Manifiesto**: Configuración de `./Cargo.toml` con dependencias (GTK4 0.9, Libadwaita 0.7, Relm4 0.9, Rayon 1.10, Image 0.25, Kamadak-Exif 0.5, Trash 5.1).
- [x] **Módulos de Utilidades (`./src/utils/`)**:
  - `file_dialog.rs`: Selectores nativos con `gtk::FileChooserNative` y canal asíncrono Tokio.
  - `image_loader.rs`: Escaneo paralelo con Rayon y validación de imágenes.
  - `metadata.rs`: Extracción de metadatos EXIF con `exif` (kamadak-exif) y formato de tamaños.
  - `trash_manager.rs`: Envío a papelera del SO y eliminación permanente.
- [x] **Componentes de Interfaz (`./src/components/`)**:
  - `header.rs`: Barra superior `adw::HeaderBar` con selector 1, 2, 3, 4, controles de zoom y panel lateral.
  - `viewport.rs`: Renderizado en fila horizontal continua de 1 a 4 imágenes con botones flotantes y modo ultra limpio.
  - `sidebar.rs`: Panel de detalles EXIF y propiedades del sistema.
  - `manual_dialog.rs`: Modal interactivo del manual de usuario.
- [x] **Modelo de Estado y Lógica Relm4 (`./src/app/`)**:
  - `model.rs`: Estructura `AppModel` con lógica de navegación circular e índices visibles.
  - `msg.rs`: Definición de la enumeración `AppMsg`.
  - `view.rs`: Integración con `SimpleComponent`, controladores de teclado global y modo ultra limpio.
  - `main.rs`: Punto de entrada con estilos CSS aplicados.
- [x] **Pruebas Unitarias y Compilación**:
  - Ejecución de 8 pruebas unitarias automatizadas (`cargo test`) finalizadas con éxito (8/8 pasadas).
  - Compilación limpia del ejecutable del sistema (`cargo build`).
- [x] **Optimización de UI y Control de Versiones**:
  - Creación y verificación de `.gitignore` excluyendo la carpeta `Retro/` y `/target/`.
  - Refactorización de `header.rs` según las observaciones de `Retro/Retro 1.png`.
- [x] **Carga Eficiente de Imágenes y Ajustes de UI (Retro 2.png)**:
  - Ventana deslizante de memoria de 6 imágenes.
- [x] **Mejoras de UI, Arrastre con Mouse y Controles de Zoom (Retro 3.png)**:
  - Arrastre panning con `gtk::GestureDrag` y zoom dinámico.
- [x] **Implementación de Retroalimentación 4 (`Retro 4.txt`)**:
  - [x] **Liberación de Memoria RAM**: Ventana de precarga dinámica (3 imágenes en modo Single, hasta 6 en modos múltiples) y desvinculación explícita `picture.set_paintable(None)` en `ViewportComponent` antes de desmontar widgets.
  - [x] **Desacoplamiento de Ultra Limpio (Ctrl+R) y Pantalla Completa (F11)**: `Ctrl+R` oculta la UI independientemente de pantalla completa F11. Ocultamiento automático de botones minimizar/maximizar en la barra flotante de título en pantalla completa.
  - [x] **Papelera Diferida (Staging Trash) y Restauración Ctrl+Z**: Mueve imágenes eliminadas a `/tmp/visor_imagenes_trash_staging/`. Al presionar `Ctrl+Z`, las devuelve a su ubicación original sin dejar duplicados en la papelera del SO. Al cerrar la app se completa el envío a la papelera del SO (`trash::delete`).
  - [x] **Remoción de Borrado Permanente (`Ctrl+Delete`)**: Eliminado por completo del código base y atajos.
  - [x] **Unificación de Apertura por Archivo**: Eliminación del botón e ícono "Abrir Carpeta". Al seleccionar una imagen, se escanea y carga automáticamente todo el directorio.
  - [x] **Navegación Ágil en Selector de Archivos**: Cambio a filtro de patrones de extensión (`add_pattern("*.jpg")`, etc.) evitando la lectura lenta de archivos I/O en el diálogo nativo.
  - [x] **Plan de Pruebas Manuales**: Creación del instructivo [`Document/plan_pruebas_manuales.md`](file:///home/bladimir/Documentos/02%20PROYECTOS/06%20Visor%20de%20imagenes/Document/plan_pruebas_manuales.md).
  - [x] **Verificación**: Ejecución y paso del 100% de las 9 pruebas unitarias automatizadas (`cargo test`).

---

## Próximas Tareas (Pendientes)

- [ ] Ninguna tarea pendiente. Todos los requerimientos especificados en Retro 4 han sido implementados y documentados.
