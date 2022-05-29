<template>
    <div>
        <vue-context ref="menu">
            <li v-for="option in options">
                <a v-if="option != undefined" @click.prevent="option.onclick($event)"><i v-if="option.icon" :class="`fa fa-${option.icon}`"></i> {{ option.title }}</a>
            </li>
        </vue-context>
    </div>
</template>

<script>
    import Vue        from 'vue'
    import VueContext from 'vue-context'

    export default {
        components: { VueContext },

        data() {
            return {
                options: []
            }
        },

        methods: {
            open(args) {
                this.options = args.options

                this.$refs.menu.open(args.event)
            }
        },

        mounted() {
            Vue.prototype.$context = this.open
        }
    }
</script>