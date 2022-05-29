<template>
    <div id="langList">
        <h1>{{ $t("labels.clickOnLanguageToChoose") }}</h1>
        <button class="btn btn-4" @click="$router.push({ name: 'boards', params: { lang: lang.code }  })" v-for="lang in langs"><span class="code">  {{ lang.code.toUpperCase() }}  </span><span class="name">{{ lang.name }}</span></button>

        <div class="create-tip">
            {{ $t("labels.dontSeeYourLanguage") }} <br>
            <button class="btn btn-a btn-1">  {{ $t("labels.makeProposalLang") }}  </button>
        </div>
    </div>
</template>

<script>
    import Lang   from '@/services/Lang'
    import Board  from '@/services/Board'
    import $      from 'jquery'
    
    export default {
        data() {
            return {
                dir:    null,
                scroll: null,
                boards: null,
                langs: null
            }
        },

        methods: {
            goBoard(dir) {
                this.$router.push({ name: "board", params: { dir } })
            },

            async fetchBoard(board) {
                // if (this.$router.restoreStateCache(this)) {
                //     return Promise.resolve()
                // }

                // this.dir = "0"
                // this.title = this.$t("titles.boardList")

                // try {
                //     this.boards = await Board.getAll()
                // } catch(e) {
                //     this.$notify({
                //         "group": "main",
                //         "type":  "error",
                //         "title": this.$t("api.boards.getAllFailed"),
                //         "text":  this.$t(`apiErrorCodes.${e.code}`)
                //     })

                //     throw e
                // }

                // this.$notify({
                //     "group": "main",
                //     "type":  "success",
                //     "title": this.$t("api.boards.getAllFinished"),
                // })
            }
        },

        async mounted() {
        	// get langs
        	this.langs = await Lang.getAll()
            // await this.fetchBoard()

            // this.$updateRoutes(this.$router.currentId(this), this)

            $('#content').scrollTop(this.scroll)
            $('#content').unbind().on('scroll resize', target => this.scroll = event.target.scrollTop)
        },
    }
</script>