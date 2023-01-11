<template>
    <div class="Window">
        <TopBar :tabs="files"
                @remove="(i) => files.splice(i, 1)"/>
        <div style="display: flex;">
            <Editor :selected="file.isSelected" :content="file.content" v-for="(file, index) in files" :key="file.name" />
        </div>
    </div>
</template>

<script>
import Editor from './Editor.vue';
import TopBar from './TopBar.vue';


export default {
    name: "MainWindow",
    props: {
        files: Array,
    },
    components: {
    TopBar,
    Editor
    },
    methods: {
        showWindow(idx) {
            this.files.forEach((tab, i) => {
                if(i != idx)
                    tab.isSelected = false;
                else
                    tab.isSelected = true;
            });
        }
    }
}
</script>

<style scoped>
    .Window {
        height: 100vh;
        background-color: none;
        width: calc(100% - min(30vw, 18rem));
    }

    
</style>