<template>
    <div id="writeform" v-on:keydown="onKeydown">
        <PopupDCaptcha v-if="dcaptchaActive" v-on:cancel="dcaptchaActive = false" v-on:accepted="sendUsingDCaptcha" />
        <div v-if="opened" class="opened">
            <div class="section combination-2" :class="{ 'open': openedSection == 'location' }">
                    <button class="head btn btn-2" v-bind:class="{ 'btn-a': openedSection == 'location' }" @click="openedSection = 'location'">{{ $t('writeform.location') }}</button>
                    <div class="body" v-if="openedSection == 'location'">
                        <input v-model="lang" :placeholder="$t('writeform.lang')" type="text">
                    </div>
                    <div class="body" v-if="openedSection == 'location'">
                        <input v-model="board" :placeholder="$t('writeform.board')" type="text">
                    </div>
                    <div class="body" v-if="openedSection == 'location'">
                        <input v-model="thread" :placeholder="$t('writeform.thread')" type="text">
                    </div>
                <div>
                    <vue-hcaptcha ref="hcaptcha" size="invisible" v-on:verify="sendUsingHCaptcha" sitekey="39636c74-4555-4686-b879-40912f147ac3"></vue-hcaptcha>
                </div>
            </div>

            <div class="section combination-4" :class="{ 'open': openedSection == 'headers' }">
                <button class="head btn btn-4" v-bind:class="{ 'btn-a': openedSection == 'headers' }" @click="openedSection = 'headers'">{{ $t('writeform.postInfo') }}</button>
                <div class="body flex">
                    <div class="left post-info">
                        <div class="body" v-if="openedSection == 'headers'">
                            <input v-model="name" :placeholder="$t('writeform.name')" type="text">
                        </div>
                        <div class="body" v-if="openedSection == 'headers'">
                            <input v-model="email" :placeholder="$t('writeform.email')" type="text">
                        </div>
                        <div class="body" v-if="openedSection == 'headers'">
                            <input v-model="subject" :placeholder="$t('writeform.subject')" type="text">
                        </div>
                    </div>

                    <!-- <div class="right file-input">
                        <div class="label">{{ files.length }} files</div>
                        <input type="file" class="flow" multiple @change="updateFilesLabel">
                    </div> -->
                </div>
            </div>

            <div class="section combination-5 section-attachments" :class="{ 'open': openedSection == 'attachments' }">
                <button class="head btn btn-5" v-bind:class="{ 'btn-a': openedSection == 'attachments' }" @click="openedSection = 'attachments'">{{ $t('writeform.attachments') }} ({{ availableFormats.join(", ") }})</button>
                <div class="body attachments-body" v-if="openedSection == 'attachments'" v-viewer>
                    <div class="attachments">
                        <button class="btn btn-5 btn-a add">
                            <icon icon="plus" />
                            <input type="file" multiple ref="loadFile" @change="registerFiles">
                        </button>
                        <div class="file" v-for="(file, num) in files">
                            <div class="container">
                                <img :src="file.url" v-if="file.shorttype == 'image'">
                                <video :src="file.url" v-if="file.shorttype == 'video'" muted></video>
                            </div>
                            <span class="settings buttons-vwrap">
                                <button class="btn btn-1" @click="deleteFile(num)"><icon icon="trash" /></button>
                            </span>
                        </div>
                    </div>
                </div>
            </div>

            <div class="section combination-3 comment-section" :class="{ 'open': openedSection == 'body' }">
                <button class="head btn btn-3" v-bind:class="{ 'btn-a': openedSection == 'body' }" @click="openedSection = 'body'">{{ $t('writeform.comment') }}</button>
                <div class="body comment-body" v-if="openedSection == 'body'">
                    <textarea v-model="comment" ref="comment" :placeholder="$t('writeform.comment')"></textarea>
                </div>
                <div class="body markup-buttons buttons-wrap" v-if="openedSection == 'body'">
                    <button class="btn btn-3 btn-a" @click="addTag('highlight')">H</button>
                    <button class="btn btn-3 btn-a" @click="addTag('b')">B</button>
                    <button class="btn btn-3 btn-a" @click="addTag('i')">I</button>
                    <button class="btn btn-3 btn-a" @click="addTag('spoiler')">S</button>
                    <button class="btn btn-3 btn-a" @click="addTextwall">T</button>
                    <button class="btn btn-3 btn-a" @click="addQuote">&gt;</button>
                    <button class="btn btn-3 btn-a" @click="addUrand">UR</button>
                </div>
            </div>
            <div class="section combination-1">
                <div class="body">
                    <button class="btn btn-4" :disabled="isSending" @click="trysend"><icon icon="reply"/></button>
                    <!-- <button class="btn btn-3" @click="clear">Clear</button> -->
                    <!-- <button class="btn btn-2" @click="applyDefault">Default</button> -->
                    <button class="btn btn-1" @click="toggle"><icon icon="times"/></button>
                </div>
            </div>
        </div>

        <!-- <button v-else class="open-btn btn btn-4" @click="toggle"><icon icon="reply"/></button> -->
    </div>
</template>

<script>
    import Thread        from '../../services/Thread'
    import Post          from '../../services/Post'
    import PopupDCaptcha from "./PopupDCaptcha"
    import VueHcaptcha   from '@hcaptcha/vue-hcaptcha';
    import Vue           from 'vue'

    export default {
        components: { PopupDCaptcha, VueHcaptcha },

        data() {
            return {
                availableFormats: ["PNG", "JPG", "JPEG", "GIF", "WEBM"],
                opened:  false,
                default: { lang: null, board: null, thread: null },

                openedSection: 'body',

                files: [],
                lang: null,
                board:   null,
                thread:  null,
                name:    null,
                email:   null,
                subject: null,
                comment: "",
                isSending: false,

                dcaptchaActive: false
            }
        },

        mounted() {
            Vue.prototype.$setWriteformDefault = this.setDefault
            Vue.prototype.$reply = this.reply

            this.$root.$refs.writeform = this
        },

        methods: {
            deleteFile (id) {
                URL.revokeObjectURL(this.files[id].url)
                this.files.splice(id, 1)
            },

            registerFiles (e) {
                let files = e.target.files

                for (const file of files) {
                    file.shorttype = file.type.split('/')[0]
                    if (this.availableFormats.indexOf(file.name.split('.').pop().toUpperCase()) == -1) {
                        this.$notify({
                            "group": "main",
                            "type":  "error",
                            "title": this.$t("writeform.fileLoadFailed.title", { name: file.name }),
                            "text":  this.$t(`writeform.fileLoadFailed.message`)
                        })

                        continue
                    }
                    file.url = URL.createObjectURL(file)
                    this.files.push(file)
                }
            },

            revokeFiles (e) {
                for (const file of this.files) {
                    URL.revokeObjectURL(file.url)
                }

                this.files = []
            },

            addTag (tag) {
                let start = this.$refs.comment.selectionStart
                let end = this.$refs.comment.selectionEnd

                if (start === end) {
                    let left = this.comment.substr(0, start)
                    let right = this.comment.substr(start, this.comment.length - start)

                    this.comment = left + `[${tag}][/${tag}]` + right
                    let newstart = left.length + 2 + tag.length
                } else {
                    let left = this.comment.substr(0, start)
                    let content = this.comment.substr(start, end-start)
                    let right = this.comment.substr(end, this.comment.length - end)

                    $(this.$refs.commen).focus()
                    this.comment = left + `[${tag}]` + content + `[/${tag}]` + right
                }
                $(this.$refs.comment).focus()
            },

            addUrand () {
                let start = this.$refs.comment.selectionStart
                let end = this.$refs.comment.selectionEnd

                if (start === end) {
                    let left = this.comment.substr(0, start)
                    let right = this.comment.substr(start, this.comment.length - start)

                    this.comment = left + `[urand]a|b|c[/urand]` + right
                } else {
                    let left = this.comment.substr(0, start)
                    let content = this.comment.substr(start, end-start)
                    let right = this.comment.substr(end, this.comment.length - end)

                    this.comment = left + `[urand]` + content + `|other[/urand]` + right
                }

                $(this.$refs.comment).focus()
            },

            addTextwall () {
                let start = this.$refs.comment.selectionStart
                let end = this.$refs.comment.selectionEnd

                if (start === end) {
                    let left = this.comment.substr(0, start)
                    let right = this.comment.substr(start, this.comment.length - start)

                    this.comment = left + `[textwall title=Title][/textwall]` + right
                } else {
                    let left = this.comment.substr(0, start)
                    let content = this.comment.substr(start, end-start)
                    let right = this.comment.substr(end, this.comment.length - end)

                    this.comment = left + `[textwall title=Title]` + content + `[/textwall]` + right
                }

                $(this.$refs.comment).focus()
            },

            addQuote () {
                let start = this.$refs.comment.selectionStart
                let end = this.$refs.comment.selectionEnd

                if (start === end) {
                    let left = this.comment.substr(0, start)
                    let right = this.comment.substr(start, this.comment.length - start)

                    this.comment = left + `>Body` + right
                } else {
                    let left = this.comment.substr(0, start)
                    let content = this.comment.substr(start, end-start)
                    let right = this.comment.substr(end, this.comment.length - end)

                    this.comment = left + `>` + content + `\n` + right
                }

                $(this.$refs.comment).focus()
            },

            onKeydown (ev) {
                if (ev.ctrlKey && ev.keyCode === 13) {
                    this.trysend()
                }
            },

            updateFilesLabel(event) {
                this.files = event.target.files
                this.$forceUpdate()
            },

            reply({ lang, dir, thread, num }) {
                this.lang = lang
                this.board = dir
                this.thread = `${thread}`
                if (this.comment && this.comment.length)
                    this.comment += `\n>>${num}`
                else
                    this.comment  = `>>${num}`

                // this.applyDefault()
                this.opened = true
                setTimeout(v => this.$refs.comment.focus(), 300)
            },

            async trysend () {
                let w = this.$store.state.threadSecrets[`${this.lang}/${this.board}/${this.thread}`]

                if (w) {
                    return await this.send('op', w);
                }

                if (!this.$store.state.admin.approved) {
                    switch (this.$store.state.settings.captchaKind) {
                        case 'dcaptcha': {
                            this.dcaptchaActive = true
                            break
                        }

                        case 'hcaptcha': {
                            this.$refs.hcaptcha.execute()
                            break
                        }

                        default: {
                            console.error('writeform 114')
                            break
                        }
                    }
                } else {
                    await this.send();
                }
            },

            async sendUsingDCaptcha (value) {
                this.dcaptchaActive = false
                this.send('dcaptcha', value)
            },

            sendUsingHCaptcha (value) {
                this.send('hcaptcha', value)
            },

            async send(captchaKind, captchaValue) {
                // 0. validate
                if (this.lang == null || !this.lang.length) {
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.posting.failed"),
                        "text":  this.$t(`api.posting.validate.noLang`)
                    })
                    return
                }

                if (this.board == null || !this.board.length) {
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.posting.failed"),
                        "text":  this.$t(`api.posting.validate.noBoard`)
                    })
                    return
                }

                if (this.comment == null || !this.comment.length) {
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.posting.failed"),
                        "text":  this.$t(`api.posting.validate.noComment`)
                    })
                    return
                }

                this.$block()

                if (this.thread && this.thread.length) {
                    // reply
                    this.isSending = true

                    try {
                        // this.$notify({
                        //     "group": "main",
                        //     "title": this.$t("api.posting.reply.started")
                        // })

                        const post = await Post.create({ captchaKind, captchaValue, lang: this.lang, dir: this.board, thread: this.thread, email: this.email, name: this.name, subject: this.subject, comment: this.comment, files: this.files, authorization: this.$store.state.admin.token })

                        this.$store.commit("markMyPost", { lang: this.lang, dir: this.board, num: post.num })

                        this.revokeFiles()

                        this.isSending = false
                        this.opened = false

                        this.$notify({
                            "group": "main",
                            "type": "success",
                            "title": this.$t("api.posting.reply.success"),
                            "text":  post.num
                        })

                        let idx = `${this.lang}/${this.board}/${this.thread}`
                        const v = this.$root.$refs[idx]
                        if (v)
                            v.getNewPosts()

                        this.$unblock()
                    } catch(e) {
                        this.isSending = false
                        this.$unblock()

                        if (e.code == 'BANNED' || e.code == 'BANNED_TOKEN') {
                            let d = e.details
                            this.$notify({
                                "group": "main",
                                "type":  "error",
                                "title": this.$t("api.posting.reply.failed"),
                                "text":  `${this.$t(`apiErrorCodes.BANNED`, { until: this.$formatDate(d.until), reason: this.$t(`banReasons.${d.reason}`), comment: d.comment, lang: d.lang, board: d.board })}`,
                                "duration": 10000
                            })
                        } else if (e.code === 'LAST_POST_TIMEOUT') {
                            this.$notify({
                                "group": "main",
                                "type":  "error",
                                "title": this.$t("api.posting.reply.failed"),
                                "text":  this.$t(`apiErrorCodes.${e.code}`, { secs: e.details.left })
                            })
                        } else {
                            this.$notify({
                                "group": "main",
                                "type":  "error",
                                "title": this.$t("api.posting.reply.failed"),
                                "text":  this.$t(`apiErrorCodes.${e.code}`)
                            })
                        }

                        throw e
                    }
                } else {
                    this.isSending = true

                    // create thread
                    // this.$notify({
                    //     "group": "main",
                    //     "title": this.$t("api.posting.thread.started")
                    // })

                    try {
                        const thread = await Thread.create({ captchaKind, captchaValue, lang: this.lang, dir: this.board, email: this.email, name: this.name, subject: this.subject, comment: this.comment, files: this.files, authorization: this.$store.state.admin.token })

                        this.revokeFiles()
                        this.isSending = false
                        this.opened = false

                        this.$store.commit("markMyPost", { lang: this.lang, dir: this.board, num: thread.post.num })

                        this.$notify({
                            "group": "main",
                            "type": "success",
                            "title": this.$t("api.posting.thread.success"),
                            // "text":  `${thread.post.num}`
                        })
                        this.$store.commit('registerThreadSecret', {
                            index: `${this.lang}/${this.board}/${thread.post.num}`,
                            value: thread.secret.numberStr
                        })

                        this.$router.push({ "name": "thread", params: { lang: this.lang, dir: this.board, num: `${thread.post.num}` }})

                        this.clear()
                        this.$unblock()
                        // console.log(`%c Present for you: '${thread.secret}' :3`, 'background: white; color: white;')
                    } catch(e) {
                        this.isSending = false

                        if (e.code == 'BORAD_SLOWMODE_REMAINING') {
                            let d = e.details;
                            this.$notify({
                                "group": "main",
                                "type":  "error",
                                "title": this.$t("api.posting.reply.failed"),
                                "text":  `${this.$t(`api.thread_creating.slowmode.message`, { left: Math.floor(d.left/60) })}`,
                                "duration": 6000
                            })
                        } else if (e.code == 'BANNED' || e.code == 'BANNED_TOKEN') {
                            let d = e.details;
                            this.$notify({
                                "group": "main",
                                "type":  "error",
                                "title": this.$t("api.posting.reply.failed"),
                                "text":  `${this.$t(`apiErrorCodes.BANNED`, { until: this.$formatDate(d.until), reason: this.$t(`banReasons.${d.reason}`), comment: d.comment, lang: d.lang, board: d.board })}`,
                                "duration": 10000
                            })
                        } else {
                            this.$notify({
                                "group": "main",
                                "type":  "error",
                                "title": this.$t("api.posting.reply.failed"),
                                "text":  this.$t(`apiErrorCodes.${e.code}`)
                            })
                        }

                        this.$unblock()

                        throw e
                    }
                }

                this.clear()
            },

            clear() {
                this.lang = null
                this.board = null
                this.thread = null
                // this.name = null
                this.email = null
                this.subject = null
                this.comment = ""
                this.files = []
            },

            applyDefault() {
                if (this.default.lang)
                    this.lang = this.default.lang

                if (this.default.board)
                    this.board = this.default.board

                if (this.default.thread !== undefined) {
                    this.thread = `${this.default.thread ? this.default.thread : ''}`
                }
            },

            setDefault(params) {
                this.default = params
            },

            toggle() {
                this.opened = !this.opened
                this.applyDefault()
            }
        }
    }
</script>
