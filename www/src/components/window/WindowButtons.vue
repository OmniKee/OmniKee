<template>
  <div class="window-buttons mac q-mr-sm" v-if="isMac">
    <div class="close" @click="onClose">
      <img src="@/assets/window/close-mac.svg" />
    </div>

    <div class="minimize" @click="onMinimize">
      <img src="@/assets/window/minimize-mac.svg" />
    </div>

    <div class="maximize" @click="onMaximize">
      <img src="@/assets/window/maximize-mac.svg" />
    </div>
  </div>

  <div class="window-buttons" v-if="isTauri && !isMac">
    <img src="@/assets/window/minimize.svg" @click="onMinimize" />
    <img src="@/assets/window/maximize.svg" @click="onMaximize" />
    <img src="@/assets/window/close.svg" @click="onClose" />
  </div>
</template>

<script setup lang="ts">
import {getCurrentWindow} from '@tauri-apps/api/window'

const isTauri = process.env.TAURI_ENV_PLATFORM !== 'web'
const isMac = process.env.TAURI_ENV_PLATFORM === 'darwin'

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
.mac {
  order: -1;

  .close,
  .minimize,
  .maximize {
    background: #fff;
    border-radius: 100%;
    width: 2vh;
    height: 2vh;

    margin: 0 6px 0 6px;
    display: flex;
    align-items: center;
    justify-content: center;

    img {
      padding: 0px;
      opacity: 0;
    }
  }

  .close {
    background: #f85951;
  }

  .minimize {
    background: #febc2e;
  }

  .maximize {
    background: #1dad2e;
  }
}

.mac.window-buttons:hover img {
  opacity: 1;
}

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
