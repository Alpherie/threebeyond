<template>
    <div id="content">
        <!-- <div v-if="title" id="pageTitle" class="third">  <span v-html="title"></span>  </div> -->
        <Frame></Frame>
    </div>
</template>

<script>
    import Frame from "./Frame/index"

    import Vue from 'vue'

    export default {
        components: { Frame },

        data() {
            return {
                title: null
            }
        },

        mounted() {
            Vue.prototype.$setTitle = (v) => {
                this.title = v
            }
        }
    }
</script>