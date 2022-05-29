<template>
    <div id="thread-route" v-if="thread">
        <!-- <DynamicScroller
            v-if="posts"
            :items="posts"
            :min-item-size="50"
            class="scroller"
            key-field="num">
            <template v-slot="{ item, index, active }">
                <DynamicScrollerItem
                    :item="item"
                    :active="active"
                    :size-dependencies="[
                      item.comment,
                    ]"
                    :data-index="index">
                        <Post :buttons="true" :threadRoute="true" :key="index" :index="index+1" :dir="dir" :post="item" :posts="posts" :repliesMap="repliesMap" :thread="thread" :postsMap="postsMap" :ref="item.num" />
                </DynamicScrollerItem>
            </template>
        </DynamicScroller> -->
        <div id="thread-top-bar">
            <div class="left">
                <div class="buttons buttons-wrap">
                    <button class="btn btn-1 btn-a" @click="goToBoard"><icon icon="chevron-left" /></button>
                    <button class="btn btn-3" :class="{ 'btn-a': isFavourite }" @click="(v) => isFavourite ? removeFromFavourites(v) : addToFavourites(v)"><icon icon="bookmark" /></button>
                    <button class="btn btn-4" @click="getNewPosts"><icon icon="sync" /></button>
                </div>
                <span class="title" v-if="thread">
                    <!-- {{ title }} -->
                </span>
            </div>
            <div class="center">
                <div class="buttons buttons-wrap">
                    <button class="btn btn-4 btn-a" :disabled="page == 0" @click="loadPrevPage" v-if="thread && board !== undefined && thread.endless"><icon icon="backward" /></button>
                    <button class="btn btn-1 btn-a" @click="goStart" v-if="thread"><icon icon="arrow-up" /></button>
                    <!-- <button class="btn btn-3 btn post-num" @click="showJumpTo" v-if="!jumpInputShown">
                        <template v-if="visibleList.length">{{ visibleList[0]+1 }}</template>
                        <template v-else>{{ lastVisible }}</template>
                    </button> -->
                    <!-- <input ref="jumpto" class="post-num-jump-to" v-click-outside="jumpToProcess" type="text" v-if="jumpInputShown && visibleList.length" v-model="jumpToInput" v-on:keyup="jumpToKeyUp"> -->
                    <button class="btn btn-2 btn-a infinity-btn" @click="pageSelectShown = true" v-if="thread && board !== undefined && thread.endless">
                        <!-- <icon icon="infinity" /> -->
                        {{ page + 1 }}
                    </button>
                    <button class="btn btn-1 btn-a" @click="goEnd" v-if="thread"><icon icon="arrow-down" /></button>
                    <button class="btn btn-4 btn-a" :disabled="posts.length < board.bumplimit" @click="loadNextPage" v-if="thread && board !== undefined && thread.endless"><icon icon="forward" /></button>
                </div>
                <div class="page-switch-box" @click="pageSelectShown = false" v-if="pageSelectShown">
                    <div class="background"></div>
                    <div class="page-switch">
                        <div class="buttons">
                            <button v-for="index in Math.ceil(thread.count/board.bumplimit)" @click="goPage(index - 1)" class="btn btn-a" :class="{ 'btn-4': page !== index-1, 'btn-2': page === index-1 }">{{ index }}</button>
                        </div>
                    </div>
                </div>
            </div>

            <div class="right">
                <div class="buttons">
                    <button class="btn btn-3 btn-a" v-if="thread && thread.pinnedPosts.length" @click="showPinnedPosts"><icon icon="thumbtack" /> {{ thread.pinnedPosts.length }}</button>
                    <button class="btn btn-4 btn-a" v-if="thread" @click="replyThread"><icon icon="reply" /> {{ thread.count-1 }}</button>
                    <!-- <button class="btn btn-4"><icon icon="search" /></button> -->
                </div>
            </div>
        </div>
        <virtual-list v-if="posts" ref="list" v-viewer="viewerOptions" :class="{ 'virtual-list': true, 'limited': this.$store.state.settings.fixedNavbar }"
            :data-key="'num'"
            :data-sources="posts"
            :data-component="postComponent"
            :estimate-size="150"
            :extra-props="{ lang, board, boardShort: board.short, visibleList, buttons: true, threadRoute: true, dir, posts, repliesMap, thread, postsMap }"
        />
        <PostsSelectMenu />
        <!-- <Post v-for="(post, index) in posts" :buttons="true" :threadRoute="true" :key="index" :index="index+1" :dir="dir" :post="post" :posts="posts" :repliesMap="repliesMap" :thread="thread" :postsMap="postsMap" :ref="post.num" /> -->
        <!-- <div class="center f-4 p-5">
            <button class="btn btn-1" @click="getNewPosts"><icon icon="sync"/></button>
        </div> -->
    </div>
</template>

<style scoped>
    .scroller {
      height: 100%;
    }
</style>

<script>
    import Api         from '../../../../services/Api'
    import Thread      from '../../../../services/Thread'
    import Post        from './PostEntry'
    import Board  from '../../../../services/Board'
    // import $           from 'jquery'
    import VirtualList from 'vue-virtual-scroll-list'
    import PostsSelectMenu from '@/components/App/Content/Frame/PostsSelectMenu'

    const REGEX_REPLY_I = new RegExp(/<span class="reply" data-num="([0-9]+)">&gt;&gt;([0-9]+)<\/span>/gi)

    export default {
        components: { PostsSelectMenu, Post, 'virtual-list': VirtualList },

        data() {
            return {
                updateInt:   null,

                title:       null,
                lang:        null,
                dir:         null,
                num:         null,
                thread:      null,
                posts:       null,
                postsMap:    null,
                scroll:      null,
                repliesMap:  {},
                postsMap:    {},
                type:        null,
                scrollInterval: null,
                postComponent: Post,
                lastIndex: 1,
                isFavourite: null,
                visibleList: [],
                jumpInputShown: false,
                jumpToInput: 0,
                lastVisible: 0,
                pageSelectShown: false,
                page: 0,

                loaded: false,

                viewerOptions: {
                    url: 'data-src'
                }
            }
        },

        watch: {
            visibleList () {
                if (this.visibleList[0] !== undefined)
                    this.lastVisible = this.visibleList[0]
            },
        },

        methods: {
            loadPrevPage () {
                this.goPage(this.page - 1)
            },
            

            loadNextPage () {
                this.goPage(this.page + 1)
            },

            jumpToKeyUp ({ keyCode }) {
                if (keyCode === 13)
                    this.jumpToProcess()
            },

            jumpToProcess () {
                this.jumpToIndex(this.jumpToInput-1, 10)

                this.$nextTick(() => {
                    this.jumpInputShown = false
                })
            },

            showJumpTo () {
                this.jumpToInput = this.visibleList[0]+1
                this.jumpInputShown = true

                this.$nextTick(() => {
                    this.$refs.jumpto.focus()
                    this.$refs.jumpto.select()
                })
            },

            goToBoard () {
                const params = this.$route.params
                this.$router.push({
                    name: 'board',
                    params: {
                        'lang': params.lang,
                        'dir': params.dir
                    }
                })
            },

            jumpToIndex (index, balance) {
                const last_index = this.visibleList[0]
                this.$refs.list.scrollToIndex(index)
                this.$nextTick(() => {
                    if (this.visibleList[0] == last_index) {
                        this.$refs.list.scrollToOffset(this.$refs.list.getOffset() + balance)
                    }
                })
            },

            goPrev () {
                this.jumpToIndex((this.visibleList[0] || this.lastVisible)-1, 10)
            },

            goNext () {
                this.jumpToIndex((this.visibleList[0] || this.lastVisible)+1, 10)
            },

            goStart () {
                this.$refs.list.reset()
            },

            goEnd () {
                this.$refs.list.scrollToBottom()
            },

            replyThread () {
                let params = this.$route.params
                this.$reply({
                    thread: params.num,
                    ...params
                })
            },

            showPinnedPosts () {
                let list = new Array()
                for (const postNum of this.thread.pinnedPosts) {
                    list.push(this.postsMap[postNum])
                }

                this.$popup({ thread: this.thread, list, posts: this.posts, repliesMap: this.repliesMap, postsMap: this.postsMap })
            },

            addToFavourites () {
                this.$store.commit("setFavouriteThread", {
                    lang: this.$route.params.lang,
                    dir: this.$route.params.dir,
                    title: this.title,
                    count: this.thread.count,
                    num: this.$route.params.num
                })
                this.isFavourite = true
                // this.$customMenu([
                //     { icon: "sync", action: this.getNewPosts },
                //     { icon: "bookmark", action: this.removeFromFavourites, active: true },
                // ])
            },

            removeFromFavourites () {
                this.$store.commit("unsetFavouriteThread", {
                    lang: this.$route.params.lang,
                    dir: this.$route.params.dir,
                    num: this.$route.params.num
                })
                this.isFavourite = false
                // this.$customMenu([
                //     { icon: "sync", action: this.getNewPosts },
                //     { icon: "bookmark", action: this.addToFavourites },
                // ])
            },

            async goPage(page) {
                this.posts = []
                this.repliesMap = {}
                this.postsMap = {}
                this.page = page
                this.lastIndex = 1
                let thread
                try {
                    thread = await Thread.getPosts({ page, full: true, from: 0, ...this.$route.params })
                    this.thread.count = thread.count
                    this.thread.endless = thread.endless
                    this.updatePosts(thread.posts)
                    this.$notify({
                        "group": "main",
                        "type":  "success",
                        "title": this.$t("api.threads.pageLoad.success", { page: page + 1 }),
                    })
                } catch(e) {
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.threads.reload.failed"),
                        "text":  this.$t(`apiErrorCodes.${e.code}`)
                    })

                    throw e
                }
            },

            updatePosts(posts) {
                for (const index in posts) {
                    const post = posts[index]
                    const ignore = {}
                    if (post.num === this.thread.num) {
                        post.isThread = true
                    }
                    post.isDeleted = false
                    post.index = this.page * this.board ? this.board.bumplimit : 1 + this.lastIndex++

                    if (post.pinned) {
                        this.thread.pinnedPosts.push(post.num)
                    }

                    const matches = post.comment.matchAll(REGEX_REPLY_I)
                    if (matches == null) { continue; }

                    for (const match of matches) {
                        let num = parseInt(match[1])

                        if (ignore[num] != undefined)
                            continue
                        else
                            ignore[num] = true

                        if (this.repliesMap[num] == undefined)
                            this.repliesMap[num] = [post]
                        else
                            this.repliesMap[num].push(post)
                        
                        if (this.$refs[num]) {
                            this.$refs[num].$forceUpdate()
                        }
                    }

                    this.postsMap[post.num] = post
                    this.posts.push(post)
                }

                this.$store.commit("updateFavouriteThread", { lang: this.$route.params.lang, dir: this.$route.params.dir, num: this.$route.params.num, count: this.thread.count, newCount: this.thread.count, title: this.title })
            },

            async getNewPosts(silent) {
                if (this.thread.endless && this.board.bumplimit === this.posts.length)
                    return;

                let from = this.posts[this.posts.length - 1] ? this.posts[this.posts.length - 1].num + 1 : 0

                let thread
                try {
                    thread = await Thread.getPosts({ page: this.page, full: true, from, ...this.$route.params })
                    this.thread.count = thread.count
                    this.thread.endless = thread.endless
                    this.updatePosts(thread.posts)

                    if (!silent || thread.posts.length) {
                        this.$notify({
                            "group": "main",
                            "type":  "success",
                            "title": this.$t("api.threads.reload.success"),
                            "text": this.$t("api.threads.reload.newPosts", { length: thread.posts.length })
                        })
                    }
                } catch(e) {
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.threads.reload.failed"),
                        "text":  this.$t(`apiErrorCodes.${e.code}`)
                    })

                    throw e
                }
            },

            async fetch() {
                const params = this.$route.params

                this.lang  = params.lang;
                this.dir   = `/${params.lang}/${params.dir}/${params.num}`;
                this.num   = params.num;
                this.posts = [];
                this.type  = this.$route.name;

                if (this.$router.restoreStateCache(this)) {
                    setTimeout(() => {
                        this.$refs.list.scrollToOffset(this.scroll)
                    }, 50)
                    return Promise.resolve();
                }

                // this.$notify({
                //     "group": "main",
                //     "type":  "info",
                //     "title": this.$t("api.threads.getStarted"),
                // })

                try {
                    this.thread = await Thread.get(params)
                    this.thread.posts[0].isThread = true
                    this.thread.pinnedPosts = []

                    let v = this.$store.state.threadSecrets[`${params.lang}/${params.dir}/${params.num}`]
                    if (v !== undefined) {
                        this.thread.secret = v
                    }
                } catch(e) {
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.threads.getFailed"),
                        "text":  this.$t(`apiErrorCodes.${e.code}`)
                    })

                    throw(e)
                }

                this.$notify({
                    "group": "main",
                    "type":  "success",
                    "title": this.$t("api.threads.getFinished"),
                })

                for (const post of this.thread.posts) {
                    const ignore = {}
                    post.isDeleted = false
                    post.index = this.lastIndex++

                    if (post.pinned) {
                        this.thread.pinnedPosts.push(post.num)
                    }

                    const matches = post.comment.matchAll(REGEX_REPLY_I)
                    if (matches == null) { continue; }

                    for (const match of matches) {
                        let num = parseInt(match[1])

                        if (ignore[num] != undefined)
                            continue
                        else
                            ignore[num] = true

                        if (this.repliesMap[num] == undefined)
                            this.repliesMap[num] = [post]
                        else
                            this.repliesMap[num].push(post)

                        if (this.$refs[num]) {
                            this.$refs[num][0].$forceUpdate()
                        }
                    }

                    this.postsMap[post.num] = post
                }

                this.posts = this.thread.posts
                let idx = `${this.$route.params.lang}/${this.$route.params.dir}/${this.thread.num}`
                this.$root.$refs[idx] = this
            },
        },

        // beforeRouteEnter(to, from, next) {
        //     next()
        // },

        beforeRouteUpdate(to, from, next) {
            if (!this.$router.sameRoute(from, to)) {
                this.visibleList = []
                this.$router.saveStateCache(this);
                this.board = null;
            }

            // this.$root.$refs[{ lang: this.$route.params.lang, dir: this.$route.params.dir, num: this.$route.params.num }] = this

            next();
        },

        beforeRouteLeave(to, from, next) {
            if (!this.$router.sameRoute(from, to)) {
                this.$router.saveStateCache(this);
            }

            this.visibleList = []
            clearInterval(this.scrollInterval);

            this.$customMenu([]);
            delete this.$root.$refs.openedThread

            next();
        },

        updated() {
        },

        beforeDestroy () {
            if (this.updateInt) {
                clearInterval(this.updateInt)
            }
        },

        async mounted() {
            const params = this.$route.params

            this.board = await Board.get(params)

            this.isFavourite = this.$store.state.favouriteThreads.langs[params.lang] !== undefined && this.$store.state.favouriteThreads.langs[params.lang].boards[params.dir] != undefined && this.$store.state.favouriteThreads.langs[params.lang].boards[params.dir].threads[params.num] !== undefined

            this.$nextTick(() => {
                this.$updateRoutes(this.$router.currentId(this), this)

                this.$keyBind(116, event => {
                    this.getNewPosts()

                    return false
                })
                let favouriteButton;
                if (this.isFavourite) {
                    favouriteButton = { icon: "bookmark", action: this.removeFromFavourites, active: true }
                } else {
                    favouriteButton = { icon: "bookmark", action: this.addToFavourites }
                }

                // this.$customMenu([
                //     { icon: "sync", action: this.getNewPosts },
                //     favouriteButton,
                // ])
            })
            
            this.scrollInterval = setInterval(() => {
                if (this.$refs.list) {
                    this.scroll = this.$refs.list.getOffset()
                }
            }, 100);

            await this.fetch().catch(err => {
                this.$unblock()
                console.error(err)
            })

            if (this.$store.state.favouriteThreads.langs[params.lang] !== undefined && this.$store.state.favouriteThreads.langs[params.lang].boards[params.dir] != undefined && this.$store.state.favouriteThreads.langs[params.lang].boards[params.dir].threads[params.num] !== undefined) {

                    if (this.$store.state.favouriteThreads.langs[params.lang].boards[params.dir].threads[params.num].newCount > this.$store.state.favouriteThreads.langs[params.lang].boards[params.dir].threads[params.num].count) {
                        this.getNewPosts()
                    }
                }

            if (this.thread.posts[0].subject)
                this.title = this.thread.posts[0].subject
            else
                this.title = this.thread.posts[0].comment

            this.title = this.title.noTags().trunc(20).trim()

            this.$setWriteformDefault({ lang: this.$route.params.lang, board: this.$route.params.dir, thread: this.$route.params.num })

            for (const post of this.posts) {
                this.postsMap[post.num] = post
            }

            this.$store.commit('visitThread', {
                lang: this.$route.params.lang,
                dir: this.$route.params.dir,
                num: this.$route.params.num,
                subject: this.title
            })

            this.$refs.list.scrollToOffset(this.scroll)
            this.$root.$refs.openedThread = this

            this.$store.commit("updateFavouriteThread", { lang: this.$route.params.lang, dir: this.$route.params.dir, num: this.$route.params.num, count: this.thread.count, newCount: this.thread.count, title: this.title })

            // this.$setTopTitle(`${params.lang.toUpperCase()} -> ${params.dir.toUpperCase()} -> ${params.num} -> ${this.title}`)
            // this.$setTopButtons([
            //     { icon: 'bookmark', action: console.log }
            // ])
            // this.$showTopBar()
            this.updateInt = setInterval(() => {
               this.getNewPosts(true) 
            }, 30000)
        }
    }
</script>
