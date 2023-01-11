<template>
    <textarea :sel="selected" v-model="text" class=""></textarea>
</template>

<script>
export default {
    name: "Editor",
    props: {
        content: String,
        selected: Boolean
    },
    data() {
        return {
            text: String
        }
    },
    mounted() {
        this.text = this.content;
    }
}
</script>

<style scoped>
    textarea {
        width: 100%;
        height: 95vh;
        background-color: rgb(45, 45, 45);
        color: aliceblue;
        border: none;
        border-radius: 0;
        font-size: 18px;
        appearance: none;
    }

    textarea:focus, textarea:active, textarea:focus-visible {
        border: none;
        appearance: none;
        outline: none;
    }

    [sel="false"] {
        display: none;
    }
</style>