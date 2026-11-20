<div align="center">
  <img src="docs/assets/architecture.svg" alt="AmilcarCore-RS architecture" width="100%">
  <h1>AmilcarCore-RS</h1>
  <p><strong>Motor nativo de procesamiento local · Rust</strong></p>
  <p>Biblioteca modular y CLI para inspección y procesamiento de archivos, con enfoque local-first.</p>
  <p><a href="docs/guide-es.html">Guía visual en español</a> · <a href="docs/guide-en.html">English visual guide</a> · <a href="docs/INSTALLATION.md">Instalación</a></p>
  <p>
    <img src="https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust" alt="Rust 1.75+">
    <img src="https://img.shields.io/badge/Version-0.1.0-blue" alt="Version 0.1.0">
    <img src="https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-green" alt="MIT OR Apache-2.0">
    <img src="https://img.shields.io/badge/Platforms-Linux%20%7C%20Windows%20%7C%20macOS%20%7C%20Termux-informational" alt="Platforms">
    <a href="https://github.com/AmilcarGit/AmilcarCore-RS/actions/workflows/ci.yml"><img src="https://github.com/AmilcarGit/AmilcarCore-RS/actions/workflows/ci.yml/badge.svg?branch=main" alt="Rust CI"></a>
    <img src="https://img.shields.io/badge/Status-Early%20development-yellow" alt="Early development">
  </p>
</div>

**[🇪🇸 Español](#español) · [🇬🇧 English](#english)**

---

# Español

## ¿Qué es AmilcarCore-RS?

AmilcarCore-RS es un workspace modular escrito en Rust. Ofrece una biblioteca reutilizable y una interfaz de línea de comandos para inspeccionar, comparar y buscar archivos localmente.

> **Estado:** desarrollo temprano · Etapa 2 — base del sistema de archivos. Las API pueden cambiar antes de una versión estable.

### Características disponibles

- Inspección de metadatos de archivos y directorios.
- Comparación de archivos por bloques y detección del primer byte diferente.
- Búsqueda recursiva por extensión o fragmento del nombre.
- Lectura y procesamiento con límites configurables.
- Escrituras explícitas: crear sin sobrescribir o sobrescribir solo cuando se solicita.
- Ejecución local para las operaciones core documentadas.

### Versiones y compatibilidad

| Elemento | Valor |
| --- | --- |
| Versión del workspace | <code>0.1.0</code> |
| Rust mínimo (MSRV) | <code>1.75</code> |
| Edition | Rust 2021 |
| Licencia | <code>MIT OR Apache-2.0</code> |
| Plataformas CI | Linux, Windows y macOS |
| Android / Termux | Compilación local sujeta a la toolchain y arquitectura disponibles |
| Estado | Desarrollo temprano |

La insignia de CI muestra el estado del workflow de GitHub Actions en <code>main</code>. La compatibilidad Android/Termux no significa que existan APK o binarios precompilados.

## Arquitectura del workspace

![Diagrama de arquitectura](docs/assets/architecture.svg)

| Crate | Responsabilidad |
| --- | --- |
| <code>amilcar-core</code> | API pública, motor, configuración y errores compartidos |
| <code>amilcar-fs</code> | Operaciones del sistema de archivos |
| <code>amilcar-data</code> | Procesamiento de texto y datos estructurados; en desarrollo |
| <code>amilcar-cli</code> | Interfaz de terminal que consume la biblioteca |

### Flujo de ejecución

![Flujo de ejecución](docs/assets/execution-flow.svg)

~~~mermaid
flowchart TD
    U[Usuario / Terminal] --> CLI[amilcar-cli]
    CLI --> CORE[amilcar-core]
    CORE --> FS[amilcar-fs]
    CORE --> DATA[amilcar-data]
    FS --> IO[Operaciones locales de archivos]
    DATA --> PROC[Procesamiento de datos]
~~~

El CLI actúa como adaptador. La lógica reutilizable pertenece a las bibliotecas y no se duplica en la interfaz de terminal.

## Instalación en Termux y Linux

Requisitos: Git, Rust y Cargo 1.75 o superior.

~~~sh
git clone https://github.com/AmilcarGit/AmilcarCore-RS.git
cd AmilcarCore-RS
sh scripts/install.sh
amilcar info
~~~

- [Guía visual HTML en español](docs/guide-es.html)
- [English HTML guide](docs/guide-en.html)
- [Manual completo para Termux y distribuciones Linux](docs/INSTALLATION.md)

El instalador compila el CLI desde el código fuente y lo coloca en <code>$HOME/.local/bin</code> por defecto.

## Demostración de terminal

![Demostración animada de AmilcarCore-RS](docs/assets/terminal-demo.svg)

La animación muestra la salida real del comando <code>amilcar info</code>. Para inspeccionar tus propios archivos:

~~~sh
amilcar file inspect ./documento.txt
~~~

El tamaño y los metadatos mostrados dependen del archivo local.

## Ejemplos de comandos

~~~sh
# Información del motor
amilcar info

# Inspeccionar metadatos
amilcar file inspect ./documento.txt

# Comparar dos archivos
amilcar file diff ./original.txt ./copia.txt

# Buscar JSON recursivamente
amilcar file find ./documentos --extension json

# Buscar por fragmento del nombre
amilcar file find ./proyecto --name config
~~~

## Seguridad y diseño

- No se requieren APIs externas ni servidor remoto para las operaciones core documentadas.
- No se siguen enlaces simbólicos durante la búsqueda recursiva.
- Los límites de entradas y tamaños reducen el riesgo de consumir recursos sin control.
- La escritura no sobrescribe archivos de forma implícita.
- Se evita <code>unsafe</code> salvo que exista una justificación técnica documentada.

## Hoja de ruta

El procesamiento ampliado de texto, JSON/CSV, más comandos, benchmarks y verificación completa en plataformas adicionales siguen en desarrollo. No se presentan como funciones terminadas.

## Licencia

Puedes elegir entre [MIT](LICENSE-MIT) o [Apache License 2.0](LICENSE-APACHE).

---

# English

## What is AmilcarCore-RS?

AmilcarCore-RS is a modular Rust workspace. It provides a reusable library and a command-line interface for local file inspection, comparison and search.

> **Status:** early development · Stage 2 — filesystem foundation. APIs may change before a stable release.

### Available features

- File and directory metadata inspection.
- Buffered file comparison and first differing byte detection.
- Recursive search by extension or filename substring.
- Bounded reading and processing primitives.
- Explicit write modes: create without replacement, or overwrite only when requested.
- Local execution for the documented core operations.

### Version and compatibility

| Item | Value |
| --- | --- |
| Workspace version | <code>0.1.0</code> |
| Minimum Rust version (MSRV) | <code>1.75</code> |
| Edition | Rust 2021 |
| License | <code>MIT OR Apache-2.0</code> |
| CI platforms | Linux, Windows and macOS |
| Android / Termux | Local compilation depends on the available toolchain and architecture |
| Status | Early development |

The CI badge reflects the GitHub Actions workflow on <code>main</code>. Android/Termux support does not mean prebuilt APKs or Android binaries are currently published.

## Workspace architecture

![Workspace architecture diagram](docs/assets/architecture.svg)

| Crate | Responsibility |
| --- | --- |
| <code>amilcar-core</code> | Public API, engine, configuration and shared errors |
| <code>amilcar-fs</code> | Filesystem operations |
| <code>amilcar-data</code> | Text and structured-data processing; in development |
| <code>amilcar-cli</code> | Terminal interface built on the library |

### Execution flow

![Execution flow diagram](docs/assets/execution-flow.svg)

~~~mermaid
flowchart TD
    U[User / Terminal] --> CLI[amilcar-cli]
    CLI --> CORE[amilcar-core]
    CORE --> FS[amilcar-fs]
    CORE --> DATA[amilcar-data]
    FS --> IO[Local filesystem operations]
    DATA --> PROC[Data processing]
~~~

The CLI is an adapter. Reusable logic belongs in the libraries rather than being duplicated in the terminal interface.

## Install on Termux and Linux

Requirements: Git, Rust and Cargo 1.75 or newer.

~~~sh
git clone https://github.com/AmilcarGit/AmilcarCore-RS.git
cd AmilcarCore-RS
sh scripts/install.sh
amilcar info
~~~

- [Spanish visual HTML guide](docs/guide-es.html)
- [English visual HTML guide](docs/guide-en.html)
- [Complete Termux and Linux installation manual](docs/INSTALLATION.md)

The installer builds the CLI from source and installs it into <code>$HOME/.local/bin</code> by default.

## Terminal demonstration

![Animated AmilcarCore-RS terminal demo](docs/assets/terminal-demo.svg)

The animation shows the actual output of <code>amilcar info</code>. To inspect your own files:

~~~sh
amilcar file inspect ./document.txt
~~~

Reported size and metadata depend on the local file.

## Command examples

~~~sh
# Engine information
amilcar info

# Inspect metadata
amilcar file inspect ./document.txt

# Compare files
amilcar file diff ./original.txt ./copy.txt

# Recursively find JSON files
amilcar file find ./documents --extension json

# Search by filename substring
amilcar file find ./project --name config
~~~

## Security and design

- No external API or remote server is required for the documented core operations.
- Recursive search does not follow symbolic links.
- Entry and size limits help control resource consumption.
- File writes never overwrite implicitly.
- <code>unsafe</code> is avoided unless technically justified and documented.

## Roadmap

Expanded text, JSON/CSV processing, additional commands, benchmarks and broader platform verification are still in development. They are not presented as completed features.

## License

Choose either the [MIT License](LICENSE-MIT) or [Apache License 2.0](LICENSE-APACHE).

---

<div align="center">
  <sub>Built with Rust · Maintained by <a href="https://github.com/AmilcarGit">AmilcarGit</a></sub>
</div>
