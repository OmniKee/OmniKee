<template>
  <div class="window-buttons" v-if="isTauri">
    <img src="@/assets/window/minimize.svg" @click="onMinimize" />
    <img src="@/assets/window/maximize.svg" @click="onMaximize" />
    <img src="@/assets/window/close.svg" @click="onClose" />
  </div>
</template>

<script setup lang="ts">
import {getCurrentWindow} from '@tauri-apps/api/window'

const isTauri = process.env.TAURI_ENV_PLATFORM !== 'web'

async function onMinimize() {
  const win = getCurrentWindow()
  await win.minimize()
}

async function onMaximize() {
  const win = getCurrentWindow()
  await win.toggleMaximize()
}

async function onClose() {
  const win = getCurrentWindow()
  await win.close()
}


</script>

<style scoped lang="scss">
.window-buttons {
  display: flex;
  flex-direction: row;

  img {
    border: 10px solid rgba(0, 0, 0, 0);
    opacity: 0.7;
    cursor: pointer;

    &:hover {
      opacity: 1.0;
    }
  }
}
</style>
