<template>
    <div class="main">
        <Explorer @OpenFile="(ele) => $emit('OpenFile', ele)"/>
    </div>
</template>

<script>
import Explorer from './Explorer.vue';

export default {
    name: "SideBar",
    components: {
        Explorer
    }
};
</script>

<style scoped>
.main {
    box-sizing: border-box;
    max-width: min(30vw, 18rem);
    min-width: min(30vw, 18rem);
    height: 100vh;
    background-color: rgb(44, 44, 44);
    border-right: 2px solid rgb(40, 40, 40);
    box-shadow: 0 1px 0px black;
}
</style>