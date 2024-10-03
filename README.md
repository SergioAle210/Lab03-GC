# Proyecto RayTracing - Simulación de Texturas y Sombras

Este proyecto es un motor de raytracing que simula un entorno 3D en el que se incluyen elementos como luces, cuboides con texturas, reflexión, refracción, sombras y animación de texturas. Se trata de una aplicación que utiliza técnicas avanzadas de gráficos por computadora para generar imágenes realistas basadas en la intersección de rayos con los objetos en la escena.

## Características del Proyecto

- **Texturas animadas**: Se incluyen texturas dinámicas que se desplazan a lo largo del tiempo, simulando efectos como el movimiento del agua.
- **Reflejos y Refracción**: Los objetos pueden tener propiedades reflectantes y transparentes, lo que permite simular efectos de reflejo en superficies y la distorsión a través de materiales transparentes.
- **Sombras dinámicas**: Las sombras se calculan en tiempo real teniendo en cuenta la posición y la intensidad de las fuentes de luz en la escena.
- **Soporte para múltiples luces**: Se implementan varias fuentes de luz, cada una con su propia posición, color e intensidad.
- **Control de cámara**: El usuario puede mover la cámara para explorar la escena desde diferentes ángulos.
- **Normal Mapping**: La opción de habilitar o deshabilitar el mapeo de normales para cambiar la apariencia de las superficies.

## Estructura del Proyecto

El proyecto está organizado en varios módulos:

- **camera.rs**: Maneja la posición y orientación de la cámara, permitiendo movimientos y rotación.
- **color.rs**: Define el sistema de color utilizado para renderizar los píxeles en la pantalla.
- **cuboid.rs**: Define los cuboides que componen los objetos 3D de la escena.
- **framebuffer.rs**: Administra el framebuffer para dibujar la escena final.
- **light.rs**: Define las propiedades de las fuentes de luz en la escena.
- **material.rs**: Define los materiales de los objetos, incluyendo propiedades como la reflectividad, transparencia y texturas.
- \*\*ray_int# Proyecto RayTracing - Simulación de Texturas y SombrasEste proyecto es un motor de raytracing que simula un entorno 3D en el que se incluyen elementos como luces, cuboides con texturas, reflexión, refracción, sombras y animación de texturas. Se trata de una aplicación que utiliza técnicas avanzadas de gráficos por computadora para generar imágenes realistas basadas en la intersección de rayos con los objetos en la escena.

  ## Características del Proyecto

  - **Texturas animadas**: Se incluyen texturas dinámicas que se desplazan a lo largo del tiempo, simulando efectos como el movimiento del agua.
  - **Reflejos y Refracción**: Los objetos pueden tener propiedades reflectantes y transparentes, lo que permite simular efectos de reflejo en superficies y la distorsión a través de materiales transparentes.
  - **Sombras dinámicas**: Las sombras se calculan en tiempo real teniendo en cuenta la posición y la intensidad de las fuentes de luz en la escena.
  - **Soporte para múltiples luces**: Se implementan varias fuentes de luz, cada una con su propia posición, color e intensidad.
  - **Control de cámara**: El usuario puede mover la cámara para explorar la escena desde diferentes ángulos.
  - **Normal Mapping**: La opción de habilitar o deshabilitar el mapeo de normales para cambiar la apariencia de las superficies.

  ## Estructura del Proyecto

  El proyecto está organizado en varios módulos:

  - **camera.rs**: Maneja la posición y orientación de la cámara, permitiendo movimientos y rotación.
  - **color.rs**: Define el sistema de color utilizado para renderizar los píxeles en la pantalla.
  - **cuboid.rs**: Define los cuboides que componen los objetos 3D de la escena.
  - **framebuffer.rs**: Administra el framebuffer para dibujar la escena final.
  - **light.rs**: Define las propiedades de las fuentes de luz en la escena.
  - **material.rs**: Define los materiales de los objetos, incluyendo propiedades como la reflectividad, transparencia y texturas.
  - **ray_intersect.rs**: Contiene las funciones para calcular las intersecciones de rayos con los objetos de la escena.
  - **texture.rs**: Administra las texturas que se aplican a las superficies de los objetos, incluyendo texturas animadas.

## Gameplay

[![Watch the video](./assets/Minecraft.jpg)](https://youtu.be/vBHp8V7ZvEk)

## Requisitos del Sistema

- **Rust**: El proyecto está escrito en Rust, por lo que necesitarás tener instalado Rust en tu sistema. Puedes descargarlo desde [aquí](https://www.rust-lang.org/).
- **minifb**: Utilizamos la biblioteca `minifb` para la creación de la ventana gráfica. Se instalará automáticamente con los comandos de abajo.

## Instrucciones para Ejecutar el Proyecto

Para ejecutar el proyecto en tu sistema local, sigue estos pasos:

### Clonar el Repositorio

```bash
git clone <url-del-repositorio>
cd <nombre-del-repositorio>ersect.rs**: Contiene las funciones para calcular las intersecciones de rayos con los objetos de la escena.
```

- **texture.rs**: Administra las texturas que se aplican a las superficies de los objetos, incluyendo texturas animadas.

## Requisitos del Sistema

- **Rust**: El proyecto está escrito en Rust, por lo que necesitarás tener instalado Rust en tu sistema. Puedes descargarlo desde [aquí](https://www.rust-lang.org/).
- **minifb**: Utilizamos la biblioteca `minifb` para la creación de la ventana gráfica. Se instalará automáticamente con los comandos de abajo.

## Instrucciones para Ejecutar el Proyecto

Para ejecutar el proyecto en tu sistema local, sigue estos pasos:

### Clonar el Repositorio

```bash
git clone <url-del-repositorio>
cd <nombre-del-repositorio>
```

### Instalar Dependencias

```bash
cargo build
```

Esto descargará e instalará las dependencias necesarias.

### Ejecutar el Proyecto

- #### Opción 1: Ejecutar con Cargo (en cualquier sistema)

  Para ejecutar el proyecto en modo release (óptimo para rendimiento), usa el siguiente comando:

  ```bash
  cargo run --release
  ```

- #### Opción 2: Ejecutar con un script (en Windows)

  Si estás en Windows, puedes usar el siguiente comando para ejecutar el proyecto usando un script .bat preconfigurado:

  ```bash
  ./run_project.bat
  ```

### Controles

- **Teclas de dirección (← ↑ ↓ →)**: Rotar la cámara alrededor de la escena.
- **Teclas W/S**: Acercar o alejar la cámara (zoom).
- **Tecla M**: Alternar entre mapeo de normales y texturas.
- **Teclas A/D**: Girar las luces para simular un ciclo de día y noche.

### Notas adicionales

- Asegúrate de que las texturas necesarias (por ejemplo, `WATER.jpg`, `ladrillos.png`, etc.) estén en la carpeta `assets` del proyecto.
- Si experimentas problemas de rendimiento, prueba a reducir la resolución de la ventana en el archivo `main.rs`.

# Autor

Este proyecto fue desarrollado como una simulación gráfica utilizando técnicas de raytracing para representar una escena 3D con texturas y efectos visuales realistas.
