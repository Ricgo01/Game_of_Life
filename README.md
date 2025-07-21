# 🧬 Conway's Game of Life - Rust Implementation

Una implementación completa  **Conway's Game of Life** desarrollado en **Rust** usando **Raylib** para gráficos.

![Conway's Game of Life Demo](2025-07-20%2022-50-35.gif)


### 🔬 Reglas del Juego

1. **Subpoblación**: Una célula viva con menos de 2 vecinos vivos muere
2. **Supervivencia**: Una célula viva con 2 o 3 vecinos vivos sobrevive
3. **Sobrepoblación**: Una célula viva con más de 3 vecinos vivos muere
4. **Reproducción**: Una célula muerta con exactamente 3 vecinos vivos nace

## 🛠️ Tecnologías Utilizadas

- **Rust** 🦀 - Lenguaje de programación principal
- **Raylib** 🎮 - Biblioteca de gráficos para renderizado
- **Framebuffer personalizado** - Sistema de píxeles para el Game of Life

## 🎯 Características Implementadas

### ✅ Funcionalidades Principales
- [x] Implementación completa de las 4 reglas de Conway
- [x] Sistema de framebuffer personalizado usando solo la función `point()`
- [x] Función `get_color()` para leer el estado de las células
- [x] Múltiples patrones de organismos predefinidos
- [x] Patrón inicial creativo que llena gran parte de la pantalla

### 🧬 Organismos Implementados
- **Glider** (Planeador) - Se mueve diagonalmente
- **Blinker** (Parpadeador) - Oscila entre dos estados
- **Block** (Bloque) - Vida estática, no cambia
- **Beehive** (Colmena) - Vida estática hexagonal
- **Toad** (Sapo) - Oscilador de período 2
- **Beacon** (Faro) - Oscilador de período 2
- **LWSS** (Lightweight Spaceship) - Nave espacial ligera
- **Pulsar** - Oscilador grande de período 3

## 🚀 Instalación y Ejecución

### Prerrequisitos
- Rust (versión 2021 o superior)
- Dependencias del sistema para Raylib

### Instalación en Ubuntu/Debian
```bash
# Instalar dependencias del sistema
sudo apt update
sudo apt install build-essential cmake libasound2-dev mesa-common-dev libx11-dev libxrandr-dev libxi-dev xorg-dev libgl1-mesa-dev libglu1-mesa-dev

# Clonar el repositorio
git clone https://github.com/TU_USERNAME/ventanas-conway-game-of-life.git
cd ventanas-conway-game-of-life

# Compilar y ejecutar
cargo run --release
```

### Instalación en macOS
```bash
# Instalar dependencias
brew install cmake

# Clonar y ejecutar
git clone https://github.com/TU_USERNAME/ventanas-conway-game-of-life.git
cd ventanas-conway-game-of-life
cargo run --release
```

## 📁 Estructura del Proyecto

```
src/
├── main.rs           # Punto de entrada principal
├── framebuffer.rs    # Sistema de framebuffer personalizado
├── gameoflife.rs     # Lógica del Game of Life
└── patterns.rs       # Patrones de organismos predefinidos
```

