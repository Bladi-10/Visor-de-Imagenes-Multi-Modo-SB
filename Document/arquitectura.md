# Arquitectura del Aplicativo y Guía del Código Fuente

## 1. Visión General de la Arquitectura
El visor de imágenes está diseñado bajo el patrón **Model-View-Update (MVU)** proporcionado por el framework **Relm4** sobre GTK4 y Libadwaita en Rust.

```
+-------------------------------------------------------------+
|                      Relm4 App Component                    |
|                                                             |
|   +------------------+             +--------------------+   |
|   |    AppModel      | --(View)--> |     AppView        |   |
|   |  (Estado App)    |             |   (GTK4 Widgets)   |   |
|   +------------------+             +--------------------+   |
|            ^                                 |              |
|            |                                 v              |
|     (Update / State)                    (User Input /       |
|            |                           Keyboard Controllers)|
|            +---------- AppMsg <--------------+              |
+-------------------------------------------------------------+
```

---

## 2. Estructura de Directorios del Código Fuente

El código fuente del proyecto se organiza en tres paquetes de módulos principales dentro del directorio `./src/`:

```
./src/
├── main.rs                 Punto de entrada de la aplicación y runtime de Relm4
├── app/                    Módulos principales del estado y ciclo de vida de la app
│   ├── mod.rs              Módulo raíz del paquete app
│   ├── model.rs            Estructuras de estado (AppModel, DeletedImageRecord, ImageItem) y gestión de ventanas de memoria
│   ├── msg.rs              Enum de mensajes y eventos (AppMsg)
│   └── view.rs             Construcción de interfaz GTK4 y manejadores de eventos
├── components/             Componentes reutilizables de interfaz de usuario
│   ├── mod.rs              Módulo raíz de componentes
│   ├── header.rs           Barra de encabezado (adw::HeaderBar) y botones de control
│   ├── viewport.rs         Área central de renderizado en fila de 1 a 4 imágenes y limpieza de texturas
│   ├── sidebar.rs          Panel lateral de metadatos EXIF y propiedades del sistema
│   └── manual_dialog.rs    Diálogo modal de ayuda con el manual de usuario
└── utils/                  Módulos auxiliares y lógica del sistema de archivos
    ├── mod.rs              Módulo raíz de utilidades
    ├── file_dialog.rs      Selector nativo de archivos con patrones por extensión (*.jpg, *.png)
    ├── image_loader.rs     Escaneo y carga de imágenes en directorios
    ├── metadata.rs         Extracción de metadatos EXIF mediante kamadak-exif
    └── trash_manager.rs    Gestión de retención temporal (Staging Trash) y envío diferido a papelera del SO
```

---

## 3. Puntos Claves de Localización de Código

Para modificar o inspeccionar componentes específicos, consulte las siguientes ubicaciones:

| Funcionalidad / Característica | Archivo de Código | Tipo / Función Clave |
| :--- | :--- | :--- |
| **Definición del estado global** | `./src/app/model.rs` | `struct AppModel` |
| **Ventana de memoria dinámica** | `./src/app/model.rs` | `AppModel::get_window_indices` (3 imágenes en Single, hasta 6 en multi-view) |
| **Pila de deshacer (Undo Stack)** | `./src/app/model.rs` | `struct DeletedImageRecord` |
| **Definición de eventos y mensajes** | `./src/app/msg.rs` | `enum AppMsg` |
| **Disposición en fila (1 a 4 imágenes)** | `./src/components/viewport.rs` | `ViewportComponent` / `gtk::Box` |
| **Limpieza de texturas de memoria RAM** | `./src/components/viewport.rs` | `picture.set_paintable(None)` |
| **Modo Ultra Limpio (Ctrl + R)** | `./src/app/view.rs` y `./src/components/viewport.rs` | `AppMsg::ToggleCleanUI` |
| **Modo Pantalla Completa (F11)** | `./src/app/view.rs` | `AppMsg::ToggleFullscreen` |
| **Barra de herramientas superior** | `./src/components/header.rs` | `HeaderComponent` |
| **Papelera Diferida (Staging Trash)** | `./src/utils/trash_manager.rs` | `TrashManager::send_to_trash_staging` / `restore_record` / `commit_trash` |
| **Diálogo nativo optimizado por patrones** | `./src/utils/file_dialog.rs` | `pick_single_file` (`add_pattern`) |
| **Extracción de metadatos EXIF** | `./src/utils/metadata.rs` | `extract_metadata` |
| **Ordenamiento de archivos** | `./src/utils/image_loader.rs` | `ImageLoader::scan_directory` (Alfabético por nombre) |
| **Diálogo modal de manual** | `./src/components/manual_dialog.rs` | `show_manual_dialog` |
| **Panel lateral de detalles** | `./src/components/sidebar.rs` | `SidebarComponent` |

---

## 4. Estrategia de Carga Peresoza y Manejo de RAM

1. **Calculo de Ventana**: En `AppModel::get_window_indices`, según el modo activo (`ViewMode`):
   - **Single**: Carga exactamente 3 elementos (índice anterior, actual y siguiente).
   - **Dual / Triple / Quad**: Carga hasta 6 elementos (1 anterior + N visibles + siguientes de búfer).
2. **Descarga de Texturas**: Toda imagen que sale de este rango ejecuta `unload_texture()`, dejando `texture = None`.
3. **Liberación en GTK**: En `ViewportComponent::update`, antes de desmantelar widgets `Picture` hijos, se limpia explícitamente el paintable (`set_paintable(None)`), obligando al motor de GTK a destruir los recursos de textura inmediatamente.
