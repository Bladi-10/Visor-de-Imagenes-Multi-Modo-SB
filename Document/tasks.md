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
  - Refactorización de `header.rs` según las observaciones de `Retro/Retro 1.png`: homogenización de icono Info, eliminación de etiqueta redundante de estado, conversión de botones 1:1 y Fit a iconos simbólicos planos (`flat`) y remoción de controles de ventana duplicados.
- [x] **Carga Eficiente de Imágenes y Ajustes de UI (Retro 2.png)**:
  - Implementación de la ventana deslizante de memoria de 6 imágenes (1 atrás, actual, 4 adelante) para evitar sobrecarga de RAM/Swap al abrir carpetas de 1,000+ elementos.
  - Integración del botón de Modo Oscuro/Claro con `dark-mode-symbolic`.
  - Reubicación del botón de Pantalla Completa a la derecha y actualización de atajo a `Alt+F11` con barra flotante hover en la parte superior.
  - Eliminación de la línea negra inferior del `headerbar` mediante CSS y del marco `gtk::Frame` con borde blanco rectangular alrededor de las vistas previas.
  - Verificación mediante 9 pruebas unitarias automatizadas (`cargo test`).
- [x] **Mejoras de UI, Arrastre con Mouse y Controles de Zoom (Retro 3.png)**:
  - Reemplazo de icono roto por el nombre estándar `weather-clear-night-symbolic`.
  - Escape de caracteres Pango (`&lt;` y `&gt;`) en `manual_dialog.rs` para renderizar negritas sin etiquetas HTML literales visibles.
  - Ocultamiento automático del grupo de zoom (`box_zoom`) en modos de vista múltiples (2, 3, 4 imágenes).
  - Integración de `gtk::GestureDrag` para panning y desplazamiento con clic izquierdo mantenido.
  - Ajuste de escalado en Zoom Out (`-`) para permitir reducir la imagen por debajo del tamaño de la UI inicial.
  - Restauración del atajo de Pantalla Completa a `F11` estándar.
- [x] **Actualización de Atajo de Teclado**:
  - Cambio del atajo global del Modo Ultra Limpio de `Ctrl + F11` a `Ctrl + R` en `src/app/view.rs`.
  - Actualización del manual interactivo en `src/components/manual_dialog.rs` y la documentación del usuario en `Document/manual_usuario.md`.

---

## Próximas Tareas (Pendientes)

- [ ] **Configuración del Comportamiento del Modo Ultra Limpio**:
  - Revisar y configurar adecuadamente la transición, elementos ocultos y experiencia general de uso del Modo Ultra Limpio (`Ctrl + R`).
- [ ] **Ajuste del Ancho de la Barra Lateral (Sidebar)**:
  - Reducir y optimizar las dimensiones del panel lateral de detalles EXIF y propiedades del sistema, el cual actualmente ocupa demasiado espacio en la pantalla.






