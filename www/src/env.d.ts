declare namespace NodeJS {
  interface ProcessEnv {
    NODE_ENV: string;
    DEPLOYMENT_TYPE: 'tauri' | 'web' | undefined;
    VUE_ROUTER_MODE: 'hash' | 'history' | 'abstract' | undefined;
    VUE_ROUTER_BASE: string | undefined;
  }
}
