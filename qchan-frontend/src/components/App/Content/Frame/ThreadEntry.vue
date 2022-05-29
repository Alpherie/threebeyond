<template>
    <div v-if="!hidden" class="thread" style="overflow: none">

        <Post :board="board" :boardShort="board.short" :lang="board.lang" :buttons="false" :overrideIsThread="true" :postsMap="{}" :thread="thread" :source="thread.posts[0]" :posts="[]" :repliesMap="{}"></Post>

        <div class="header pl-2 w100-centered-mobile buttons-wrap">
            <button @click="open" class="btn btn-4 btn-a"><icon icon="external-link-alt"/></button>
            <button v-if="thread.posts.length > 1" @click="showReplies" class="btn btn-2 btn-a">{{ thread.count-1 }} <icon icon="code-branch" /></button>
            <button @click="hide" class="btn btn-1 btn-a"><icon icon="eye-slash"/></button>
        </div>
    </div>
    
    <div v-else class="thread-hidden">
        <button @click="show" class="btn btn-1" v-html="thread.posts[0].subject ? thread.posts[0].subject : thread.posts[0].comment.noTags().trunc(50)"></button>
    </div>
</template>

<script>
    import Post from './PostEntry'
    import Hidden from '../../../../app/Hidden'

    export default {
        components: { Post },

        props: [ 'thread', 'board' ],

        data() {
            return {
                hidden: false,
            }
        },

        methods: {
            showReplies() {
                this.$popup({ typeThreadPreview: { count: this.thread.count }, thread: this.thread, list: this.thread.posts, posts: {}, repliesMap: {}, postsMap: {}, lang: this.board.lang, boardShort: this.board.short })
            },

            hide() {
                Hidden.hide(this.dir, this.thread.num)
                this.hidden = true
            },

            show() {
                Hidden.show(this.dir, this.thread.num)
                this.hidden = false
            },

            open() {
                this.$router.push({ name: "thread", params: { lang: this.$route.params.lang, dir: this.$route.params.dir, num: `${this.thread.num}` } })
            }
        },

        mounted() {
            this.hidden = Hidden.isHidden(this.dir, this.thread.num)
            this.thread.posts[0].isThread = true
        }
    }
</script>