<template>
    <div class="main">
        <h4>Explorer</h4>
        <button @click="GetDir()">O</button>
        <div class="file-tree-div">
            <span v-for="ele in folders" :key="ele.isDir" @click="$emit('OpenFile', ele.name)">
                <img src="../assets/folder.svg" alt="" v-if="ele.is_dir">
                <img src="../assets/file.svg" alt="" v-else>
                <p>{{ ele.name.split('/').slice(-1)[0] }}</p>
            </span>
        </div>
        <input type="text" v-model="projPath" id="proj-dir" @keyup.enter="GetDir()">
    </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";

export default {
    name: "Explorer",
    data() {
        return {
            folders: Array,
            projPath: String
        }
    },
    methods: {
        GetDir,
        OnEnter
    },
    mounted() {
        this.projPath = ""
    }
}

async function GetDir() {
    console.log(this.projPath);
    const files = await invoke("get_proj_dir", { path: this.projPath});
    //files.sort((a, b) => a.is_dir < b.is_dir ? 1 : -1);
    this.folders = files;
    console.log(this.folders);
}

function OnEnter() {
    GetDir();
}
</script>

<style scoped>
main {
    position: relative;
    overflow: hidden;
    padding: 0;
    box-sizing: border-box;
    display: grid;
    grid-template-columns: 2fr 10fr;
}

h4 {
    text-align: left;
    margin: 1rem;
    margin-left: 1rem;
}

.file-tree-div {
    display: flex;
    flex-direction: column;
    gap: 0.5ch;
    overflow-y: auto;
    overflow-x: hidden;
    height: 100%;
}

img {
    aspect-ratio: 1;
    width: 1.5ch;
}

span {
    display: flex;
    gap: 0.5ch;
}

p {
    margin: 0;
    font-size: 12px;
    text-align: left;
    white-space: nowrap;
}

button {
    margin: 5px;
    padding: 1ch;
    border-radius: 5px;
    position: absolute;
    top: 1ch;
    left: min(15vw, 8rem);
    font-size: 12px;
}

input {
    position: absolute;
    bottom: 1rem;
}
</style>