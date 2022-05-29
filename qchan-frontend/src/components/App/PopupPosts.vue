<template>
    <div v-if="popups.length" id="popup-posts" v-viewer="viewerOptions">
        <div class="block" @click="hideLastPost"></div>
        <!-- <button class="btn btn-1 hide-all-btn" @click="hideAll"><icon icon="times"/></button> -->
        <div class="entries">
            <div v-for="popup in popups" class="entry-wrap">
                <div class="entry">
                    <div class="body">
                        <Post v-if="popup.post" :boardShort="popup.boardShort" :lang="popup.lang" :thread="popup.thread" :postsMap="popup.postsMap || {}" :dir="$route.params.dir" :source="popup.post" :repliesMap="popup.repliesMap || []" :posts="popup.posts || []" :goThread="popup.goThread"></Post>
                        <Post v-if="popup.list" v-for="(post, index) in popup.list" :key="index" :boardShort="popup.boardShort" :lang="popup.lang" :thread="popup.thread" :postsMap="popup.postsMap || {}" :index="popup.typeThreadPreview ? ( index === 0 ? 0 : popup.typeThreadPreview.count - popup.list.length + index ) : null" :dir="$route.params.dir" :source="post" :repliesMap="popup.repliesMap || []" :posts="popup.posts || []"></Post>
                    </div>
                    <button v-if='popup.goThread' class="btn btn-5 btn-a w100" @click="goToThread(popup.goThread)"> <icon icon="forward" /> </button>
                    <div class="blur" @click="hideLastPost" v-if="popups[popups.length-1] != popup"></div>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
    import Vue from 'vue'
    import Post from './Content/Frame/PostEntry'

    export default {
        components: { Post },

        data() {
            return {
                popups: [],
                viewerOptions: {
                    url: 'data-src'
                }
            }
        },

        mounted() {
            Vue.prototype.$popup = params => {
                this.popups.push(params)
            }

            Vue.prototype.$popupHide = params => {
                this.popups = []
            }
        },

        methods: {
            goToThread({ lang, dir, num }) {
                this.$router.push({ name: 'thread', params: { lang, dir, num } })
                this.$popupHide()
            },

            hideAll() {
                this.popups = []
            },

            hideLastPost() {
                this.popups.pop()
            }
        }
    }
</script>