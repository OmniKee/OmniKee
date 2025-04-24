declare namespace NodeJS {
  interface ProcessEnv {
    NODE_ENV: string;
    TAURI_ENV_PLATFORM: 'windows' | 'darwin' | 'linux' | 'web' | undefined;
    VUE_ROUTER_MODE: 'hash' | 'history' | 'abstract' | undefined;
    VUE_ROUTER_BASE: string | undefined;
  }
}

/// <reference types="vite/client" />
/// <reference types="unplugin-vue-router/client" />

