<template>
    <div id="navbar" class="buttons-wrap" v-bind:class="{ floating: !this.$store.state.settings.fixedNavbar }">
        <button v-if="!$store.state.settings.activeRouteBar" class="btn btn-1" v-bind:class="{ 'btn-a': $root.$refs.sidebar && !$root.$refs.sidebar.hidden }" @click="toggleSidebar"><icon icon="bars"/></button>
        <button class="btn btn-1"  @click="goHome"><icon icon="home"/></button>
        <button class="btn btn-1"  @click="goLangsList"><icon icon="tags"/></button>
        <button v-for="custom in customs" :class="{ 'btn': true, 'btn-1': !custom.active, 'btn-3': custom.active }" @click="custom.action"><icon :icon="custom.icon"/></button>
        <button class="btn btn-1"  @click="goSettings"><icon icon="cog"/></button>
        <button class="btn btn-1"  @click="goHistory"><icon icon="history"/></button>
        <!-- <button class="btn btn-1"v-bind:class="{ 'btn-a': $root.$refs.writeform && $root.$refs.writeform.opened }" @click="toggleWriteform"><icon icon="reply"/></button> -->
        <RouteBar v-if="$store.state.settings.activeRouteBar"></RouteBar>
    </div>
</template>

<script>
    import Vue from 'vue'
    import RouteBar from '../RouteBar'

    export default {
        components: { RouteBar },
        data() {
            return { dir: null, customs: [] }
        },

        methods: {
            goLangsList() {
                this.$router.push({ name: 'langs' })
            },

            goHome() {
                this.$router.push({ name: 'home' })
            },

            goSettings() {
                this.$router.push({ name: 'settings' })
            },

            goHistory() {
                this.$router.push({ name: 'history' })
            },

            toggleWriteform() {
                if (this.$root.$refs.writeform)
                    this.$root.$refs.writeform.toggle()
            },

            toggleSidebar() {
                if (this.$root.$refs.sidebar) {
                    this.$root.$refs.sidebar.toggle()
                }
            },
        },

        mounted() {
            Vue.prototype.$customMenu = (v) => this.customs = v;
        }
    }
</script>
<style lang="scss">
  #navbar {
        text-align: center;
        // background: var(--color0);
        z-index: 51;
        height: 50px;
        // border-top: 1px solid var(--color8);
        overflow-x: auto;
        white-space: nowrap;
        display: flex;
        justify-content: center;
        flex-wrap: wrap;

        &.floating {
            position: fixed;
            bottom: 0px;
            left: 50%;
            transform: translate(-50%, 0%);
            opacity: 0.2;
            border: none;        
            &:hover {
                transition: 0.1s;
                opacity: 0.8;
            }
        }

        > button {
            padding: 5px 15px 5px 15px;
        }
    }

</style>
