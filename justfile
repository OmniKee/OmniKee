alias b := build
alias d := dev-www
alias dt := dev-tauri

# full build of Web and Tauri app
build: build-lib build-www build-tauri

# check the code without compiling
check: check-lib check-www check-tauri

# build the WebAssembly core
[working-directory('lib')]
build-lib $RUSTFLAGS='--cfg getrandom_backend="wasm_js"':
    wasm-pack build -t web

# build the Vue frontend
[working-directory('www')]
build-www: build-lib
    npm install
    npm run build

# build the Tauri app
[working-directory('tauri')]
build-tauri $NO_STRIP='true': build-lib
    cargo tauri build

# build the Android app
[working-directory('tauri')]
build-android: build-lib
    cargo tauri android build

# check the core
[working-directory('lib')]
check-lib:
    cargo check

# lint the Vue frontend
[working-directory('www')]
check-www:
    npm run lint

# check the core
[working-directory('tauri')]
check-tauri:
    cargo check

# develop the web app with live reloading
[working-directory('www')]
dev-www: build-lib
    npm install
    npm run dev

# develop the Tauri app with live reloading
[working-directory('tauri')]
dev-tauri: build-lib
    cargo tauri dev

# develop on Android
[working-directory('tauri')]
dev-android: build-lib
    cargo tauri android dev
