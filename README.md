# Hello GL

Dette projekt er et Rust-baseret program, der bruger `glfw` og `gl` til at skabe et vindue og rendere grafik. Følg denne guide for at opsætte Rust og køre projektet.

## Krav

For at køre dette projekt skal du have følgende software installeret:

1. **Rust**: Rust-programmeringssproget og værktøjer.
2. **CMake**: Bruges til at bygge `glfw-sys` biblioteket.
3. **Visual Studio med C++ værktøjer** (Windows): Bruges som compiler.
4. **GLFW-biblioteker** (Linux): Til vindueshåndtering og OpenGL-integration.

## Opsætning

### 1. Installer Rust
Rust kan installeres ved hjælp af værktøjet `rustup`. Kør følgende kommando i din terminal:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Følg instruktionerne på skærmen for at fuldføre installationen. Når installationen er færdig, skal du sikre dig, at `cargo` og `rustc` er tilgængelige i din terminal:

```sh
cargo --version
rustc --version
```

### 2. Installer CMake
CMake bruges til at bygge `glfw-sys`. Du kan downloade og installere det fra [CMake's officielle hjemmeside](https://cmake.org/download/).

På Linux kan du installere det via din pakkehåndtering:

```sh
sudo apt-get install cmake
```

### 3. Installer Visual Studio (Windows)
Hvis du bruger Windows, skal du installere Visual Studio med "Desktop development with C++"-arbejdsbelastningen. Sørg for, at `cl.exe` er tilgængelig i din PATH.

### 4. Installer GLFW (Linux)
På Linux skal du installere GLFW og dets afhængigheder:

```sh
sudo apt-get update
sudo apt-get install -y libglfw3 libglfw3-dev libx11-dev libxrandr-dev libxi-dev libxxf86vm-dev libxinerama-dev libxcursor-dev
```

### 5. Klon projektet
Klon dette repository til din lokale maskine:

```sh
git clone https://github.com/KingoBoiii/hello_gl.git
cd hello_gl
```

### 6. Byg projektet
Byg projektet med `cargo`:

```sh
cargo build --release
```

### 7. Kør projektet
Kør projektet med følgende kommando:

```sh
cargo run
```

## Test
For at køre testene i projektet skal du bruge:

```sh
cargo test
```

## CI/CD
Dette projekt bruger GitHub Actions til kontinuerlig integration. Workflowet er defineret i [`.github/workflows/rust.yml`](.github/workflows/rust.yml). Det bygger projektet og kører testene på hver push eller pull request til `master`-branchen.

## Licens
Dette projekt er licenseret under MIT-licensen. Se `LICENSE`-filen for detaljer.