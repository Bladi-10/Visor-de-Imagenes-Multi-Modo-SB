# Visor de Imágenes Multi-Modo

**Visor de imágenes avanzado, modular y ultra rápido desarrollado en Rust 2021, GTK4, Libadwaita y Relm4.**

Diseñado especialmente para entornos Linux (Linux Mint XFCE, GNOME, Ubuntu) con integración nativa para el gestor de archivos Thunar.

---

## Características Principales

- **Distribución Multi-Imagen (1, 2, 3 y 4)**: Alterna al instante entre visualizar 1 sola imagen o hasta 4 imágenes dispuestas en una sola fila continua horizontal.
- **Modo Ultra Limpio (`Ctrl + R`)**: Oculta por completo la barra superior, botones flotantes y paneles laterales para dedicar el 100% de la pantalla al renderizado de imágenes.
- **Navegación Ágil e Instantánea (< 50ms)**: Decodificación acelerada por hardware C/GPU nativa de GTK4 (`libjpeg-turbo`/`libpng`) para una navegación fluida entre carpetas de miles de imágenes.
- **Optimización de Memoria RAM**: Sistema de ventana deslizante de memoria y reutilización en sitio de widgets que mantiene un consumo estable de RAM (estabilizado en 2.1 GB probados con 1,000 imágenes masivas).
- **Gestión de Papelera Diferida (Staging) y Deshacer (`Ctrl + Z`)**:
  - Al presionar `Delete`, las imágenes se desplazan a un directorio de retención temporal sin alterar inmediatamente la papelera del SO.
  - Presionar `Ctrl + Z` recupera instantáneamente la última imagen borrada a su ubicación e índice original.
  - Al cerrar la aplicación, un diálogo modal de confirmación permite confirmar el envío a la papelera del SO (`trash::delete`) o descartar cambios.
- **Navegación Panning con Mouse e Interacción**: Arrastre fluido con clic sostenido (`GestureDrag`) y controles de zoom (+, -, 1:1, Fit).
- **Panel Lateral de Metadatos EXIF (`Ctrl + E`)**: Muestra detalles del archivo (nombre, dimensión, peso, MIME) e información de cámara (modelo, ISO, apertura f-number).
- **Pantalla Completa Adaptativa (`F11`)**: Soporte para pantalla completa con barra superior flotante emergente al mover el cursor hacia el borde superior.

---

## Requisitos del Sistema

### Dependencias de Compilación (Debian / Ubuntu / Linux Mint)

Para compilar el proyecto en Linux, instala las librerías de desarrollo requeridas:

```bash
sudo apt update && sudo apt install -y \
    libgtk-4-dev \
    libadwaita-1-dev \
    pkg-config \
    cargo \
    rustc
```

---

## Compilación y Ejecución

### Ejecución en Modo Desarrollo

```bash
cargo run
```

### Ejecución de Pruebas Unitarias

```bash
cargo test
```

### Compilación del Binario de Producción

```bash
cargo build --release
```

El ejecutable optimizado se generará en `./target/release/herramientas-sistema`.

---

## Matriz de Atajos de Teclado

| Atajo de Teclado | Acción | Descripción |
| :--- | :--- | :--- |
| **`1`**, **`2`**, **`3`**, **`4`** | Cambiar Modo de Vista | Muestra 1, 2, 3 o 4 imágenes simultáneas en fila horizontal. |
| **`Izquierda` (`←`)** | Imagen Anterior | Navega a la imagen anterior (navegación circular). |
| **`Derecha` (`→`)** | Imagen Siguiente | Navega a la imagen siguiente (navegación circular). |
| **`F11`** | Pantalla Completa | Alterna el modo pantalla completa. |
| **`Ctrl + R`** | Modo Ultra Limpio | Oculta/muestra la barra superior y controles flotantes. |
| **`Delete` / `Supr`** | Eliminar a Staging | Mueve la imagen activa a la papelera temporal de retención. |
| **`Ctrl + Z`** | Deshacer Borrado | Restaura la última imagen eliminada a su ubicación original. |
| **`Ctrl + E`** | Panel Lateral | Muestra u oculta el panel lateral de metadatos EXIF. |
| **`ESC`** | Salir de la App | Cierra la aplicación (despliega confirmación si hay cambios retenidos). |

---

## Estructura del Código y Documentación

El proyecto incluye una completa documentación técnica en el directorio [`Document/`](Document/):

- [`Document/ciclo_de_vida_y_aprendizaje_rust_gtk4.md`](Document/ciclo_de_vida_y_aprendizaje_rust_gtk4.md): Guía pedagógica exhaustiva sobre la arquitectura MVU de Relm4, el ciclo de vida de la app y la guía de aprendizaje de Rust y GTK4.
- [`Document/arquitectura.md`](Document/arquitectura.md): Mapa arquitectónico de módulos del sistema.
- [`Document/bitacora.md`](Document/bitacora.md): Historial cronológico de desarrollo, decisiones de diseño y resolución de retroalimentaciones.
- [`Document/pruebas_unitarias.md`](Document/pruebas_unitarias.md): Reporte de la suite de pruebas unitarias automatizadas (`cargo test`).
- [`Document/manual_usuario.md`](Document/manual_usuario.md): Guía de usuario final.

---

## Licencia

Este proyecto está distribuido bajo la **Licencia MIT**, la cual es la licencia de código abierto permisiva estándar más libre e inclusiva.

Consulta el archivo [`LICENSE`](LICENSE) para obtener más detalles.
