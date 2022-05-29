<template>
    <div v-if="board !== null" id="threads" v-viewer="viewerOptions">
        <h1 style="padding-left: 25px">{{ lang.toUpperCase() }}/{{ board.short }}   {{ board.name }}  </h1>
        <BoardInformation v-if="shownBoardInfo" v-on:close="shownBoardInfo = false" v-bind:board="board" />
        <CreateEditBoard v-if="openBoardSettings" @cancel="openBoardSettings = false" @done="onBoardEdited" :lang="lang" :defaultValue="board" />

        <div class="pl-6 buttons-wrap">
            <!-- <button class="btn btn-2" @click="refresh">  {{ $t('labesls.createThread') }}  </button> -->
            <button class="btn btn-4 btn-a" v-bind:class="{ 'btn-a': isCatalog }" @click="refresh"><icon v-bind:icon="isCatalog ? 'chevron-left' : 'sync'" /></button>
            <button class="btn btn-5 btn-a" @click="refreshCatalog" v-if="!isCatalog">
                <icon icon="list" />
            </button>
            <button class="btn btn-2 btn-a" @click="$root.$refs.writeform.toggle"><icon icon="plus" /></button>
            <button class="btn btn-3 btn-a" v-bind:class="{ 'btn': true, 'btn-3': true, 'btn-a': shownBoardInfo }" @click="shownBoardInfo = !shownBoardInfo">
                <icon icon="question-circle" />
            </button>
            <button class="btn btn-6 btn-a" v-if="hasPermTo('control')" @click="openBoardSettings = true"><icon icon="cog" /></button>
            <button class="btn btn-1 btn-a" @click="goToReportsPage" v-if="hasPermTo('reports::list')"><icon icon="exclamation-circle" /></button>
        </div>
       
        <div class="threads">
            <ThreadEntry v-for="(thread, index) in threads" v-bind:key="index" :board="board" v-bind:thread="thread"></ThreadEntry>
        </div>

        <div v-if="threads.length === 0 && threadsLoading === false" class="p-5 m-5 combination-1">
            {{ $t('boardList.thereAreNoThreads') }}
        </div>
        <div v-if="threadsLoading === true" class="p-5 m-5 combination-3">
            {{ $t('boardList.loadingThreads') }}
        </div>

        <center style="padding-top: 5px" v-if="threads.length > 0 && !isCatalog">
            <button v-if="board.pages_count-1 > page" class="btn btn-3 btn-a" @click="loadNextPage">
                <icon icon="forward" />
                {{ page + 1 }}
            </button>
        </center>

        <div class="pl-6 buttons-wrap" v-if="threads.length">
            <!-- <button class="btn btn-2" @click="refresh">  {{ $t('labesls.createThread') }}  </button> -->
            <button class="btn btn-4 btn-a" v-bind:class="{ 'btn-a': isCatalog }" @click="refresh"><icon v-bind:icon="isCatalog ? 'chevron-left' : 'sync'" /></button>
            <button class="btn btn-5 btn-a" @click="refreshCatalog" v-if="!isCatalog">
                <icon icon="list" />
            </button>
            <button class="btn btn-2 btn-a" @click="$root.$refs.writeform.toggle"><icon icon="plus" /></button>
            <button class="btn btn-3 btn-a" v-bind:class="{ 'btn': true, 'btn-3': true, 'btn-a': shownBoardInfo }" @click="shownBoardInfo = !shownBoardInfo">
                <icon icon="question-circle" />
            </button>
            <button class="btn btn-6 btn-a" v-if="hasPermTo('control')" @click="openBoardSettings = true"><icon icon="cog" /></button>
            <button class="btn btn-1 btn-a" @click="goToReportsPage" v-if="hasPermTo('reports::list')"><icon icon="exclamation-circle" /></button>
        </div>
    </div>
</template>

<script>
    import Api    from '../../../../services/Api'
    import Board  from '../../../../services/Board'
    import BoardInformation from "@/components/App/BoardInformation"
    import CreateEditBoard  from '@/components/CreateEditBoard'

    import ThreadEntry from './ThreadEntry'

    import $ from 'jquery'

    export default {
        components: {  CreateEditBoard, BoardInformation, ThreadEntry },

        data() {
            return {
                shownBoardInfo: false,
                title:      null,
                lang:       null,
                board:      null,
                dir:        null,
                threads:    [],
                threadsLoading: true,
                page:       null,
                scroll:     null,
                type:       null,
                isCatalog:  false,
                openBoardSettings: false,

                viewerOptions: {
                    url: 'data-src'
                }
            }
        },

        methods: {
            async onBoardEdited () {
                try {
                    this.board = await Board.getFresh(this.$route.params)
                } catch(e) {
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.boards.getPageFailed"),
                        "text":  this.$t(`apiErrorCodes.${e.code}`)
                    })

                    throw e
                }
                this.openBoardSettings = false
            },

            hasPermTo(perm) {
                let params = this.$route.params
                let label = `langs::${params.lang}::boards::${params.dir}::${perm}`

                return this.$store.state.admin.approved && this.$store.state.admin.perms.indexOf(label) != -1
            },

            async refreshCatalog() {
                this.page = 0
                this.threads = []
                this.isCatalog = true

                this.threadsLoading = true
                this.threads = await Board.loadCatalog(this.$route.params)
                this.threadsLoading = false
            },

            async refresh() {
                this.page = 0
                this.threads = []
                await this.loadNextPage()
            },

            goToReportsPage () {
                this.$router.push({ name: 'board-reports', params: { lang: this.$route.params.lang, dir: this.$route.params.dir }  })
            },

            async applyThread(newThread) {
                // 0. make sure if this thread is already registered
                if (this.threads)
                    for (let thread of this.threads) {
                        if (thread.num == newThread.num)
                            return false
                    }
                else
                    return false

                // 1. apply
                this.threads.push(newThread)
            },

            async loadNextPage() {
                const params = this.$route.params
                let threads
                this.isCatalog = false

                // this.$notify({
                //     "group": "main",
                //     "type":  "info",
                //     "title": this.$t("api.boards.loadPageStarted")
                // })
                
                try {
                    this.threadsLoading = true
                    threads = await Board.getPage({ ...params, page: this.page++ })
                    for (const thread of threads) {
                        thread.pinnedPosts = []
                    }
                    this.threadsLoading = false
                } catch(e) {
                    console.error(e)
                    this.page = 9999

                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.boards.getPageFailed"),
                        "text":  this.$t(`apiErrorCodes.${e.code}`)
                    })
                
                    throw e
                }

                this.$notify({
                    "group": "main",
                    "type":  "success",
                    "title": this.$t("api.boards.loadPageFinished")
                })

                threads.map(this.applyThread)
            },

            async fetch() {
                const params = this.$route.params

                this.board = null
                this.threads = []
                this.type = this.$route.name
                this.dir = `/${params.lang}/${params.dir}/`;
                
                if (this.$router.restoreStateCache(this)) {
                    return Promise.resolve()
                }

                try {
                    this.board = await Board.get(this.$route.params)
                } catch(e) {
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.boards.getPageFailed"),
                        "text":  this.$t(`apiErrorCodes.${e.code}`)
                    })

                    throw e
                }

                this.page = 999

                await Board.getPage({ ...params, page: 0 })
                .then(threads => {
                    this.threadsLoading = false
                    for (const thread of threads) {
                        thread.pinnedPosts = []
                    }
                    this.threads = threads
                    this.page    = 1
                })
                .catch(e => {
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.boards.getPageFailed"),
                        "text":  this.$t(`apiErrorCodes.${e.code}`)
                    })

                    throw e
                })
            },
        },

        beforeRouteUpdate(to, from, next) {
            if (!this.$router.sameRoute(from, to)) {
                this.$router.saveStateCache(this);
                this.board = null;
            }

            next();
        },
        beforeRouteLeave(to, from, next) {
            if (!this.$router.sameRoute(from, to)) {
                this.$router.saveStateCache(this);
            }

            next();
        },

        updated() {
            const target = $('#content')

            target.scrollTop(this.scroll)
            target.unbind().on('scroll resize', target => this.scroll = event.target.scrollTop)
        },

        async mounted() {
            this.lang = this.$route.params.lang
            await this.fetch().catch(console.error)

            this.$setWriteformDefault({ lang: this.$route.params.lang, board: this.$route.params.dir, thread: null })

            this.title = this.board.name
            this.$updateRoutes(this.$router.currentId(this), this)

            this.$keyBind(116, event => {
                this.refresh()

                return false
            })
        }
    }
</script>
