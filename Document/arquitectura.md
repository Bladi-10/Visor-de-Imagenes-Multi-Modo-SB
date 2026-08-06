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
│   ├── model.rs            Estructuras de estado (AppModel, DeletedImageRecord, ImageItem)
│   ├── msg.rs              Enum de mensajes y eventos (AppMsg)
│   └── view.rs             Construcción de interfaz GTK4 con macro view!
├── components/             Componentes reutilizables de interfaz de usuario
│   ├── mod.rs              Módulo raíz de componentes
│   ├── header.rs           Barra de encabezado (adw::HeaderBar) y botones de control
│   ├── viewport.rs         Área central de renderizado en fila de 1 a 4 imágenes
│   ├── sidebar.rs          Panel lateral de metadatos EXIF y propiedades del sistema
│   └── manual_dialog.rs    Diálogo modal de ayuda con el manual de usuario
└── utils/                  Módulos auxiliares y lógica del sistema de archivos
    ├── mod.rs              Módulo raíz de utilidades
    ├── file_dialog.rs      Selectores nativos de archivos y carpetas de GTK4
    ├── image_loader.rs     Carga asíncrona concurrente con Rayon
    ├── metadata.rs         Extracción de metadatos EXIF mediante kamadak-exif
    └── trash_manager.rs    Operaciones de papelera con el crate trash
```

---

## 3. Puntos Claves de Localización de Código

Para modificar o inspeccionar componentes específicos, consulte las siguientes ubicaciones:

| Funcionalidad / Característica | Archivo de Código | Tipo / Función Clave |
| :--- | :--- | :--- |
| **Definición del estado global** | `./src/app/model.rs` | `struct AppModel` |
| **Pila de deshacer (Undo Stack)** | `./src/app/model.rs` | `struct DeletedImageRecord` |
| **Definición de eventos y mensajes** | `./src/app/msg.rs` | `enum AppMsg` |
| **Disposición en fila (1 a 4 imágenes)** | `./src/components/viewport.rs` | `ViewportComponent` / `gtk::Box` |
| **Modo Ultra Limpio (Ctrl + F11)** | `./src/app/view.rs` y `./src/components/viewport.rs` | `AppMsg::ToggleCleanUI` |
| **Barra de herramientas superior** | `./src/components/header.rs` | `HeaderComponent` |
| **Borrado seguro a la papelera** | `./src/utils/trash_manager.rs` | `TrashManager::send_to_trash` |
| **Borrado permanente** | `./src/utils/trash_manager.rs` | `TrashManager::delete_permanently` |
| **Extracción de metadatos EXIF** | `./src/utils/metadata.rs` | `MetadataExtractor::extract` |
| **Carga paralela de imágenes** | `./src/utils/image_loader.rs` | `ImageLoader::load_async` |
| **Diálogo modal de manual** | `./src/components/manual_dialog.rs` | `ManualDialog` |
| **Panel lateral de detalles** | `./src/components/sidebar.rs` | `SidebarComponent` |

---

## 4. Flujo de Datos para Disposición de 4 Imágenes en Fila

1. Cuando el usuario selecciona el modo 4 imágenes, se emite el evento `AppMsg::SetViewMode(ViewMode::Quad)`.
2. `AppModel` actualiza `view_mode` a `ViewMode::Quad`.
3. `./src/components/viewport.rs` recibe la notificación y configura un contenedor horizontal (`gtk::Box` con orientación `Horizontal`).
4. Las 4 imágenes activas contiguas se renderizan de izquierda a derecha en una sola fila continua.

