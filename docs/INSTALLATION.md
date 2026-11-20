# Instalación de AmilcarCore-RS

AmilcarCore-RS se compila localmente con Rust. No requiere una API externa ni un servidor remoto para procesar archivos. La primera instalación necesita conexión para obtener el código y las dependencias de Cargo.

## Requisitos

- Rust y Cargo **1.75 o superior**.
- Git para clonar el repositorio.
- Espacio libre para dependencias y compilación.

Comprueba las herramientas:

\`\`\`sh
rustc --version
cargo --version
git --version
\`\`\`

## Android con Termux

**1. Actualiza los paquetes e instala Git y Rust**

\`\`\`sh
pkg update && pkg upgrade
pkg install git rust
\`\`\`

**2. Comprueba las versiones**

\`\`\`sh
rustc --version
cargo --version
\`\`\`

Rust debe ser 1.75 o superior. Si la versión es anterior, actualiza tu toolchain antes de continuar.

**3. Descarga el repositorio**

\`\`\`sh
git clone https://github.com/AmilcarGit/AmilcarCore-RS.git
cd AmilcarCore-RS
\`\`\`

**4. Compila e instala el CLI**

\`\`\`sh
sh scripts/install.sh
\`\`\`

El instalador compila en modo release e instala el ejecutable en \`$HOME/.local/bin/amilcar\`.

**5. Configura el PATH si es necesario**

\`\`\`sh
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
\`\`\`

**6. Verifica la instalación**

\`\`\`sh
amilcar --version
amilcar info
amilcar --help
amilcar text --help
amilcar json --help
amilcar csv --help
\`\`\`

**7. Ejecuta una prueba local**

\`\`\`sh
mkdir -p ~/amilcar-prueba
cd ~/amilcar-prueba
echo "Hola mundo" > notas.txt
echo '{"activo":true}' > datos.json
printf 'nombre,edad\nAna,20\n' > personas.csv

amilcar text stats notas.txt
amilcar json validate datos.json
amilcar json format datos.json datos-formateados.json
amilcar csv inspect personas.csv
amilcar csv select personas.csv nombres.csv --columns nombre
amilcar file hash datos.json
\`\`\`

**8. Usa salida estructurada para scripts**

\`\`\`sh
amilcar --json text stats notas.txt
amilcar --json file inspect datos.json
\`\`\`

## Actualizar una instalación existente en Termux

Si ya clonaste el repositorio y solo quieres instalar la versión más reciente de \`main\`, no repitas el clonado:

\`\`\`sh
cd ~/AmilcarCore-RS
git pull --ff-only origin main
sh scripts/install.sh
hash -r
amilcar --version
amilcar info
\`\`\`

\`git pull\` actualiza el código fuente. \`sh scripts/install.sh\` vuelve a compilar e instala el ejecutable actualizado en \`$HOME/.local/bin/amilcar\`.

## Linux

### Debian, Ubuntu y derivados

\`\`\`sh
sudo apt update
sudo apt install -y git build-essential curl
\`\`\`

Instala Rust estable si la versión del sistema es anterior a 1.75:

\`\`\`sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
rustup update stable
\`\`\`

### Fedora

\`\`\`sh
sudo dnf install -y git gcc
\`\`\`

### Arch Linux y derivados

\`\`\`sh
sudo pacman -Syu --needed git base-devel rust
\`\`\`

Verifica que Rust sea 1.75 o superior. En caso contrario, usa rustup.

### Clonar, compilar e instalar

\`\`\`sh
git clone https://github.com/AmilcarGit/AmilcarCore-RS.git
cd AmilcarCore-RS
sh scripts/install.sh
\`\`\`

Comprueba el ejecutable:

\`\`\`sh
amilcar --version
amilcar info
amilcar --help
\`\`\`

Asegúrate de que \`$HOME/.local/bin\` esté en el PATH.

## Instalación manual desde el código fuente

\`\`\`sh
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo install --path crates/amilcar-cli --root "$HOME/.local" --force
\`\`\`

## Comandos principales

\`\`\`sh
amilcar info
amilcar file inspect ./documento.txt
amilcar file diff ./original.txt ./copia.txt
amilcar file find ./documentos --extension json
amilcar file hash ./documento.txt
amilcar file duplicates ./documentos

amilcar text stats ./notas.txt
amilcar text normalize ./entrada.txt ./salida.txt

amilcar json validate ./datos.json
amilcar json format ./datos.json ./formateado.json
amilcar json format ./datos.json ./compacto.json --compact

amilcar csv validate ./personas.csv
amilcar csv inspect ./personas.csv
amilcar csv format ./personas.csv ./limpio.csv
amilcar csv select ./personas.csv ./seleccion.csv --columns nombre,edad

amilcar --json file inspect ./documento.txt
\`\`\`

Los comandos de texto, JSON y CSV leen un máximo de 64 MiB por archivo. Las transformaciones crean un destino nuevo de forma predeterminada. Para reemplazar un archivo existente, utiliza \`--overwrite\` explícitamente.

\`file diff\` devuelve código de salida 2 cuando detecta diferencias; los errores de ejecución devuelven código 1.

## Benchmarks

Desde el repositorio:

\`\`\`sh
cargo bench --workspace
\`\`\`

Las mediciones dependen del dispositivo y no representan garantías de rendimiento.

## Desinstalar

Desde el repositorio:

\`\`\`sh
sh scripts/uninstall.sh
\`\`\`

La desinstalación no elimina el código fuente ni la caché de Cargo.

## Problemas frecuentes

- **\`cargo: command not found\`:** instala Rust/Cargo y vuelve a abrir la terminal o carga \`$HOME/.cargo/env\`.
- **Rust demasiado antiguo:** actualiza Rust estable y comprueba la versión.
- **\`amilcar: command not found\`:** añade \`$HOME/.local/bin\` al PATH y verifica que exista el ejecutable.
- **Destino ya existente:** vuelve a ejecutar el comando con \`--overwrite\` solo cuando aceptes reemplazarlo.
- **Termux se queda sin memoria:** cierra otras aplicaciones y vuelve a ejecutar la compilación.

## Alcance Android

Termux compila el programa en el propio dispositivo. La CI realiza una comprobación de compilación para \`aarch64-linux-android\`, pero eso no sustituye una prueba en el teléfono. No se distribuyen APK ni binarios Android precompilados.
