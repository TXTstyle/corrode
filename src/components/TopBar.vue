<template>
    <div class="main">
        <span v-for="(tab, idx) in tabs" :key="tab.name" :selected="tab.isSelected" @click="selectTab(idx)">
            <img src="../assets/file.svg" alt="">
            <p>{{ tab.name.split('/').slice(-1)[0] }}</p>
            <button @click="$emit('remove', idx)">x</button>
        </span>
    </div>
</template>

<script>
export default {
    name: "TopBar",
    props: {
        tabs: Array
    },
    data() {
        return {
            
        }
    },
    methods: {
        selectTab(idx) {
            //console.log(idx);
            this.tabs.forEach((tab, i) => {
                if(i != idx)
                    tab.isSelected = false;
                else
                    tab.isSelected = true;
            });
            this.$emit('selected', idx)
        }
    }
}
</script>

<style scoped>
.main {
    box-sizing: border-box;
    display: flex;
    flex-direction: row;
    background-color: rgb(45, 45, 45);
    gap: 0.1rem;
    padding: 0 1rem;
    border-bottom: 2px solid rgb(40, 40, 40);
    border-left: 2px solid rgb(40, 40, 40);
    box-shadow: 0 1px 0px black;
    min-height: 2rem;
}

button {
    aspect-ratio: 1;
    width: 1rem;
    font-size: 10px;
    padding: 0;
    background: none;
    border: none;
    box-shadow: none;
}

button:hover {
    background-color: black;
}

p {
    margin: 0;
    padding: 0;
}

span {
    display: flex;
    gap: 0.5ch;
    background-color: rgb(48, 48, 48);
    border-right: 1px, solid rgb(40, 40, 40);
    padding: 1ch;
}
span:hover {
    cursor: pointer;
    background-color: rgb(52, 52, 52);
}

img {
    aspect-ratio: 1;
    width: 14px;
}

[selected="true"] {
    background-color: rgb(58, 58, 58);
}
</style>