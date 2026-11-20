# Instalación de AmilcarCore-RS

AmilcarCore-RS se compila localmente con Rust. No requiere una API externa ni un servidor remoto. La primera compilación necesita conexión para obtener el código fuente y las dependencias de Cargo; después, Cargo puede reutilizar su caché local.

## Requisitos

- Rust y Cargo **1.75 o superior**.
- Git, para clonar el repositorio.
- Espacio libre para descargar dependencias y compilar.
- En Linux, las herramientas de compilación que requiera el paquete de Rust de tu distribución.

Comprueba los requisitos:

```sh
rustc --version
cargo --version
git --version
```

## Android con Termux

Instala Termux desde una fuente mantenida y abre la terminal:

```sh
pkg update && pkg upgrade
pkg install git rust
```

Comprueba la versión de Rust. Debe ser 1.75 o superior:

```sh
rustc --version
cargo --version
```

Si tu paquete de Rust es anterior a 1.75, actualiza Rust con rustup siguiendo su documentación oficial antes de continuar.

Clona e instala:

```sh
git clone https://github.com/AmilcarGit/AmilcarCore-RS.git
cd AmilcarCore-RS
sh scripts/install.sh
```

El instalador coloca el ejecutable en `$HOME/.local/bin/amilcar`. Si Termux indica que la ruta no está en PATH, ejecuta:

```sh
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

Verifica:

```sh
amilcar --version
amilcar info
amilcar --help
```

> Termux usa Android. La compilación se realiza en el propio dispositivo y depende de que la versión de Rust disponible soporte la arquitectura y el entorno del teléfono. No se distribuye todavía un APK ni un binario Android precompilado.

## Linux

### Debian, Ubuntu y derivados

```sh
sudo apt update
sudo apt install -y git build-essential curl
```

Instala Rust estable con rustup si tu distribución no ofrece Rust 1.75 o superior:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
rustup update stable
```

### Fedora

```sh
sudo dnf install -y git gcc
```

Instala Rust estable con rustup si el paquete de la distribución no cumple la versión mínima.

### Arch Linux y derivados

```sh
sudo pacman -Syu --needed git base-devel rust
```

Comprueba que `rustc --version` sea 1.75 o superior. Si no, utiliza rustup.

### Compilar e instalar

```sh
git clone https://github.com/AmilcarGit/AmilcarCore-RS.git
cd AmilcarCore-RS
sh scripts/install.sh
```

Verifica:

```sh
amilcar --version
amilcar info
amilcar --help
```

## Instalación manual desde el código fuente

Si ya tienes el repositorio y Rust:

```sh
cargo check --workspace
cargo test --workspace
cargo install --path crates/amilcar-cli --root "$HOME/.local" --force
```

Asegúrate de que `$HOME/.local/bin` esté en tu PATH.

## Ejemplos de uso

```sh
# Información del motor
amilcar info

# Inspeccionar un archivo o directorio
amilcar file inspect ./documento.txt

# Comparar dos archivos
amilcar file diff ./original.txt ./copia.txt

# Buscar archivos JSON recursivamente
amilcar file find ./documentos --extension json

# Buscar nombres que contengan "config"
amilcar file find ./proyecto --name config
```

El comando `file diff` devuelve código de salida 2 cuando detecta diferencias. Los errores de ejecución devuelven código 1.

## Desinstalar

Desde el repositorio:

```sh
sh scripts/uninstall.sh
```

También puedes borrar manualmente `$HOME/.local/bin/amilcar`. La desinstalación no borra el código fuente ni la caché de Cargo.

## Problemas frecuentes

- **`cargo: command not found`:** instala Rust/Cargo y abre una terminal nueva o carga `$HOME/.cargo/env`.
- **Rust demasiado antiguo:** ejecuta `rustup update stable` y comprueba con `rustc --version`.
- **`amilcar: command not found`:** añade `$HOME/.local/bin` al PATH.
- **Error al compilar dependencias nativas:** instala el compilador y las herramientas de desarrollo de tu distribución.
- **Termux sin memoria durante la compilación:** cierra otras aplicaciones y vuelve a ejecutar la compilación; evita procesos paralelos pesados.

## Alcance actual

El instalador compila el CLI que existe hoy. No instala una interfaz gráfica, no crea un servicio en segundo plano y no descarga modelos. Las funciones disponibles se limitan a los comandos documentados en el README.
