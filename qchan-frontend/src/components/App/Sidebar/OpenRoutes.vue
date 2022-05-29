<template>
    <div id="open-routes">
       <button v-if="currentStateIndex" class="btn btn-a btn-4">
            <!-- <span @click="goState(currentStateIndex)" class="header third">{{ currentState.dir }}</span> -->
            <span @click="goState(currentStateIndex)" class="title" v-html="currentState.title"></span>
            <button class="btn easy destroy-btn btn-1" @click="destroyState(currentStateIndex)"><icon icon="times" /></button>
        </button>

        <button v-for="(state, index) in states" v-bind:key="index" class="btn btn-a btn-7">
            <!-- <span @click="goState(state.index)" class="header third">{{ state.data.dir }}</span> -->
            <span @click="goState(state.index)" class="title" v-html="state.data.title"></span>
            <button class="btn easy destroy-btn btn-1" @click="destroyState(state.index)"><icon icon="times" /></button>
        </button>
    </div>
</template>

<script>
    import Vue from 'vue'

    export default {
        data() {
            return {
                currentStateIndex: null,
                currentState:      null,
                states:            []
            }
        },

        methods: {
            context(event, state, index) {
                this.$context({ event, options: [
                    { title: this.$t("context.copyLink"), onclick: console.log },
                    { title: this.$t("context.openRoute.destroy"), onclick: e => this.destroyState(index) }
                ]})
            },

            updateStates() {
                const sortable = []
                for (const index in this.$router.stateCache.states) {
                    const t = this.$router.stateCache.states[index]
                    
                    sortable.push({index: index, data: t.data, _date: t._date})
                }

                sortable.sort((a, b) => {
                    return a._date < b._date
                })

                this.states = sortable
            },

            go(dir) {
                this.$router.push({ name: 'board', params: { dir: dir } }).catch(e => console.error("already"))
            },

            destroyState(id) {
                this.$router.destroyStateById(this, id)

                if (id == this.currentStateIndex) {
                    this.currentStateIndex = null
                    this.currentState = null
                }

                this.updateStates()
            },

            update(index, state) {
                this.currentStateIndex = index
                this.currentState = state

                this.updateStates()
            },

            goState(data) {
                this.$router.push(JSON.parse(data))
                if (this.$mobile())
                    this.$toggleSidebar()
            }
        },

        mounted() {
            this.$root.$refs.openRoutes = this

            Vue.prototype.$updateRoutes = this.update
        }
    }
</script>