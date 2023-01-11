<template>
  <div class="container">
    <SideBar @OpenFile="(ele) => OpenFile(ele)"/>
    <MainWindow :files="fileContents"/>
  </div>
</template>

<script setup>
import { register, isRegistered } from '@tauri-apps/api/globalShortcut';

import SideBar from "./components/SideBar.vue";
import MainWindow from "./components/MainWindow.vue"

import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri';
onMounted(async () => {
    const isRegistered = await isRegistered('CommandOrControl+Shift+O');

    if (!isRegistered) {
      await register('CommandOrControl+Shift+O', () => {
      console.log('Shortcut triggered');
      });
    }
})

let fileContents = ref([])

async function OpenFile(path) { 
  fileContents.value.push({
    content: await invoke('get_file', {file: path}).catch((e) => {
      return e.split(' (')[0];
    }),
    name: path,
    isSelected: false
  }
  )
  console.log(fileContents.value)
}

</script>

<style>
body {
  box-sizing: border-box;
}
</style>
