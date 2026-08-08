# Guía de Ciclo de Vida y Ruta de Aprendizaje: Rust, GTK4, Libadwaita y Relm4

## 1. Introducción y Propósito

Esta guía tiene como objetivo ofrecer una explicación técnica y pedagógica detallada sobre la arquitectura, el ciclo de vida y los fundamentos del visor de imágenes desarrollado en Rust 2021 utilizando **Relm4**, **GTK4** y **Libadwaita**.

Sirve como mapa de estudio para desarrolladores que deseen comprender el funcionamiento interno del proyecto, aprender cómo interactúan Rust y el ecosistema de interfaces gráficas de GNOME/GTK, y contar con una base sólida para realizar futuras extensiones.

---

## 2. Mapa Arquitectónico del Proyecto

El código fuente está estructurado de forma modular respetando la separación de responsabilidades:

```
src/
├── main.rs                 # Punto de entrada, configuración CSS global y runtime de Relm4
├── app/                    # Núcleo de estado y arquitectura MVU
│   ├── mod.rs              # Exportación de submódulos de app
│   ├── model.rs            # Estado de la aplicación (AppModel, ImageItem, ViewMode)
│   ├── msg.rs              # Definición de la enumeración de eventos (AppMsg)
│   └── view.rs             # Integración con SimpleComponent de Relm4 y construcción GTK
├── components/             # Componentes de interfaz gráfica reutilizables
│   ├── header.rs           # Barra de encabezado adw::HeaderBar y sus controles
│   ├── viewport.rs         # Área central de imágenes (renderizado de 1 a 4 imágenes en fila)
│   ├── sidebar.rs          # Panel lateral de detalles de metadatos EXIF
│   ├── confirm_dialog.rs   # Diálogo modal de confirmación al cerrar
│   └── manual_dialog.rs    # Diálogo modal con el manual de usuario
└── utils/                  # Librerías auxiliares y utilidades de sistema
    ├── file_dialog.rs      # Diálogos nativos de apertura de archivos
    ├── image_loader.rs     # Escaneo paralelo de directorios con Rayon
    ├── metadata.rs         # Extracción de metadatos EXIF
    └── trash_manager.rs    # Gestión de papelera diferida (staging) y papelera del SO
```

---

## 3. Ciclo de Vida Completo de la Aplicación

El ciclo de vida de la aplicación sigue el patrón **MVU (Model-View-Update)** implementado por **Relm4**, el cual garantiza un flujo de datos unidireccional y predictivo.

```
                    +--------------------------------+
                    |        1. Inicialización       |
                    |           (main.rs)            |
                    +---------------+----------------+
                                    |
                                    v
                    +--------------------------------+
                    |    2. Construcción de UI       |
                    |      (init en view.rs)         |
                    +---------------+----------------+
                                    |
                                    v
+-------------------> 3. Bucle de Eventos (Event Loop) <-------------------+
|                   |   (Captura teclado / mouse / UI) |                   |
|                   +---------------+----------------+                   |
|                                   |                                    |
|                                   v                                    |
|                   +--------------------------------+                   |
|                   |  4. Envío de Mensaje (AppMsg)  |                   |
|                   +---------------+----------------+                   |
|                                   |                                    |
|                                   v                                    |
|                   +--------------------------------+                   |
|                   |     5. Actualización Estado    |                   |
|                   |       (update en view.rs)      |                   |
|                   +---------------+----------------+                   |
|                                   |                                    |
|                                   v                                    |
|                   +--------------------------------+                   |
|                   |   6. Renderizado de Interfaz   |                   |
|                   |     (update_view / Viewport)   |                   |
|                   +---------------+----------------+                   |
|                                   |                                    |
+-----------------------------------+------------------------------------+
                                    | (Evento CloseApp / ESC)
                                    v
                    +--------------------------------+
                    |   7. Cierre Limpio y Staging   |
                    |     (TrashManager / Quit)      |
                    +--------------------------------+
```

### Fase 1: Inicialización y Bootstrapping (`src/main.rs`)
1. El programa inicia en `fn main()`.
2. Se instancia la aplicación Relm4 mediante `RelmApp::new("com.herramientas.visor")`.
3. Se crea y registra un proveedor CSS (`gtk::CssProvider`) global en la pantalla activa de GDK (`gtk::style_context_add_provider_for_display`). Esto define estilos como fondo negro en `.clean-viewport` y botones circulares semitransparentes en `.osd.circular`.
4. Se ejecuta el bucle de eventos invocando `app.run::<AppModel>(())`.

### Fase 2: Construcción Inicial del Estado y la UI (`AppModel::init` en `src/app/view.rs`)
1. `init_root()` construye la ventana raíz `adw::ApplicationWindow` con tamaño por defecto (1100x720).
2. `init()` inicializa el modelo principal `AppModel::new()`.
3. Se instancian los componentes secundarios:
   - `HeaderComponent::new`: Crea la barra de herramientas superior con botones de navegación, modos de vista y zoom.
   - `ViewportComponent::new`: Crea el contenedor central con superposición `gtk::Overlay` y botones flotantes `<` y `>`.
   - `SidebarComponent::new`: Crea el contenedor del panel lateral de detalles.
4. Se conectan los controladores globales de eventos a la ventana raíz:
   - `gtk::EventControllerKey`: Captura atajos de teclado (`1..4`, `F11`, `Ctrl+R`, `Left`, `Right`, `Delete`, `Ctrl+Z`, `Ctrl+E`, `ESC`).
   - `gtk::EventControllerMotion`: Detecta el movimiento del puntero del mouse cerca de la parte superior (Y <= 25px) durante pantalla completa para revelar temporalmente la barra flotante.
   - `connect_close_request`: Intercepta la solicitud de cierre de ventana para verificar si existen archivos retenidos en la papelera temporal.

### Fase 3: El Bucle de Eventos y el Modelo MVU (`AppMsg` -> `update` -> `update_view`)
Cuando ocurre una acción de usuario (ej. presionar la tecla `Right` o presionar un botón):
1. **Disparo de Evento**: El controlador envía un mensaje `AppMsg::NextImage` al canal de entrada de Relm4 (`input_sender.send(...)`).
2. **Fase Update (`SimpleComponent::update`)**:
   - Se ejecuta el método `update(&mut self, msg: AppMsg, sender: ComponentSender<Self>)`.
   - El modelo muta su estado: por ejemplo, `self.next_image()` incrementa `current_index` de forma circular.
   - `self.update_loaded_window()` recalcula la ventana de precarga de imágenes (-1 a +4 según el modo activo), cargando texturas necesarias con `item.load_texture()` y liberando las que salen de la ventana con `item.unload_texture()`.
3. **Fase View (`SimpleComponent::update_view`)**:
   - Relm4 invoca `update_view(&self, widgets: &mut AppWidgets, sender: ComponentSender<Self>)`.
   - Se actualizan las propiedades de visibilidad de la barra superior y panel lateral.
   - Se delega la actualización visual a `widgets.viewport.update(self)` y `widgets.header.update(self)`.

### Fase 4: Renderizado Optimizado en Viewport (`src/components/viewport.rs`)
Para garantizar un rendimiento de < 50ms e impedir fugas de memoria durante la navegación continua por carpetas de ~1000 imágenes:
1. `ViewportComponent::update` inspecciona los componentes hijos actuales de `self.row_box`.
2. **Reutilización en Sitio**: Si la cantidad de vistas horizontales requeridas coincide con la cantidad de contenedores `gtk::ScrolledWindow` existentes en pantalla, **no se destruye ningún widget GTK**. Simplemente se invoca `picture.set_paintable(Some(texture))` sobre las instancias `gtk::Picture` existentes.
3. **Desvinculación de Texturas**: Si una imagen sale de la vista o se requiere reconstruir contenedores, se ejecuta `picture.set_paintable(None::<&gdk::Texture>)` antes de desvincular el contenedor. Esto le indica explícitamente a GTK4 y GLib que liberen los buffers de memoria de la GPU/RAM de la textura.
4. **Verificación de Jerarquía**: Se valida `child.parent() == Some(row_box)` antes de ejecutar `row_box.remove(&child)`, previniendo advertencias de GTK en tiempo de ejecución.

### Fase 5: Cierre Limpio y Gestión de Papelera (`src/utils/trash_manager.rs`)
1. Si el usuario intenta cerrar la aplicación habiendo eliminado imágenes en la sesión:
   - Se despliega el diálogo modal de confirmación `show_close_confirmation_dialog`.
2. Si el usuario selecciona **"Enviar a Papelera y Salir"**:
   - `TrashManager::commit_trash_and_verify()` envía los archivos en staging a la papelera del sistema operativo mediante `trash::delete`.
   - Se verifica empíricamente que la carpeta temporal haya quedado completamente vacía.
   - Se invoca `relm4::main_application().quit()`.
3. Si el usuario selecciona **"Descartar Cambios y Salir"**:
   - `TrashManager::restore_all()` restituye las imágenes a sus rutas y carpetas originales en disco.
   - Se verifica la existencia de cada archivo restituido antes de finalizar la aplicación.

---

## 4. Conceptos Fundamentales del Lenguaje Rust Aplicados

Este proyecto sirve como una excelente referencia práctica de las características más potentes de Rust.

### A. Ownership (Propiedad), Move Semantics y Borrowing (`&` y `&mut`)
Rust garantiza la seguridad de memoria en tiempo de compilación sin requerir un recolector de basura (Garbage Collector):
- **Propiedad Exclusiva**: `let model = AppModel::new();` toma la propiedad de la estructura.
- **Referencias Inmutables (`&T`)**: En `update_view(&self, ...)`, el método puede leer el estado del modelo sin modificarlo ni duplicarlo en memoria.
- **Referencias Mutables (`&mut T`)**: En `update(&mut self, ...)`, Rust garantiza que solo existe una referencia mutable activa al modelo en un instante dado, evitando condiciones de carrera de datos.

### B. Enumeradores Avanzados y Pattern Matching (`enum`, `match`, `if let`)
En Rust, los enumeradores pueden contener datos asociados. `AppMsg` encapsula todos los eventos posibles:

```rust
pub enum AppMsg {
    SetViewMode(ViewMode),
    SingleFileSelected(Option<PathBuf>),
    SelectActiveImage(usize),
    // ...
}
```

El procesamiento mediante `match` en `src/app/view.rs` obliga a manejar exhaustivamente cada variante de mensaje, evitando estados inválidos no contemplados.

### C. Tipos Seguros para Ausencia de Valores (`Option<T>`) y Manejo de Errores (`Result<T, E>`)
En lugar de utilizar valores nulos (`null` / `undefined`), Rust utiliza `Option<T>` (`Some(valor)` o `None`).
- En `ImageItem`: `pub texture: Option<gdk::Texture>` indica explícitamente que la textura puede no estar cargada en memoria en un momento dado.
- `Result<T, E>` se utiliza en `TrashManager` para propagar errores de operaciones de archivos sin colapsar el programa de forma inesperada.

### D. Punteros Inteligentes y Mutabilidad Interior (`Rc`, `Cell`)
En `src/components/viewport.rs`, para conectar el gesto de arrastre de mouse `gtk::GestureDrag` sin violar las reglas de propiedad de Rust:
- `Rc<T>` (Reference Counted): Permite compartir la propiedad de las coordenadas iniciales de desplazamiento entre múltiples cierres de eventos (closures).
- `Cell<f64>`: Permite mutabilidad interior segura en un solo hilo para actualizar las coordenadas `start_h` y `start_v` durante el arrastre.

```rust
let start_h = std::rc::Rc::new(std::cell::Cell::new(0.0));
let start_h_begin = start_h.clone();
```

### E. Concurrencia y Asincronía (`rayon`, `tokio` / `relm4::spawn_local`)
- **Escaneo en Paralelo**: En `src/utils/image_loader.rs`, se utiliza `rayon` con `paths.into_par_iter()` para filtrar y ordenar cientos de imágenes del disco utilizando todos los núcleos del procesador en paralelo.
- **Diálogos de Archivo Asíncronos**: En `src/utils/file_dialog.rs`, `relm4::spawn_local` permite abrir el selector nativo de archivos de forma asíncrona sin congelar la interfaz de usuario.

---

## 5. El Framework GTK4 y Libadwaita en Rust

### Libadwaita vs GTK4 Tradicional
- **GTK4**: Ofrece los bloques de construcción básicos (ventanas, botones, cajas de diseño, eventos).
- **Libadwaita**: Es la librería de widgets modernos de GNOME. Proporciona componentes como `adw::ApplicationWindow`, `adw::HeaderBar` y `adw::ToastOverlay` con soporte nativo para temas claro/oscuro, esquinas redondeadas adaptativas y animaciones fluídas.

### Estructura de Contenedores de la Interfaz
La jerarquía de widgets construida en `src/app/view.rs` sigue esta distribución:

```
adw::ApplicationWindow (Ventana Raíz)
 └── gtk::Box (Vertical, spacing: 0)
      ├── adw::HeaderBar (Barra Superior)
      └── gtk::Box (Horizontal, Content Box)
           ├── adw::ToastOverlay (Notificaciones flotantes)
           │    └── gtk::Overlay (Capa de Viewport)
           │         ├── gtk::Box (row_box: Fila continua 1..4 imágenes)
           │         │    └── gtk::ScrolledWindow (Desplazamiento / Panning)
           │         │         └── gtk::Picture (Renderizado de gdk::Texture)
           │         ├── gtk::Button (btn_prev: Flecha flotante izquierda)
           │         └── gtk::Button (btn_next: Flecha flotante derecha)
           └── gtk::Box (Sidebar Container: Panel lateral EXIF)
```

### Controladores de Eventos (Event Controllers)
En GTK4, los eventos de entrada no se manejan sobrecargando métodos de widgets, sino asociando controladores independientes:
- `gtk::EventControllerKey`: Se añade a la ventana principal para interceptar pulsaciones de teclas globales antes de que lleguen a los aparatos individuales.
- `gtk::EventControllerMotion`: Supervisa las coordenadas del mouse (`x, y`) para mostrar u ocultar dinámicamente la barra superior en modo pantalla completa.
- `gtk::GestureDrag`: Se asocia a cada `gtk::ScrolledWindow` para permitir el desplazamiento por arrastre con clic sostenido del mouse.

---

## 6. Guía de Estudio y Hoja de Ruta para Futuras Mejoras

Para dominar este proyecto y extender sus capacidades, se sugiere seguir este orden de lectura y experimentación:

### Paso 1: Comprensión del Estado y Mensajes
- Estudiar [`src/app/msg.rs`](file:///home/bladimir/Documentos/02%20PROYECTOS/06%20Visor%20de%20imagenes/src/app/msg.rs) para comprender todos los eventos que la aplicación puede procesar.
- Inspeccionar [`src/app/model.rs`](file:///home/bladimir/Documentos/02%20PROYECTOS/06%20Visor%20de%20imagenes/src/app/model.rs) y sus pruebas unitarias para entender la ventana deslizante de precarga de imágenes (`get_window_indices`).

### Paso 2: Comprensión de la Construcción Visual
- Revisar [`src/app/view.rs`](file:///home/bladimir/Documentos/02%20PROYECTOS/06%20Visor%20de%20imagenes/src/app/view.rs) para seguir el flujo de `init`, `update` y `update_view`.
- Estudiar [`src/components/viewport.rs`](file:///home/bladimir/Documentos/02%20PROYECTOS/06%20Visor%20de%20imagenes/src/components/viewport.rs) para aprender cómo se reutilizan los componentes gráficos y se gestiona el zoom.

### Paso 3: Propuestas para Futuras Extensiones
Si se desea agregar nuevas funcionalidades en el futuro, las siguientes son excelentes prácticas de aprendizaje:
1. **Rotación de Imágenes (90° / 180° / 270°)**:
   - Agregar una variante `RotateClockwise` a `AppMsg`.
   - Aplicar una transformación gráfica mediante `gdk::MemoryTexture` o transformaciones CSS en `ViewportComponent`.
2. **Modo Pase de Diapositivas (Slideshow)**:
   - Agregar un temporizador asíncrono en Relm4 que envíe periódicamente `AppMsg::NextImage` cada N segundos.
3. **Filtros Visuales y Ajustes de Brillo/Contraste**:
   - Integrar shaders de GTK4 (`gtk::GLShader`) o manipulaciones de píxeles con el crate `image` para aplicar efectos en tiempo real.
