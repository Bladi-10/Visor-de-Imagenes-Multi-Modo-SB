# **Especificación Técnica de Arquitectura y Diseño: Visor de Imágenes**

**Versión:** 1.3.0  
**Estado:** Especificación Técnica Aprobada y Actualizada  
**Plataforma Objetivo:** Linux Mint XFCE / GTK4 / Libadwaita / Relm4 (Integración con Thunar)  
**Lenguaje:** Rust 2021

## **1\. Resumen Ejecutivo y Objetivos**

El objetivo de esta especificación es definir la arquitectura, la experiencia de usuario (UI/UX), la matriz de atajos de teclado, la gestión de archivos (borrado/restauración) y el modelo de estado para el desarrollo de un **Visor de Imágenes Avanzado y Modular** escrito en Rust utilizando **Relm4**, **GTK4** y **Libadwaita**.

### **Principales Innovaciones y Funcionalidades**

> * **Modo Ultra Limpio (Ctrl \+ F11)**: Desaparición total de la interfaz (HeaderBar, botones flotantes \< \>), dejando un canvas 100% dedicado al renderizado de las imágenes.  
> * **Modos Multi-Imagen (1, 2, 3 y 4)**: Distribución dinámica de imágenes en fila (modo 4 imágenes en una sola fila continua).  
> * **Sistema de Gestión de Papelera y Deshacer**:  
  * Delete: Envío de la imagen activa a la papelera del SO (GIO / trash) con diálogo/notificación de confirmación.  
  * Ctrl \+ Delete / Ctrl \+ Supr: Eliminación permanente directa del sistema de archivos.  
  * Ctrl \+ Z: Pila de deshacer (Undo) que restaura la última imagen eliminada y la reincorpora a la interfaz.  
> * **Visualización de Manual de Usuario**: Modal/Diálogo informativo interactivo invocado desde el ícono de información (info-symbolic).  
> * **Controles Integrados de Ventana**: Botones nativos para Minimizar y Maximizar/Restaurar integrados en la barra principal.  
> * **Navegación Circular e Inmersiva**: Desplazamiento contiguo (![][image1]), con soporte para salida rápida con la tecla ESC.

## **2\. Stack Tecnológico y Dependencias**

### **2.1 Dependencias del Sistema (Debian/Ubuntu/Linux Mint)**

sudo apt update && sudo apt install \-y \\  
    libgtk-4-dev \\  
    libadwaita-1-dev \\  
    pkg-config \\  
    cargo \\  
    rustc \\  
    thunar

### **2.2 Configuración del Proyecto (Cargo.toml)**

\[package\]  
name \= "herramientas-sistema"  
version \= "0.1.0"  
edition \= "2021"  
authors \= \["Equipo de Desarrollo"\]  
description \= "Visor de imágenes multi-modo desarrollado en Rust, GTK4 y Relm4"

\[dependencies\]  
gtk \= { package \= "gtk4", version \= "0.9" }  
adw \= { package \= "libadwaita", version \= "0.6" }  
relm4 \= { version \= "0.9", features \= \["libadwaita"\] }  
relm4-components \= "0.9"  
rayon \= "1.10"  
image \= "0.25"  
kamadak-exif \= "0.5"  
trash \= "5.1" \# Gestión multiplataforma de papelera del sistema de archivos

## **3\. Arquitectura del Sistema y Módulos**

src/  
├── main.rs                 \# Punto de entrada y runtime de Relm4  
├── app/  
│   ├── mod.rs              \# Componente principal Dashboard  
│   ├── model.rs            \# Estado del visor (AppModel y UndoStack)  
│   ├── msg.rs              \# Enumeración de eventos y mensajes (AppMsg)  
│   └── view.rs             \# Macro view\! y construcción GTK  
├── components/  
│   ├── mod.rs  
│   ├── header.rs           \# HeaderBar (Controles de ventana, zoom, modofs, info)  
│   ├── viewport.rs         \# Área central (Renderizado 1..4, flechas flotantes superpuestas)  
│   ├── sidebar.rs          \# Panel lateral de detalles (EXIF / Sistema)  
│   └── manual\_dialog.rs    \# Diálogo modal con el Manual de Usuario  
└── utils/  
    ├── mod.rs  
    ├── file\_dialog.rs      \# Selección nativa de archivos/carpetas  
    ├── image\_loader.rs     \# Decodificación concurrente con Rayon  
    ├── metadata.rs         \# Extracción EXIF  
    └── trash\_manager.rs    \# Operaciones de borrado, papelera del SO y restauración

## **4\. Especificación Funcional de la Interfaz (UI/UX)**

### **4.1 Esquema de Interfaz General**

\+---------------------------------------------------------------------------------+  
| [Info] [F11] [Abrir] [Carpeta]   [ 1 | 2 | 3 | 4 ]   [-] [+] [1:1] [Fit]  [_] [[]] [X] |  <- HeaderBar  
+---------------------------------------------------------------------------------+  
| < |                                                                         | > |  
|   |                      [ Área Central de Imágenes ]                       |   |  <- Viewport  
|   |            (Distribución dinámica de 1 a 4 imágenes en fila)             |   |  
|   |                                                                         |   |  +------------+  
|   | +-------+ +-------+ +-------+ +-------+                                 |   |  | DETALLES   |  
|   | | Img A | | Img B | | Img C | | Img D |                                 |   |  | (Ctrl + E) |  
|   | +-------+ +-------+ +-------+ +-------+                                 |   |  | Nombre: .. |  
|   |                                                                         |   |  | Res: ..    |  
|   |                                                                         |   |  | Exif: ..   |  
|   |                                                                         |   |  +------------+  
+---------------------------------------------------------------------------------+

### **4.2 Detalle de Componentes de Interfaz**

#### **A. Barra de Encabezado (HeaderBar)**

> * **Izquierda**:  
  * info-symbolic: Abre el Diálogo Modal de Manual de Usuario.  
  * view-fullscreen-symbolic (F11): Alterna el modo pantalla completa estándar.  
  * document-open-symbolic: Abre diálogo nativo de archivo individual.  
  * folder-open-symbolic: Abre diálogo nativo de carpeta completa (integra con Thunar).  
> * **Centro**:  
  * Selector de Modo de Vista (\[ 1 \], \[ 2 \], \[ 3 \], \[ 4 \]).  
> * **Derecha**:  
  * Botones de Zoom: Disminuir (-), Aumentar (+), Real (1:1), Ajustar pantalla (Fit).  
  * info-symbolic / sidebar-show-symbolic: Alterna el panel lateral (Ctrl \+ E).  
  * Controles Nativo de Ventana: Minimizar (window-minimize-symbolic), Maximizar/Restaurar (window-maximize-symbolic) y Cerrar.

#### **B. Modo Ultra Limpio (Ctrl \+ F11)**

> * Al presionar Ctrl \+ F11, la interfaz ejecuta las siguientes mutaciones:  
  1. HeaderBar.set\_visible(false).  
  2. Visibilidad de los botones flotantes \< \> en el Viewport: false.  
  3. Visibilidad del Sidebar: false.  
  4. La ventana conmuta automáticamente a pantalla completa sin bordes.  
> * Presionar Ctrl \+ F11 nuevamente restaura la interfaz a su estado previo.

#### **C. Sistema de Borrado, Papelera y Notificaciones**

> 1. **Borrado Normal (Delete)**:  
   * Envía la imagen seleccionada a la papelera mediante la API nativa de la papelera del SO (trash crate / gio::File::trash).  
   * Remueve la imagen del catálogo activo en memoria.  
   * Dispara una notificación emergente (adw::Toast o gtk::AlertDialog) indicando: "Imagen enviada a la papelera del sistema", con un botón de acción directa **"Deshacer"**.  
> 2. **Borrado Permanente (Ctrl \+ Delete / Ctrl \+ Supr)**:  
   * Muestra un cuadro de diálogo modal de confirmación crítica: *"¿Desea eliminar permanentemente este archivo? Esta acción no se puede deshacer."*  
   * Si se confirma, elimina el archivo físicamente en disco.  
> 3. **Deshacer Borrado (Ctrl \+ Z)**:  
   * Recupera la última imagen enviada a la papelera y la vuelve a incluir en la lista de imágenes activas en su índice correspondiente.

## **5\. Matriz Completa de Atajos de Teclado y Controladores de Eventos**

| Atajo de Teclado | Acción Asociada / Evento | Comportamiento en la UI |
| :---- | :---- | :---- |
| **1** | AppMsg::SetViewMode(ViewMode::Single) | Cambia al modo de visualización de 1 imagen. |
| **2** | AppMsg::SetViewMode(ViewMode::Dual) | Cambia al modo de visualización de 2 imágenes en fila. |
| **3** | AppMsg::SetViewMode(ViewMode::Triple) | Cambia al modo de visualización de 3 imágenes en fila. |
| **4** | AppMsg::SetViewMode(ViewMode::Quad) | Cambia al modo de visualización de 4 imágenes en fila. |
| **F11** | AppMsg::ToggleFullscreen | Alterna el modo pantalla completa manteniendo visible la UI según configuración. |
| **Ctrl \+ F11** | AppMsg::ToggleCleanUI | **Modo Ultra Limpio**: Oculta HeaderBar, controles flotantes y deja solo la imagen. |
| **Left (![][image2])** | AppMsg::PreviousImage | Navega a la imagen anterior (cálculo circular ![][image3]). |
| **Right (![][image4])** | AppMsg::NextImage | Navega a la imagen siguiente (cálculo circular ![][image5]). |
| **Delete / Supr** | AppMsg::TrashActiveImage | Mueve la imagen actual a la papelera del SO y muestra cuadro de diálogo/toast. |
| **Ctrl \+ Delete** | AppMsg::PermanentlyDeleteActiveImage | Muestra confirmación modal y elimina la imagen definitivamente en disco. |
| **Ctrl \+ Z** | AppMsg::UndoDelete | Restaura la última imagen borrada a la interfaz y catálogo. |
| **Ctrl \+ E** | AppMsg::ToggleSidebar | Muestra u oculta el panel lateral de detalles EXIF/Metadatos. |
| **ESC** | AppMsg::CloseApp | Cierra la aplicación de forma inmediata. Si está en Modo Ultra Limpio, puede restaurar primero la UI. |

## **6\. Modelo de Estado y Mensajes de Relm4**

### **6.1 Tipos de Datos y Estructura del Estado**

use std::path::PathBuf;

\#\[derive(Debug, Clone, Copy, PartialEq, Eq)\]  
pub enum ViewMode {  
    Single \= 1,  
    Dual \= 2,  
    Triple \= 3,  
    Quad \= 4,  
}

\#\[derive(Debug, Clone)\]  
pub struct DeletedImageRecord {  
    pub original\_path: PathBuf,  
    pub original\_index: usize,  
    pub was\_trashed: bool,  
}

\#\[derive(Debug, Clone)\]  
pub struct ImageItem {  
    pub path: PathBuf,  
    pub texture: Option\<gdk::Texture\>,  
    pub metadata: Option\<ImageMetadata\>,  
}

\#\[derive(Debug, Clone)\]  
pub struct ImageMetadata {  
    pub file\_name: String,  
    pub file\_path: String,  
    pub dimensions: (u32, u32),  
    pub size\_bytes: u64,  
    pub mime\_type: String,  
    pub camera\_model: Option\<String\>,  
    pub iso: Option\<u32\>,  
    pub f\_number: Option\<f32\>,  
}

### **6.2 Mensajes de la Aplicación (AppMsg)**

\#\[derive(Debug)\]  
pub enum AppMsg {  
    // Control de Ventana y Modos Visuales  
    ToggleFullscreen,  
    ToggleCleanUI,  
    MinimizeWindow,  
    MaximizeWindow,  
    CloseApp,  
    ShowUserManual,  
      
    // Naves e Interacción  
    NextImage,  
    PreviousImage,  
    SelectActiveImage(usize),  
    SetViewMode(ViewMode),  
      
    // Zoom  
    ZoomIn,  
    ZoomOut,  
    ZoomReset,  
    ZoomFit,  
      
    // Gestión de Archivos y Papelera  
    OpenSingleFile,  
    OpenFolder,  
    TrashActiveImage,  
    PermanentlyDeleteActiveImage,  
    UndoDelete,  
      
    // Visualización  
    ToggleSidebar,  
    ImageTextureReady { index: usize, texture: gdk::Texture, metadata: ImageMetadata },  
}

### **6.3 Modelo Principal (AppModel)**

pub struct AppModel {  
    pub images: Vec\<ImageItem\>,  
    pub current\_index: usize,  
    pub view\_mode: ViewMode,  
    pub zoom\_level: f64,  
    pub is\_fit\_mode: bool,  
    pub is\_fullscreen: bool,  
    pub is\_clean\_ui: bool,          // Control de Ctrl \+ F11  
    pub is\_sidebar\_open: bool,       // Control de Ctrl \+ E  
    pub undo\_stack: Vec\<DeletedImageRecord\>, // Pila para Ctrl \+ Z  
}

## **7\. Estrategia de Carga y Manejo de Datos**

> 1. **Borrado Seguro (TrashManager)**:  
   * Las imágenes borradas con Delete utilizan el crate trash::delete(\&path) para aprovechar el mecanismo nativo de la papelera en XFCE/Desktop Environments de Linux.  
   * El registro en undo\_stack guarda la ruta original e índice previo para permitir la reinserción inmediata si el usuario presiona Ctrl \+ Z.  
> 2. **Renderizado en Modo Ultra Limpio**:  
   * El componente Viewport detecta cuando is\_clean\_ui \== true y aplica una clase CSS .clean-viewport eliminando paddings, márgenes y los widgets flotantes overlay.

## **8\. Recursos de Referencia Obligatoria**

> * **Relm4 Book (Estable)**: [https://relm4.org/book/stable/](https://relm4.org/book/stable/)  
> * **Documentación API Relm4**: [https://relm4.org/docs/next/relm4/index.html](https://relm4.org/docs/next/relm4/index.html)  
> * **Libro Oficial GTK4 para Rust**: [https://gtk-rs.org/gtk4-rs/stable/book/](https://gtk-rs.org/gtk4-rs/stable/book/)  
> * **API Bindings GTK4 en Rust**: [https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/)

[image1]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGEAAAAaCAYAAACn4zKhAAAEWElEQVR4Xu2Ze+hlUxTH13i/CyENQiRNqcGQ5y+SPEMk+YcxJqSJTCZixtAQySsSifnDO+/MRIxp8ghTRIzBeKQ8ZvI2MpP39/NbZ3f3WXPu75zfne69w+9+6tvv7LXPOfecvfZea+3zMxswYMCA/y23SsdHYx9YT9pW2iJ2dJFHpUnR2Gsule6Lxj5wm/SX9I90bujrJjtJ70k7xo5esbe0XNoydvSJY6z3ToDzpBeicTQMSd9Jp8SOBrAUb4rGPrKndeaEy6WPpdXm119d7rY9pG+KPvS9NCXr30z6QTo4s40KYvmb0l6xo4btpT/NV8O6AoPViRNgfWmptMp8kDcvdw/zmHRBNBbcIT0Zjd2GF2UFjcS4aOgya+OEg6S7pbvM7zGt3D3MW+azvoozpF/Mndkz7pHmR6N40Hzp/i4dYp4wn5G+Mr9mQ+kS81D2gfns2Wr4yhZUN4S5d6TXpUXSYfkJ5g6mKPhQWmg+S8+xzp1wpXSquSNJ8J9ZeUB3kBZk7chu5r89qkrpaOk56SPp2NDXhMXSLdEodpZuNH+gt60V5lLSfM38ZYFykuV/bdGGjaU3zGclJSfgzJXm90jcLq2wVjjEufdb5054Sdq6OH7C/D6nt7rtTOmyrF3FH9LZ0diO8dI8c0+zxDqJZV9IV0VjAYPMS8zObMwkbLxsDpPgxazNjPzb3EE5D5mvJpw00fxecVD2L+yjdQLx/+WsfaD5fZhoibnSvlm7CsLz9Ghsx4XSodKu5i98Uam3GT9Z++tONn+JEzMbg4qNVZJDSMoHgPbXWTsxy/z6o8ydy/Fx+QnWuRO4z5xg45m41+FF+12rz3GfSNdFYx2EAcqybWJHA3609k5g8HmBPHyw1LHFl31feiVr/2y+yiLMeq6fbB6q8gFKdOoEwupQsKV3eFbaR3q43F0JTrg+GkdiA/ME+kDsaMin1j4cnWDNnbDEyk6gXVV1MWHSPWcXx+S1nE6dQIm+UbAx61mVRAq+CJD06+C52XM0JsVtqo7dbc3BqaNdYoYUjpo64dWsfY35edtlNiBv8ZKbWCsnXFw6w+yAwj412EeC/EiBUkWqthAFRx0k5nwTVwsD+GVxTOk4IetrwlyrLlGBmpkHPymzMajY4g57mZUTIOUq8Zf7s1phyLyKSlUV3Gm+uUrlLdXRU+a/cYO0aWGvg1wT81SC1UF+4nfqSCVqXfIuwdKlMnlaOi30NeEs6dtoFI9Lv5k/EH/ZDxDPmcXY2GV/Lh1h5U8BTIj9zGHVMEnIF4h9wJFFX4JBn2meQNlgkSfIUel+xOeRuMK8xOVc9gXsN6q+gc2Qbo7GCph4v9qaYa2rsIyrPlsQLlJ9z1/aPFja+BBrsTHLGchk43htd5v8DiVsP+CzxSPR2AtI6jG8jEXSB7xYrfUEVgFLumoZjyX4lP18NPaS86V7o3EMwT91+Ay+S+zoNSSudeHfm/2APMBnjgEDBgz4L/IvqAv2rvHcCMEAAAAASUVORK5CYII=>

[image2]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABIAAAAWCAYAAADNX8xBAAAAa0lEQVR4XmNgGAWjgHYgEV2AHGAExLPRBckBC4HYBF2QVBAFxJPRBUkBHEBcA8TLgJgVTY4kMAuI/wPxRSA+QgaeyIAEPIH4ABCLIAuSC0CBvBVdkFzQBsQ+6ILkAAUgXo0uSC4wRxcYwQAAYbAU6eEAGUYAAAAASUVORK5CYII=>

[image3]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAHUAAAAXCAYAAAA1OADtAAABE0lEQVR4Xu3YMUtCURiH8QPaF5A2l3BJkLbGwi2UprBvIeSXqLVWwbVZGoJa2gRx6AMETSK5GDQkGkH1HK6D951Ejwe6/H/wgJz3Or2XC/c6JyIiIlm0Zw/kf9qhMl3Th5lJAFWa0JkdbMkBvVOfhvSZHksIpzSgfTuI4NFpqZmjpWaQlhrYCT3QC9XNLBYtNaAi3VOOnqmbHkejpQbUpCOXvCP+UCs1jccvdWoPZTNXNKeCHRgX1Fux9uI/q/BLndlDWV+exnRrBxH5pfqbSgJp0C8dU4ku0+Mo/FK/7KGs74ZGi98dqizNYnmib5d8NpQADl3yOnNH52a2Tbv0Sm8ueVL4/Pdff1Zbuk5EREREZHN/q64349e7sroAAAAASUVORK5CYII=>

[image4]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABIAAAAWCAYAAADNX8xBAAAAa0lEQVR4XmNgGAWjYOBAEroAuWA7EIuhC5IDAoG4A12QXLASiJ3QBckBXEC8AYhzgJgVJrgMiI+QgW8C8T8gbmagAKgyQAwzRpcgBbAD8VEgVkATJxnkAnEGuiA54AAQc6ILkgNM0AWGMQAAHrMU7X943RsAAAAASUVORK5CYII=>

[image5]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAHUAAAAXCAYAAAA1OADtAAABP0lEQVR4Xu3YPUqDURCF4QF1A6JVGhFEQewsIzZBFCvRXVjYWljGNllA2tRiIZjGWgy6AMVK/GkULBQNgnqGz8I7CN94vZBLPA+8IDPEZpKQRISIiIgG0YQdUH+N2IGTPm4GNdCj2VECi+gerdlFiQW0bYcOc+gBHaMr9BSuKYVVdIKm7aJEDe3Y4S91hEfNypLwqANnWXjU7Ogr7RCdoxWz8+BRM1NBB2gInaG9cO3Co2ZmE1Wl+I74jraCbWgcddGp6QLd/DDX9JOxhx712Q7pb3bRKxq1C4dUr9QXO6R4w+gOte3CKdVR9UlFiayjDyneKidRPVyXSnXUnh1SvCa6/vq7hWa/7TxSHPUIvUn8z41kzEvxdWYfbZidR+xRx9AlupXinULT3391pv+T+ij2qJQx/cQ8ZYdERET/yCeKT0KTGTmpkQAAAABJRU5ErkJggg==>