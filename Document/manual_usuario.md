# Manual de Usuario - Visor de Imágenes

## 1. Introducción
El Visor de Imágenes es una aplicación desarrollada en Rust utilizando Relm4, GTK4 y Libadwaita para Linux Mint / XFCE. Permite visualizar imágenes individuales o múltiples imágenes simultáneamente en fila, gestionar la eliminación y restauración de archivos y explorar metadatos EXIF.

---

## 2. Modos de Visualización

### 2.1 Vista Multi-Imagen
Mediante el selector en la barra superior (HeaderBar) o usando las teclas numéricas **1**, **2**, **3** y **4**, es posible conmutar entre los modos de visualización:
- **Modo 1 Imagen** (Tecla `1`): Visualización a pantalla completa de la imagen activa.
- **Modo 2 Imágenes** (Tecla `2`): Muestra 2 imágenes contiguas horizontalmente.
- **Modo 3 Imágenes** (Tecla `3`): Muestra 3 imágenes contiguas horizontalmente.
- **Modo 4 Imágenes** (Tecla `4`): Muestra 4 imágenes dispuestas en una sola fila continua.

### 2.2 Modo Ultra Limpio (Ctrl + F11)
- Oculta completamente la barra de encabezado, los botones flotantes de navegación y el panel lateral.
- Establece la ventana en modo pantalla completa sin bordes.
- Dedica el 100% del área de pantalla a las imágenes.
- Para salir del Modo Ultra Limpio, presione **Ctrl + F11** o **ESC**.

---

## 3. Navegación
- **Siguiente Imagen**: Tecla de flecha **Derecha** o clic en el botón flotante derecho (`>`).
- **Imagen Anterior**: Tecla de flecha **Izquierda** o clic en el botón flotante izquierdo (`<`).
- La navegación es **circular**: al llegar a la última imagen, continua desde la primera.

---

## 4. Gestión de Archivos y Papelera

### 4.1 Enviar a Papelera (Tecla Supr / Delete)
- Mueve la imagen seleccionada a la papelera del sistema operativo de forma segura.
- Muestra una notificación con la opción directa de deshacer.

### 4.2 Deshacer Borrado (Ctrl + Z)
- Recupera la última imagen enviada a la papelera y la inserta nuevamente en la posición original del catálogo.

### 4.3 Borrado Permanente (Ctrl + Supr / Ctrl + Delete)
- Muestra un diálogo modal de confirmación crítica antes de eliminar el archivo físicamente del disco.

---

## 5. Panel Lateral de Detalles (Ctrl + E)
- Presione **Ctrl + E** o utilice el botón de metadatos en la barra superior.
- Muestra información técnica de la imagen activa: nombre, ruta, dimensiones, tamaño de archivo, tipo MIME y datos EXIF (cámara, ISO, apertura f-number).

---

## 6. Resumen de Atajos de Teclado

| Atajo | Función |
| :--- | :--- |
| **1** | Cambiar a modo 1 imagen |
| **2** | Cambiar a modo 2 imágenes en fila |
| **3** | Cambiar a modo 3 imágenes en fila |
| **4** | Cambiar a modo 4 imágenes en fila |
| **F11** | Alternar modo pantalla completa |
| **Ctrl + F11** | Modo Ultra Limpio |
| **Flecha Izquierda** | Imagen anterior |
| **Flecha Derecha** | Imagen siguiente |
| **Supr / Delete** | Enviar imagen activa a la papelera |
| **Ctrl + Supr** | Borrado permanente en disco (con confirmación) |
| **Ctrl + Z** | Deshacer último borrado |
| **Ctrl + E** | Alternar panel de detalles EXIF |
| **ESC** | Cerrar aplicación / Restaurar interfaz |
