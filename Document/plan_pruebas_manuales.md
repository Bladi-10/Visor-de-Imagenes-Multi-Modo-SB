# Plan de Pruebas Manuales - Visor de Imágenes (Retroalimentación 4)

Este documento contiene la guía paso a paso para que el usuario o tester ejecute manualmente la comprobación funcional y de rendimiento de todos los requerimientos y correcciones implementadas en la actualización **Retro 4**.

---

## 1. Prueba de Carga Peresoza y Liberación de Memoria RAM

### Objetivo:
Verificar que la memoria RAM se mantiene estable y no aumenta indefinidamente al inspeccionar una gran cantidad de imágenes de alta resolución secuencialmente.

### Pasos a ejecutar:
1. Abra una terminal en su sistema y ejecute un monitor de sistema (por ejemplo `htop` o `top -p $(pgrep herramientas-sistema)`).
2. Inicie el visor de imágenes con `cargo run`.
3. Presione el botón **Abrir Archivo de Imagen** y seleccione una fotografía dentro de una carpeta que contenga 50 o más imágenes de alta resolución.
4. Con la tecla de dirección **Derecha**, comience a avanzar una por una por todas las imágenes a un ritmo constante (mantenga presionada la tecla o avance continuamente).
5. Observe en el monitor de procesos (`htop`) la columna de consumo de memoria RAM del proceso `herramientas-sistema`.

### Criterio de Éxito:
- El consumo de memoria RAM se mantiene estable en un nivel bajo/constante (solo mantiene 3 imágenes en memoria en modo 1) y **NO va en aumento continuo** ni genera fuga de memoria al recorrer 50 o más fotos.

---

## 2. Prueba del Modo Ultra Limpio (Ctrl + R) e Independencia de Pantalla Completa (F11)

### Objetivo:
Comprobar que `Ctrl+R` oculta la UI sin forzar el modo pantalla completa, y que en pantalla completa `F11` los botones de minimizar/maximizar se ocultan adecuadamente.

### Pasos a ejecutar:
1. Abra el visor de imágenes en modo ventana normal.
2. Presione **Ctrl + R**:
   - Verifique que la barra superior, las flechas flotantes `<` `>` y los paneles desaparecen.
   - Verifique que la ventana **permanece en modo ventana normal** (NO se maximizó ni se fue a pantalla completa F11).
3. Presione **Ctrl + R** nuevamente:
   - Verifique que la barra superior y los controles de UI vuelven a aparecer.
4. Presione **F11**:
   - Verifique que la ventana entra en **Modo Pantalla Completa**.
   - Mueva el cursor del mouse a la parte superior de la pantalla para desplegar la barra de título flotante.
   - Compruebe que los íconos de **Minimizar** y **Maximizar** están ocultos y **solo se muestra el botón de Cerrar**.
5. Con la pantalla completa activa, presione **Ctrl + R**:
   - Verifique que el Modo Ultra Limpio funciona dentro de pantalla completa (ocultando los controles flotantes).
6. Presione **F11** para salir de pantalla completa:
   - Verifique que la ventana regresa a su estado normal y los íconos de minimizar/maximizar vuelven a ser visibles.

### Criterio de Éxito:
- `Ctrl+R` no altera el estado de pantalla completa F11.
- En pantalla completa F11 solo permanece el ícono de cerrar en la barra flotante.

---

## 3. Prueba de Gestión de Papelera y Deshacer con Ctrl + Z (Sin duplicados en el SO)

### Objetivo:
Confirmar que al borrar una imagen con `Delete`, presionar `Ctrl+Z` la restaura inmediatamente sin mensaje de error y sin dejar duplicados acumulados en la papelera del SO.

### Pasos a ejecutar:
1. Abra el visor de imágenes y cargue un directorio de prueba.
2. Anote el nombre de la imagen activa en pantalla.
3. Presione la tecla **Delete** / **Supr**:
   - Verifique que la imagen desaparece de la vista y el visor pasa a la siguiente imagen.
4. Presione **Ctrl + Z**:
   - Verifique que la imagen borrada **reaparece de inmediato renderizada** en pantalla (sin quedar en blanco ni mostrar el mensaje *"Cargando vista previa..."*).
5. Abra el gestor de archivos nativo de su sistema (Thunar/Nautilus) y abra la **Papelera del Sistema**:
   - Compruebe que la imagen restaurada con `Ctrl+Z` **NO está en la papelera del SO** (quedó limpia sin duplicados).
6. Regrese al visor, borre una imagen con **Delete** y **cierre el aplicativo**:
   - Abra la Papelera del SO en Thunar y verifique que la imagen eliminada se encuentra allí guardada de forma definitiva.

### Criterio de Éxito:
- `Ctrl+Z` restaura la imagen al instante sin errores visuales y la papelera del SO no acumula archivos duplicados no deseados.

---

## 4. Prueba de Remoción de Borrado Permanente (`Ctrl + Delete`)

### Objetivo:
Verificar que el atajo `Ctrl+Delete` y los diálogos de borrado permanente han sido totalmente retirados.

### Pasos a ejecutar:
1. En el visor de imágenes, presione la combinación de teclas **Ctrl + Delete** (o `Ctrl + Supr`).
2. Compruebe que no aparece ningún diálogo modal de confirmación permanente y que la imagen simplemente se procesa mediante la retención temporal de la papelera normal.

### Criterio de Éxito:
- La combinación `Ctrl + Delete` no abre diálogos de borrado permanente ni elimina archivos fuera de la papelera.

---

## 5. Prueba de Apertura Única de Imagen y Carga Automática de Directorio

### Objetivo:
Validar la eliminación del botón "Abrir Carpeta" y comprobar que seleccionar un archivo de imagen carga automáticamente todas las imágenes vecinas de esa carpeta.

### Pasos a ejecutar:
1. Observe la barra de herramientas superior (HeaderBar):
   - Confirme que el botón de **Abrir Carpeta** fue removido y solo existe el botón **Abrir Archivo de Imagen**.
2. Haga clic en **Abrir Archivo de Imagen**.
3. Navegue a una carpeta que contenga 5 imágenes (ejemplo: `foto1.jpg`, `foto2.jpg`, `foto3.jpg`, `foto4.jpg`, `foto5.jpg`) y seleccione `foto3.jpg`.
4. Observe la pantalla:
   - Verifique que la vista muestra `foto3.jpg`.
5. Presione la tecla de flecha **Derecha**:
   - Verifique que pasa a `foto4.jpg` y luego a `foto5.jpg`.
6. Presione la tecla de flecha **Izquierda**:
   - Verifique que navega hacia atrás pasando por `foto2.jpg` y `foto1.jpg`.

### Criterio de Éxito:
- Al abrir una sola imagen, el visor carga automáticamente todas las imágenes del mismo directorio.

---

## 6. Prueba de Navegación Ágil e Instantánea en el Selector de Archivos

### Objetivo:
Comprobar que la navegación entre directorios dentro de la ventana de selección de archivos nativa es instantánea y sin demoras.

### Pasos a ejecutar:
1. Haga clic en **Abrir Archivo de Imagen**.
2. En la ventana de diálogo que se despliega, navegue rápidamente entre diferentes carpetas de su sistema operativo que contengan muchas imágenes y subcarpetas.
3. Observe la velocidad con la que la ventana del sistema cambia de directorio al hacer doble clic en las carpetas.

### Criterio de Éxito:
- El cambio de directorio en el selector de archivos es instantáneo y fluido, idéntico al comportamiento nativo de Thunar/Nautilus.

---

## 7. Prueba de Renderizado Inmediato al Alternar Modos de Vista (`1, 2, 3, 4`)

### Objetivo:
Comprobar que al cambiar el modo de vista mediante las teclas de atajo `1`, `2`, `3` o `4` (o los botones de la barra superior), las nuevas imágenes visibles se renderizan **inmediatamente** sin requerir presionar las teclas de dirección.

### Pasos a ejecutar:
1. Abra el visor de imágenes con un catálogo de al menos 10 imágenes.
2. Inicie en modo **1** (1 imagen).
3. Presione la tecla **3**:
   - Verifique que las 3 imágenes en fila se renderizan al instante en la pantalla sin mostrar avisos de *"Cargando vista previa..."*.
4. Presione la tecla **4**:
   - Verifique que la cuarta imagen se suma y renderiza de inmediato en la fila continua.
5. Presione la tecla **2** y luego **1**:
   - Compruebe la respuesta instantánea del renderizado.

### Criterio de Éxito:
- El cambio de modo renderiza inmediatamente todas las imágenes de los nuevos slots sin necesidad de presionar `<-` o `->`.

---

## 8. Prueba del Diálogo de Confirmación de Cierre con Verificación (`Punto 5`)

### Objetivo:
Verificar que al cerrar la aplicación con imágenes en retención temporal, se despliega el diálogo de confirmación y se validan las opciones de borrado definitivo y restauración a carpeta de origen.

### Pasos a ejecutar:
1. **Prueba 8A (Descartar Cambios y Salir):**
   - Abra el visor y cargue una carpeta con imágenes de prueba.
   - Elimine 2 imágenes con la tecla **Delete / Supr**.
   - Presione **ESC** o haga clic en el botón **X** de la ventana.
   - En el diálogo modal desplegado, seleccione **"Descartar Cambios y Salir"**.
   - Abra el gestor de archivos nativo y compruebe que las 2 imágenes eliminadas **han regresado intactas a su carpeta original**.
2. **Prueba 8B (Enviar a Papelera y Salir con Verificación):**
   - Abra el visor nuevamente y elimine 1 imagen con **Delete**.
   - Haga clic en el botón **X** de la ventana.
   - Seleccione **"Enviar a Papelera y Salir"**.
   - Verifique que el visor valida el éxito del borrado, deja limpia la carpeta de staging temporal y se cierra cleanly.
   - Abra la Papelera del SO y confirme que la imagen se encuentra guardada allí.

### Criterio de Éxito:
- El diálogo modal se dispara correctamente tanto por `ESC` como por la `X` de la ventana.
- "Descartar Cambios y Salir" devuelve las imágenes verificadas a su carpeta de origen.
- "Enviar a Papelera y Salir" transfiere los archivos verificadamente a la Papelera del SO.

