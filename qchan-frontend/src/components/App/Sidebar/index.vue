<template>
    <div v-if="!$store.state.settings.activeRouteBar" v-bind:class="{ hidden: hidden }" id="sidebar">
        <div class="header">
            <!-- <div class="label"><b><span class="btn btn-1">で</span><span class="btn btn-2">す</span><span class="btn btn-3">か</span> <span class="btn btn-4">ちゃん</span></b></div> -->
            <div class="label"><b>デスカチャン</b></div>
            <!-- <div class="sublabel">メリークリスマス</div> -->
            
        </div>
        <OpenRoutes ref="openRoutes"></OpenRoutes>
        <div id="favourite">
            <div class="head" v-if="favList.length">
                <div class="title">{{ $t('labels.favourite') }}</div>
                <button class="btn btn-4 easy sync-btn" @click="reloadFavourite"><icon icon="sync"/></button>
            </div>
            <div class="content">
                <div v-for="(board, index) in favList" :class="{ 'board': true, 'highlight': board.newCount !== 0, 'expanded': board.expanded }">
                    <div class="head" @click="expandBoard(board.lang, board.dir)">
                        <div class="title">{{ board.title }}</div>
                        <!-- <div class="new" v-if="board.newCount !== 0">{{ board.newCount }}</div> -->
                    </div>
                    <div class="threads">
                        <div class="thread" v-for="(thread, tid) of board.threads" @click="goThread(board.lang, board.dir, thread.num)">
                            <div class="point-box">
                                <div class="point" :class="{ 'active': thread.new }"></div>
                            </div>
                            <div class="title">{{ thread.title }}</div>
                            <div class="counter new" v-if="thread.new">{{ thread.new }}</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
    import Vue        from 'vue'
    import OpenRoutes from './OpenRoutes'
    import Favourite  from '@/services/Favourite'

    export default {
        components: { OpenRoutes },

        data() {
            return {
                hidden: false,
                favList: {},
            }
        },

        methods: {
            async reloadFavourite () {
                // collect all
                let collection = ""
                let origin = this.$store.state.favouriteThreads

                for (let lang in origin.langs) {
                    for (let dir in origin.langs[lang].boards) {
                        if (origin.langs[lang].boards[dir] !== undefined && Object.keys(origin.langs[lang].boards[dir].threads).length !== 0) {
                            for (let num in origin.langs[lang].boards[dir].threads) {
                                collection += `${lang}/${dir}/${num},`
                            }
                        }
                    }
                }
                if (collection === "") {
                    return
                }
                collection = collection.substring(0, collection.length - 1)

                let favourite = await Favourite.get(collection)
                let searchingPath = ""
                if (this.$route.name === "thread") {
                    searchingPath = `${this.$route.params.lang}/${this.$route.params.dir}/${this.$route.params.num}`
                }

                for (let fav in favourite) {
                    let details = fav.split('/')
                    let lang = details[0]
                    let dir = details[1]
                    let num = details[2]

                    if (fav === searchingPath) {
                        let t = origin.langs[lang].boards[dir].threads[num]

                        if (t.count < favourite[fav] && this.$root.$refs.openedThread !== undefined) {
                            this.$root.$refs.openedThread.getNewPosts()
                        }
                    }

                    this.$store.commit("updateFavouriteNewCount", { lang, dir, num, count: favourite[fav] })
                }
            },

            expandBoard (lang, dir) {
                this.$store.commit("expandFavouriteBoard", { lang, dir })
            },

            goThread (lang, dir, num) {
                this.$router.push({ name: 'thread', params: { lang, dir, num } })
            },

            genFavList (state) {
                let origin = state.favouriteThreads

                let unpackedBoards = new Array()
                for (let lang in origin.langs) {
                    for (let dir in origin.langs[lang].boards) {
                        if (origin.langs[lang].boards[dir] !== undefined && Object.keys(origin.langs[lang].boards[dir].threads).length !== 0) {
                            const threads = new Array()
                            let newCount = 0
                            for (let num in origin.langs[lang].boards[dir].threads) {
                                let thread = origin.langs[lang].boards[dir].threads[num]
                                let newC = thread.newCount - thread.count

                                threads.push({ num, title: thread.title, new: thread.newCount - thread.count })
                            
                                if (newC !== 0) {
                                    newCount += newC
                                }
                            }

                            unpackedBoards.push({ newCount, expanded: origin.langs[lang].boards[dir].expanded, lang, dir, title: `${lang.toUpperCase()}/${dir}`, threads })
                        }
                    }
                }
                unpackedBoards.sort((a, b) => (a.newCount * 10000 + a.threads.length) < (b.newCount * 10000 + b.threads.length))
                return unpackedBoards
            },

            toggle() {
                if (this.hidden) {
                    this.show()
                } else {
                    this.hide()
                }
            },

            hide() {
                this.hidden = true
            },

            show() {
                this.hidden = false
            }
        },

        created () {
            this.favList = this.genFavList(this.$store.state);
            this.unsubscribe = this.$store.subscribe((mutation, state) => {
                if (mutation.type === "setFavouriteThread" || mutation.type === "unsetFavouriteThread" || mutation.type === "expandFavouriteBoard" || mutation.type === "updateFavouriteNewCount" || mutation.type === "updateFavouriteThread") {
                    this.favList = this.genFavList(state)
                }
            })
        },

        beforeDestroy() {
            this.unsubscribe();
          },

        mounted() {
            Vue.prototype.$toggleSidebar = this.toggle
            this.$root.$refs.sidebar = this

            setInterval(this.reloadFavourite, 30000)
        }
    }
</script>
