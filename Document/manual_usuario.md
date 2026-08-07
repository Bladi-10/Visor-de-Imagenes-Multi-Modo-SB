# Manual de Usuario - Visor de Imágenes

## 1. Introducción
El Visor de Imágenes es una aplicación desarrollada en Rust utilizando Relm4, GTK4 y Libadwaita para Linux. Permite visualizar imágenes individuales o múltiples imágenes simultáneamente en fila, gestionar la eliminación y restauración de archivos y explorar metadatos EXIF.

---

## 2. Apertura y Selección de Imágenes
- **Abrir Archivo de Imagen**: Haga clic en el ícono de carpeta/documento en la barra superior o use la tecla correspondiente.
- **Carga Automática de Directorio**: Al seleccionar una imagen individual, el visor identifica automáticamente el directorio contenedor, escanea todas las imágenes compatibles presentes en esa carpeta y las carga en la secuencia de navegación, posicionando la vista en la imagen seleccionada.

---

## 3. Modos de Visualización

### 3.1 Vista Multi-Imagen
Mediante el selector en la barra superior (HeaderBar) o usando las teclas numéricas **1**, **2**, **3** y **4**, es posible conmutar entre los modos de visualización:
- **Modo 1 Imagen** (Tecla `1`): Visualización de la imagen activa con ventana de precarga optimizada a 3 imágenes en memoria.
- **Modo 2 Imágenes** (Tecla `2`): Muestra 2 imágenes contiguas horizontalmente.
- **Modo 3 Imágenes** (Tecla `3`): Muestra 3 imágenes contiguas horizontalmente.
- **Modo 4 Imágenes** (Tecla `4`): Muestra 4 imágenes dispuestas en una sola fila continua.

### 3.2 Modo Ultra Limpio (Ctrl + R)
- Oculta completamente la barra de encabezado, los botones flotantes de navegación (`<` y `>`) y el panel lateral.
- Funciona de manera independiente tanto en modo ventana normal como en pantalla completa.
- Para activar o desactivar el Modo Ultra Limpio, presione **Ctrl + R**.

### 3.3 Modo Pantalla Completa (F11)
- Alterna la ventana a pantalla completa.
- La barra de encabezado permanece oculta por defecto y aparece al acercar el cursor al borde superior de la pantalla (`y <= 25px`).
- En modo pantalla completa, los botones de minimizar y maximizar de la barra flotante se ocultan automáticamente, manteniendo el botón de cerrar.

---

## 4. Navegación
- **Siguiente Imagen**: Tecla de flecha **Derecha** o clic en el botón flotante derecho (`>`).
- **Imagen Anterior**: Tecla de flecha **Izquierda** o clic en el botón flotante izquierdo (`<`).
- La navegación es **circular**: al llegar a la última imagen, continúa desde la primera.

---

## 5. Gestión de Archivos y Papelera Diferida

### 5.1 Enviar a Papelera (Tecla Supr / Delete)
- Al presionar **Supr** / **Delete**, la imagen activa se mueve a una carpeta de retención temporal (`Staging Trash`).
- La imagen se retira inmediatamente de la interfaz sin tocar la papelera del sistema operativo en ese momento.

### 5.2 Deshacer Borrado (Ctrl + Z)
- Al presionar **Ctrl + Z**, la imagen retenida vuelve inmediatamente a su ubicación original en disco.
- **Ventaja**: No se genera ningún archivo duplicado en la papelera del sistema operativo.

### 5.3 Diálogo de Confirmación al Cerrar
- Si al intentar cerrar la aplicación (tecla `ESC` o botón `X` de la ventana) existen imágenes en la papelera de retención temporal, la aplicación despliega un diálogo de confirmación interactivo con 3 opciones:
  1. **Enviar a Papelera y Salir**: Transfiere los archivos a la papelera del SO (`trash::delete`), verifica el éxito de la operación y la limpieza del directorio temporal y cierra el aplicativo.
  2. **Descartar Cambios y Salir**: Restaura automáticamente todas las imágenes de retención de regreso a sus **directorios y rutas originales**, verifica la existencia de cada archivo en disco y cierra la aplicación.
  3. **Cancelar**: Cierra el diálogo y permite continuar la sesión de trabajo.

---

## 6. Panel Lateral de Detalles (Ctrl + E)
- Presione **Ctrl + E** o utilice el botón de metadatos en la barra superior.
- Muestra información técnica de la imagen activa: nombre, ruta, dimensiones, tamaño de archivo, tipo MIME y datos EXIF.

---

## 7. Resumen de Atajos de Teclado

| Atajo | Función |
| :--- | :--- |
| **1** | Cambiar a modo 1 imagen |
| **2** | Cambiar a modo 2 imágenes en fila |
| **3** | Cambiar a modo 3 imágenes en fila |
| **4** | Cambiar a modo 4 imágenes en fila |
| **F11** | Alternar modo pantalla completa |
| **Ctrl + R** | Alternar Modo Ultra Limpio |
| **Flecha Izquierda** | Imagen anterior |
| **Flecha Derecha** | Imagen siguiente |
| **Supr / Delete** | Mover imagen activa a la retención temporal |
| **Ctrl + Z** | Deshacer último borrado y restaurar imagen original |
| **Ctrl + E** | Alternar panel de detalles EXIF |
| **ESC** | Salir de pantalla completa / Cerrar aplicación |
