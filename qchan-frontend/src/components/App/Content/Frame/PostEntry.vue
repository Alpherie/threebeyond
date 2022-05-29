<template>
    <div v-if="!hidden" :id='dir + "-" + source.num' class="post" v-bind:class="{ 'post-limit': board && thread.endless === false && index+1 == board.bumplimit, 'post-limit-endless': isLastEndless, deleted: source.isDeleted }">
        <BanDialog v-if="shownBanDialog" v-bind:lang="$route.params.lang" v-bind:board="$route.params.dir" v-bind:postNum="source.num" v-on:cancel="shownBanDialog = false" />
        <ReportDialog v-if="shownReportDialog" :lang="$route.params.lang" :board="$route.params.dir" :postNums="source.num" v-bind:title="$t('dialogs.report.title')" v-bind:message="$t('dialogs.report.message')" v-on:cancel="shownReportDialog = false" v-on:confirm="shownReportDialog = false" />

        <div v-bind:class="{ 'select': true, 'selected': selected }" v-if="$store.state.settings.postMultiselection && buttons && !source.isDeleted" @click="select"></div>
        <div class="head">
            <div class="info buttons-wrap">
                <span v-if="source.index" class="combination-3">{{ source.index }}</span>
                <span v-if="source.index === undefined && index+1" class="combination-3">{{ index+1 }}</span>
                <button class="btn btn-2 btn-a" @click="reply">{{ source.num }}</button>
                <span v-if="source.name" v-html='source.name' class="combination-3"></span>
                <strong v-if="source.op" class="combination-5"><icon icon="user" /></strong>
                <strong v-if="isMy" class="my-post combination-4"><icon icon="eye" /></strong>
                <strong v-if="(isThread && thread.endless)" class="thread-endless"><icon icon="infinity" /></strong>
                <strong class="thread-mutation" v-if="(isThread && thread.pinned) || source.pinned"><icon icon="thumbtack" /></strong>
                <strong class="thread-mutation" v-if="isThread && thread.closed"><icon icon="lock" /></strong>
                <strong class="thread-bumplimit" v-if="isThread && !thread.endless && board && thread.count > board.bumplimit"><icon icon="burn" /></strong>
                <strong v-if="source.mod">MOD</strong>
                <strong v-if="source.sage"><icon icon="burn" /></strong>
                <span v-if="hasFiles" class="combination-3"> {{ filesInfo }}</span>
                <span class="combination-7" ref="timeago">{{ timeAgo }}</span>
                <div v-if="source.trip" class="tripcode">
                  <div class="colors">
                    <div v-for="tripcolor in tripcolors" :style="{ background: tripcolor }"></div>
                  </div>
                  <div class="full">
                    {{ source.trip }}
                  </div>
                </div>
            </div>

            <div class="controls" v-if="!source.isDeleted">
                <button class="btn btn-4 center" v-if="buttons" @click="shownOptions = true"><icon icon="bars" /></button>
                <div class="options" v-if="shownOptions" v-click-outside="closeOptions">
                    <div class="row">
                        <button class="btn btn-1" @click="hide"><icon icon="eye-slash"/></button>
                        <button class="btn btn-1" @click="shownReportDialog = true"><icon icon="exclamation-circle"/></button>
                        <button v-if="thread.secret && thread.num != source.num" v-bind:class="{ btn: true, 'btn-3': source.pinned, 'btn-4': !source.pinned}" @click="pinUnpinPost">
                            <icon icon="thumbtack"/>
                        </button>
                        <button class="btn btn-4" @click="reply"><icon icon="reply"/></button>
                    </div>
                    <div class="row special" v-if="$store.state.admin.approved || (thread.secret && board &&  board.op_deletion_enabled)">
                        <button class="btn btn-1" v-if="hasPermTo('ban')" @click="shownBanDialog = true"><icon icon="user-slash"/></button>
                        <ConfirmDialog v-if="shownDeleteDialog" v-bind:title="$t('dialogs.delete.title')" v-bind:message="$t('dialogs.delete.message')" v-on:confirm="deletePost" v-on:cancel="shownDeleteDialog = false" />
                        <button class="btn btn-1" v-if="hasPermTo('delete') || (thread.secret && board && board.op_deletion_enabled)" @click="shownDeleteDialog = true"><icon icon="trash"/></button>
                        <button class="btn btn-4" @click="showDetails" v-if="hasPermTo('view_details')"><icon icon="info"/></button>
                    </div>
                    <div class="row thread-special" v-if="$store.state.admin.approved && isThread">
                        <ConfirmDialog v-if="shownCloseDialog" v-bind:title="$t('dialogs.closeThread.title')" v-bind:message="$t('dialogs.closeThread.message')" v-on:confirm="closeThread" v-on:cancel="shownCloseDialog = false" />
                        <button class="btn btn-seventh" @click="shownCloseDialog = true" v-if="hasPermToThreads('close') && !thread.closed"><icon icon="lock"/></button>

                        <ConfirmDialog v-if="shownOpenDialog" v-bind:title="$t('dialogs.openThread.title')" v-bind:message="$t('dialogs.openThread.message')" v-on:confirm="openThread" v-on:cancel="shownOpenDialog = false" />
                        <button class="btn btn-seventh" @click="shownOpenDialog = true" v-if="hasPermToThreads('open') && thread.closed"><icon icon="lock-open"/></button>

                        <ConfirmDialog v-if="shownPinDialog" v-bind:title="$t('dialogs.pinThread.title')" v-bind:message="$t('dialogs.pinThread.message')" v-on:confirm="pinThread" v-on:cancel="shownPinDialog = false" />
                        <button class="btn btn-seventh" @click="shownPinDialog = true" v-if="hasPermToThreads('pin') && !thread.pinned"><icon icon="thumbtack"/></button>

                        <ConfirmDialog v-if="shownUnpinDialog" v-bind:title="$t('dialogs.unpinThread.title')" v-bind:message="$t('dialogs.unpinThread.message')" v-on:confirm="unpinThread" v-on:cancel="shownUnpinDialog = false" />
                        <button class="btn btn-seventh" @click="shownUnpinDialog = true" v-if="hasPermToThreads('unpin') && thread.pinned"><icon icon="gavel"/></button>

                        <ConfirmDialog v-if="shownMakeEndlessDialog" v-bind:title="$t('dialogs.makeThreadEndless.title')" v-bind:message="$t('dialogs.makeThreadEndless.message')" v-on:confirm="makeThreadEndless" v-on:cancel="shownMakeEndlessDialog = false" />
                        <button class="btn btn-seventh" @click="shownMakeEndlessDialog = true" v-if="hasPermToThreads('make_endless') && !thread.endless"><icon icon="infinity"/></button>

                        <!-- <button class="btn btn-seventh" @click="closeThread" v-if="hasPermToThreads('pin')"><icon icon="gavel"/></button> -->
                    </div>
                </div>
            </div>

            <div class="files-preview" v-if="hasFiles">
                <div class="file" v-for="(file, index) in source.files">
                    <div v-if="file.extension === 'webm'" class="video-preview"
                        :style="{
                            backgroundImage: `url(${$host()}/res/thumbs/${file.uuid}.png)`
                        }" @click="playVideo(file)"></div>
                    <img
                        v-else
                        :src="`${$host()}/res/thumbs/${file.uuid}.png`"
                        :data-src="`${$host()}/res/uploads/${file.uuid}.${file.extension}`"
                        :key="index">
                    <div class="video-play-overlay" v-if="file.extension === 'webm'">
                        <icon icon="play" />
                    </div>
                    <div class="hover-info">
                        <div class="extension">{{ file.extension.toUpperCase() }}</div>
                        <div class="size">{{ $bytesToSize(file.size) }}</div>
                    </div>
                </div>
            </div>

            <div class="files-preview" v-if="imgurLinks.length">
                <div class="file" v-for="(file, index) in imgurLinks">
                    <img
                        :src="`https://i.imgur.com/${file}t.png`"
                        :data-src="`https://i.imgur.com/${file}.png`"
                        :key="index" />
                    <div class="hover-info">
                        <div class="extension">PNG</div>
                        <div class="size">***</div>
                    </div>
                </div>
            </div>

            <div class="files-preview video-preview" v-if="youtubeLinks.length">
                <YoutubeVideoPreview v-for="(video, index) in youtubeLinks" v-bind:video="video" v-bind:key="index"/>
            </div>
        </div>

        <div class="comment" ref="comment" style="max-height: 200px; overflow: auto;">
            <div class="subject combination-3" v-if="source.subject"><b style="font-size: 16px">  <span v-html="source.subject"></span>  </b></div>
            <div v-html="source.comment"></div>
        </div>

        <!-- <button v-if="!shownFull" @click="showFull" class="btn btn-1">Read more</button> -->
        
        <div class="replies">
            <button v-if="repliesMap[source.num]" @click="showReplies" class="btn btn-2 btn-a">{{ repliesMap[source.num].length }} <icon icon="code-branch" /></button>
            <!-- <button v-if="goThread" @click="goToThread(goThread)" class="btn btn-4 btn-a">
                <icon icon="external-link-alt" />
            </button> -->
        </div>

        <center v-if="isLastEndless" class="forward">
            <button class="btn btn-a btn-3" @click="$parent.$parent.$parent.loadNextPage"><icon icon="angle-double-down" /></button>
        </center>
    </div>
    <div v-else @click="show" ref="showref" class="post-hidden">
        <button v-html='`${ source.num } ${ source.comment.noTags().trunc(25) }`'></button>
    </div>
</template>

<script>
    import moment from 'moment'
    import YoutubeVideoPreview from './YoutubeVideoPreview'
    import Regexes from '../../../../app/Regexes'
    import Hidden from '../../../../app/Hidden'
    import Post  from '../../../../services/Post'
    import Thread  from '../../../../services/Thread'
    import BanDialog from '@/components/App/Dialogs/BanDialog'
    import ReportDialog from '@/components/App/Dialogs/ReportDialog'
    import ConfirmDialog from '@/components/App/Dialogs/ConfirmDialog'
    import Admin from '@/services/Admin'

    export default {
        components: { YoutubeVideoPreview, ReportDialog, BanDialog, ConfirmDialog },
        // props: [ "index", "thread", "post", "posts", "repliesMap", "dir", "postsMap", "buttons" ],
        props: {
            index: {
                type: Number
            },
            source: {
                type: Object,
                default () {
                  return {}
                }
            },
            overrideIsThread: { type: Boolean },
            thread: { type: Object },
            buttons: { type: Boolean },
            threadRoute: { type: Boolean },
            lang: { type: String },
            boardShort: { type: String },
            dir: { type: String },
            posts: { },
            repliesMap: { type: Object },
            postsMap: { type: Object },
            goThread: { type: Number },
            visibleList: { type: Array },
            board: { type: Object },
        },

        data() {
            return {
                agoupdate: null,
                selected: false,
                hidden: false,
                shownDeleteDialog: false,
                shownOpenDialog: false,
                shownCloseDialog: false,
                shownPinDialog: false,
                shownUnpinDialog: false,
                shownBanDialog: false,
                shownReportDialog: false,
                shownMakeEndlessDialog: false,
                shownFull: true,
                shownOptions: false,
                rendered:  false,
                innerActions: false,
                youtubeLinks: [],
                imgurLinks: [],
                INTERVAL: null,
                isMy: false,
                visible: false,
            }
        },

        computed: {
            tripcolors () {
                if (this.source.trip) {
                  let arr = [];
                  let offset = 0;
                  while (offset < this.source.trip.length) {
                      let part = this.source.trip.substr(offset, 6);
                      if (part.length == 6) {
                        arr.push("#" + part);
                      }
                      offset += 6;
                  }
                  return arr;
                } else {
                    return []
                }
            },

            timeAgo: {
               cache: false,
               get () {
                   const local = new Date().getTime()
                   const seconds = Math.floor(local/1000 - this.source.at)

                   const days = Math.floor(seconds / (24 * 60 * 60))
                   const hours = Math.floor(seconds / (60 * 60))
                   const minutes = Math.floor(seconds / 60)

                   if (days > 2) {
                       return this.$formatDate(this.source.at)
                   }

                   if (hours >= 24) {
                       return this.$t('timeLabel.yesterday')
                   } else if (hours <= 24 && hours >= 1) {
                       return `${ hours } ${ this.$t('timeLabel.hoursAgo') }`
                   } else if (minutes >= 1) {
                       return `${ minutes } ${ this.$t('timeLabel.minutesAgo') }`
                   } else {
                       return `${ seconds } ${ this.$t('timeLabel.secondsAgo') }`
                   }
               }
            },

            isLastEndless () {
                return this.board && this.thread.endless && ((this.$parent.$parent.$parent.page === 0 && this.index === this.board.bumplimit) || (this.$parent.$parent.$parent.page != 0 && this.index + 1 === this.board.bumplimit))
            },

            isThread () {
                return this.source.isThread || this.overrideIsThread
            },

            hasFiles() {
                return this.source.files.length != 0
            },

            filesInfo() {
                if (this.source.files.length > 1) {
                    let totalSize = 0
                    this.source.files.map(v => totalSize += v.size)

                    return `${this.$bytesToSize(totalSize)} ${this.source.files.length} ${this.$t('labels.files')}`
                } else {
                    const file = this.source.files[0]

                    return `${file.extension.toUpperCase()} ${this.$bytesToSize(file.size)}`
                }
            }
        },

        beforeDestroy() {
            if (this.agoupdate) {
                clearInterval(this.agoupdate)
            }
        },

        async mounted() {
            this.hidden = Hidden.isHidden(this.dir, this.source.num)
            this.update()
            this.applyInnerActions()

            // update time ago
            this.agoupdate = setInterval(() => {
                this.$forceUpdate()
            }, 1000);

            // wtf?
            this.$parent.$parent.$parent.$refs[this.source.num] = this

            if (this.thread && this.source.num === this.thread.num) {
                this.$parent.$parent.$parent.$refs.opPost = this
            }

            this.isMy = this.$store.state.myPosts[`${this.lang}/${this.boardShort}/${this.source.num}`]

            // if (!this.$mobile() && this.persisted['options.source.preload'] == true) {
            //     this.setCommentHeightInterval()
            // }
        },

        updated() {
            this.applyInnerActions()
            this.update()
        },

        methods: {
            playVideo (file) {
                const url = `${this.$host()}/res/uploads/${file.uuid}.${file.extension}`

                this.$root.$refs.videoPlayer.play({
                    url: url,
                    title: file.uuid,
                    type: 'raw'
                })
            },

            showPinnedPosts () {
                let list = new Array()
                for (const postNum of this.thread.pinnedPosts) {
                    list.push(this.postsMap[postNum])
                }

                this.$popup({ thread: this.thread, list, posts: this.posts, repliesMap: this.repliesMap, postsMap: this.postsMap })
            },

            async pinUnpinPost () {
                let params = this.$route.params
                if (this.source.pinned) {
                    try {
                        await Thread.unpinPost({ secret: this.thread.secret, lang: params.lang, dir: params.dir, thread: this.thread.num, num: this.source.num })
                        this.source.pinned = false
                        this.thread.pinnedPosts.splice(this.thread.pinnedPosts.indexOf(this.source.num), 1)
                        this.$notify({
                            "group": "main",
                            "type":  "success",
                            "title": this.$t("api.admin.postUnpin.success"),
                        })
                    } catch(e) {
                        console.error(e)
                        this.$notify({
                            "group": "main",
                            "type":  "error",
                            "title": this.$t("api.admin.postUnpin.failed"),
                            "text":  this.$t(`apiErrorCodes.${e.code}`)
                        })
                    }
                } else {
                    try {
                        await Thread.pinPost({ secret: this.thread.secret, lang: params.lang, dir: params.dir, thread: this.thread.num, num: this.source.num })
                        this.source.pinned = true
                        this.thread.pinnedPosts.push(this.source.num)
                        this.thread.pinnedPosts.sort((a, b) => a > b)
                        this.$notify({
                            "group": "main",
                            "type":  "success",
                            "title": this.$t("api.admin.postPin.success"),
                        })
                    } catch(e) {
                        console.error(e)
                        this.$notify({
                            "group": "main",
                            "type":  "error",
                            "title": this.$t("api.admin.postPin.failed"),
                            "text":  this.$t(`apiErrorCodes.${e.code}`)
                        })
                    }
                }

                this.$parent.$parent.$parent.$refs.opPost.$forceUpdate()
            },

            goToThread(num) {
                this.$router.push({ name: 'thread', params: { lang: this.$route.params.lang, dir: this.$route.params.dir, num } })
                this.$popupHide()
            },
            async showDetails () {
                const params = this.$route.params
                let details = await Admin.getPostDetails(this.$store.state.admin.token, { lang: params.lang, dir: params.dir, num: this.source.num })

                this.$notify({
                    "group": "main",
                    "type":  "success",
                    "text": `IP ${ details.ip } <br />TokenID ${ details.token }`,
                })
            },

            banPoster () {
                this.shownBanDialog = true
            },

            select () {
                this.selected = !this.selected

                this.$selectPost(this, this.thread, this.source)
            },

            async pinThread () {
                let params = this.$route.params
                this.shownPinDialog = false

                this.$block()
                try {
                    let res = await Admin.pinThread(this.$store.state.admin.token, { 'lang': params.lang, 'dir': params.dir, 'num': this.source.num })
                    this.thread.pinned = true
                    this.$notify({
                        "group": "main",
                        "type":  "success",
                        "title": this.$t("api.admin.threadPin.success"),
                    })
                } catch(e) {
                    console.error(e)
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.admin.threadPin.failed"),
                        "text":  this.$t(`apiErrorCodes.${e.code}`)
                    })
                }
                this.$unblock()
            },

            async makeThreadEndless () {
                let params = this.$route.params
                this.shownMakeEndlessDialog = false

                this.$block()
                try {
                    let res = await Admin.makeThreadEndless(this.$store.state.admin.token, { 'lang': params.lang, 'dir': params.dir, 'num': this.source.num })
                    this.thread.endless = true
                    this.$notify({
                        "group": "main",
                        "type":  "success",
                        "title": this.$t("api.admin.makeThreadEndless.success"),
                    })
                } catch(e) {
                    console.error(e)
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.admin.makeThreadEndless.failed"),
                        "text":  this.$t(`apiErrorCodes.${e.code}`)
                    })
                }
                this.$unblock()
            },

            async unpinThread () {
                let params = this.$route.params
                this.shownUnpinDialog = false

                this.$block()
                try {
                    let res = await Admin.unpinThread(this.$store.state.admin.token, { 'lang': params.lang, 'dir': params.dir, 'num': this.source.num })
                    this.thread.pinned = false
                    this.$notify({
                        "group": "main",
                        "type":  "success",
                        "title": this.$t("api.admin.threadUnpin.success"),
                    })
                } catch(e) {
                    console.error(e)
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.admin.threadUnpin.failed"),
                        "text":  this.$t(`apiErrorCodes.${e.code}`)
                    })
                }
                this.$unblock()
            },

            async closeThread () {
                let params = this.$route.params
                this.shownCloseDialog = false

                this.$block()
                try {
                    let res = await Admin.closeThread(this.$store.state.admin.token, { 'lang': params.lang, 'dir': params.dir, 'num': this.source.num })
                    this.thread.closed = true
                    this.$notify({
                        "group": "main",
                        "type":  "success",
                        "title": this.$t("api.admin.threadClose.success"),
                    })
                } catch(e) {
                    console.error(e)
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.admin.threadClose.failed"),
                        "text":  this.$t(`apiErrorCodes.${e.code}`)
                    })
                }
                this.$unblock()
            },

            async openThread () {
                let params = this.$route.params
                this.shownOpenDialog = false

                this.$block()
                try {
                    let res = await Admin.openThread(this.$store.state.admin.token, { 'lang': params.lang, 'dir': params.dir, 'num': this.source.num })
                    this.thread.closed = false
                    this.$notify({
                        "group": "main",
                        "type":  "success",
                        "title": this.$t("api.admin.threadOpen.success"),
                    })
                } catch(e) {
                    console.error(e)
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.admin.threadOpen.failed"),
                        "text":  this.$t(`apiErrorCodes.${e.code}`)
                    })
                }
                this.$unblock()
            },

            async pin () {

            },

            async deletePost() {
                let params = this.$route.params
                this.shownDeleteDialog = false

                this.$block()
                if (this.hasPermTo('delete')) {
                    try {
                        let res = await Admin.deletePost(this.$store.state.admin.token, { 'lang': params.lang, 'dir': params.dir, 'num': this.source.num })
                        this.$notify({
                            "group": "main",
                            "type":  "success",
                            "title": this.$t("api.admin.postDelete.success"),
                        })
                        this.source.isDeleted = true
                    } catch(e) {
                        this.$notify({
                            "group": "main",
                            "type":  "error",
                            "title": this.$t("api.admin.postDelete.failed"),
                            "text":  this.$t(`apiErrorCodes.${e.code}`)
                        })
                    }
                } else {
                    if (this.thread.secret) {
                        try {
                            let res = await Thread.deletePost({ 'lang': params.lang, 'dir': params.dir, 'thread': this.thread.num, 'num': this.source.num, 'secret': this.thread.secret })
                            this.$notify({
                                "group": "main",
                                "type":  "success",
                                "title": this.$t("api.admin.postDelete.success"),
                            })
                            this.source.isDeleted = true
                        } catch(e) {
                            this.$notify({
                                "group": "main",
                                "type":  "error",
                                "title": this.$t("api.admin.postDelete.failed"),
                                "text":  this.$t(`apiErrorCodes.${e.code}`)
                            })
                        }
                    }
                }
                this.$unblock()
                this.thread.posts[this.index].isDeleted = true
                this.$forceUpdate()
            },

            hasPermTo(perm) {
                let params = this.$route.params
                let label = `langs::${params.lang}::boards::${params.dir}::posts::${perm}`

                return this.$store.state.admin.approved && this.$store.state.admin.perms.indexOf(label) != -1
            },

            hasPermToThreads(perm) {
                let params = this.$route.params
                let label = `langs::${params.lang}::boards::${params.dir}::threads::${perm}`

                return this.$store.state.admin.approved && this.$store.state.admin.perms.indexOf(label) != -1
            },

            reply () {
                this.shownOptions = false
                this.$popupHide()
                this.$reply({ lang: this.$route.params.lang, dir: this.$route.params.dir, thread: this.thread.num, num: this.source.num })
            },

            closeOptions () {
                this.shownOptions = false;
            },

            setCommentHeightInterval() {
                // let oldHeight = 0

                // this.INTERVAL = setInterval(() => {
                //     if (this.$refs.comment && this.$refs.comment.offsetHeight > oldHeight) {
                //         /* code */
                //         if (this.visible && !this.hidden && !this.rendered)
                //             this.checkCommentHeight()
                        
                //         oldHeight = this.$refs.comment.offsetHeight
                //     }

                // }, 100)
            },

            loadImage(event) {
                console.log(event)
            },

            // openFiles() {
            //     if (!this.source.files.length)
            //         return

            //     if (this.source.files.length == 1) {
            //         const file = this.source.files[0]

            //         return this.$popupFile({
            //             source: `${this.$host()}/res/uploads/${file.uuid}.${file.extension}`,

            //             label: file.uuid,
            //             width: file.width,
            //             height: file.height,
            //             size: file.size,
            //         })
            //     }
            // },

            // context(event) {
            //     if (this.buttons)
            //         this.$context({ event, options: [
            //             this.hidden
            //                 ? { title: this.$t("context.source.show"), icon: "eye", onclick: this.show }
            //                 : { title: this.$t("context.source.hide"), icon: "eye-slash", onclick: this.hide }
            //         ]})
            // },

            update() {
                // if (!this.hidden && !this.rendered)
                //     this.checkCommentHeight()

                if (this.$refs.comment && (this.$refs.comment.style.maxHeight == "none" || this.$refs.comment.offsetHeight < 200))
                    this.shownFull = true
            },

            hide() {
                this.selected = false
                this.shownOptions = false
                Hidden.hide(this.dir, this.source.num)
                this.hidden = true
            },

            show(ev) {
                if (ev.target != this.$refs.showref) {
                    return;
                }

                Hidden.show(this.dir, this.source.num)
                this.innerActions = false
                this.applyInnerActions()
                this.hidden = false
                this.shownFull = false
            },

            showReplies() {
                this.$popup({ thread: this.thread, list: this.repliesMap[this.source.num], posts: this.posts, repliesMap: this.repliesMap, postsMap: this.postsMap, lang: this.lang, boardShort: this.boardShort })
            },

            goToPost(num) {
                window.location.href = '#' + this.dir + '-' + num
            },

            showFull() {
                this.$refs.comment.style.maxHeight = "none"
                this.shownFull = true
            },

            checkCommentHeight() {
                if (this.$refs.comment.style.maxHeight != "none" && this.$refs.comment.offsetHeight >= 200) {
                    this.rendered = true
                    this.shownFull = false
                }
            },

            applyInnerActions() {
                if (this.innerActions || !this.$refs.comment)
                    return

                this.innerActions = true
                let stags = 0

                // process all mark-urand
                const urands = this.$refs.comment.getElementsByClassName("mark-urand")
                for (const urand of urands) {
                    stags += 1

                    if (stags < this.$store.state.settings.specialTagsLimit) {
                        // get available variants
                        let variants = urand.innerText.split('|')
                        let variant = variants[Math.floor(Math.random()*variants.length)]

                        $(urand).addClass('done').text(variant)
                    }
                }

                // process textwalls
                const textwalls = this.$refs.comment.getElementsByClassName("textwall")
                for (const ttt of textwalls) {
                    stags += 1
                    const textwall = $(ttt)

                    if (stags < this.$store.state.settings.specialTagsLimit) {
                        let content = textwall.find("> .content")
                        textwall.find("> .title")[0].addEventListener('click', (event) => {
                            if (content.hasClass('shown')) {
                                content.removeClass('shown')
                            } else {
                                content.addClass('shown')
                            }
                        })
                    }
                }

                // get links to parse youtube-links
                let counterYoutube = 0
                let counterImgur = 0
                const links = this.$refs.comment.getElementsByClassName("link")
                let ytMap = {}
                let imgurMap = {}
                for (const link of links) {
                    // youtubeLinks
                    let m;
                    const ytRegex = /^((?:https?:)?\/\/)?((?:www|m)\.)?((?:youtube\.com|youtu.be))(\/(?:[\w\-]+\?v=|embed\/|v\/)?)([\w\-]+)(\S+)?$/gm;
                    const linkDetected = false

                    while ((m = ytRegex.exec(link.href)) !== null) {
                        // This is necessary to avoid infinite loops with zero-width matches
                        if (m.index === ytRegex.lastIndex) {
                            ytRegex.lastIndex++;
                        }
                        
                        // The result can be accessed through the `m`-variable.
                        m.forEach((match, groupIndex) => {
                            if (groupIndex === 5 && ytMap[match] === undefined) {
                                if (counterYoutube > this.$store.state.settings.youtubePreloadLimit) {
                                    return
                                } else {
                                    counterYoutube += 1
                                }

                                this.youtubeLinks.push(match)
                                ytMap[match] = 1
                                linkDetected = true
                            }
                        });
                    }

                    if (linkDetected === false) {
                        const imgurRegex = /https?:\/\/imgur.com\/([a-zA-Z0-9]+)$/gi;

                        while ((m = imgurRegex.exec(link.href)) !== null) {
                            // This is necessary to avoid infinite loops with zero-width matches
                            if (m.index === imgurRegex.lastIndex) {
                                imgurRegex.lastIndex++;
                            }
                            
                            // The result can be accessed through the `m`-variable.
                            m.forEach((match, groupIndex) => {
                                if (groupIndex === 1 && imgurMap[match] === undefined) {
                                    if (counterImgur > this.$store.state.settings.imgurPreloadLimit) {
                                        return
                                    } else {
                                        counterImgur += 1
                                    }
                                    this.imgurLinks.push(match)
                                    imgurMap[match] = 1
                                }
                            });
                        }
                    }

                }

                // get replies
                const replies = this.$refs.comment.getElementsByClassName("reply")
                const repliesBoard = this.$refs.comment.getElementsByClassName("reply-board")
                const repliesBoardLang = this.$refs.comment.getElementsByClassName("reply-length")
                let repliesCounter = 0

                for (const reply of replies) {   
                    if (repliesCounter < this.$store.state.settings.repliesLimit) {
                        const j        = $(reply)
                        const num      = reply.dataset.num
                        const lang     = reply.dataset.lang || this.lang
                        const short    = reply.dataset.short || this.boardShort
                        const post     = this.postsMap[num]

                        let isMy = this.$store.state.myPosts[`${lang}/${short}/${num}`]

                        if (this.thread && num == this.thread.num) {
                            j.addClass('to-op')
                        } else if (isMy !== undefined) {
                            j.addClass('to-me')
                        }

                        if (post) {
                            const text = post.comment.noTags().trunc(20).trim()
                            if (text.length)
                                j.append($('<span class="preview"></span>').html(text))
                            reply.addEventListener('click', event => {
                                this.$popup({ thread: this.thread, post, posts: this.posts, repliesMap: this.repliesMap, postsMap: this.postsMap, lang: lang, boardShort: short })
                            })
                        } else {
                            let l = false
                            reply.addEventListener('click', event => {
                                if (l === false) {
                                    l = true
                                    Post.get({ 'lang': lang, 'dir': short, num })
                                    .then((post) => {
                                        this.$popup({ post, posts: [], repliesMap: {}, postsMap: {}, goThread: { lang, 'dir': short, num: post.thread }, lang: lang, boardShort: short })
                                        l = false
                                    })
                                    .catch((e) => {
                                        this.$notify({
                                            "group": "main",
                                            "type":  "error",
                                            "title": this.$t("api.posts.getting.failed"),
                                            "text":  this.$t(`apiErrorCodes.${e.code}`)
                                        })
                                        l = false
                                    })
                                }
                            })
                            // let split = reply.innerText.split('/')
                            // switch (split.length) {
                            //     case 2: {

                            //         break
                            //     }

                            //     case 1: {

                            //         break
                            //     }

                            //     case 0: {
                            //         if (/^[a-z]*$/.test(split[0])) {
                            //             reply.addEventListener('click', event => {
                            //                 this.$router.push({ name: 'board', params: { lang: this.$route.params.lang, dir: split[0] } })
                            //             })
                            //         }
                            //         break
                            //     }
                            // }
                        }
                    } else {
                        reply.style.opacity = .2
                    }
                    // else
                    //     reply.addEventListener('click', event => {
                    //         const post = Post.get()
                    //         // this.$popup({ thread: this.thread, post, posts: this.posts, repliesMap: this.repliesMap, postsMap: this.postsMap })
                    //     })

                    repliesCounter++
                }
            }
        },
    }
</script>
