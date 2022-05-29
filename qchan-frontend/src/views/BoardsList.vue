<template>
    <div id="boardsList">
        <h1 v-if="lang">// {{ lang.toUpperCase() }} //</h1>
        <div class="buttons-wrap">
            <button class="btn btn-a btn-3" @click="refresh"><icon icon="sync" /></button>
            <button class="btn btn-a btn-2" v-if="hasPermTo('create')" @click="createNewBoardShown = true"><icon icon="plus" /></button>
        </div>

        <button v-for="(board, index) in boards" v-bind:key="index" @click="$router.push({ name: 'board', params: { lang: lang, dir: board.short } })" class="board btn btn-4">
            <div class="head"><span class="short">/{{ board.short }}/</span> <span class="name">{{ board.name }}</span></div>
            <div class="description">{{ board.description }}</div>
        </button>

        <CreateEditBoard v-if="createNewBoardShown" :lang="lang" @done="createNewBoardShown = false" @cancel="createNewBoardShown = false" />
    </div>
</template>

<script>
    import Lang   from '@/services/Lang'
    import Board  from '@/services/Board'
    import CreateEditBoard  from '@/components/CreateEditBoard'
    import $      from 'jquery'
    
    export default {
        components: { CreateEditBoard },

        data() {
            return {
                lang:  null,
                scroll: null,
                boards: null,
                createNewBoardShown: false
            }
        },

        methods: {
            async refresh () {
                this.boards = []
                try {
                    this.boards = await Board.getAllFresh(this.$route.params)
                } catch(e) {
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.boards.getAllFailed"),
                        "text":  this.$t(`apiErrorCodes.${e.code}`)
                    })

                    throw(e)
                }
            },

            hasPermTo(perm) {
                let params = this.$route.params
                let label = `langs::${params.lang}::boards::${perm}`

                return this.$store.state.admin.approved && this.$store.state.admin.perms.indexOf(label) != -1
            },
        },

        async mounted() {
            this.lang = this.$route.params.lang

            try {
            	this.boards = await Board.getAll(this.$route.params)
            } catch(e) {
                this.$notify({
                    "group": "main",
                    "type":  "error",
                    "title": this.$t("api.boards.getAllFailed"),
                    "text":  this.$t(`apiErrorCodes.${e.code}`)
                })

                throw(e)
            }
        },
    }
</script>