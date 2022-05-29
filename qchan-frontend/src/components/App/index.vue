<template>
    <div id="application-flow">
        <div id="background" v-if="bgShown" :style="{ 'background-image': bgUrl }" ref="background"></div>
        <div id="app">
            <LanguageSelect />
            <VideoPlayer />
            <Context></Context>
            <Spinner></Spinner>

            <!-- <Helpbar></Helpbar> -->
            <div class="container">
                <div class="flex-x flex-1">
                  <Sidebar></Sidebar>
                  <div class="wrapper">
                      <!-- <TopBar></TopBar> -->
                      <Content></Content>
                  </div>
                </div>
            </div>

            <notifications group="main" width="200" position="bottom right" />

            <Writeform></Writeform>
            <PopupPosts></PopupPosts>
            <PopupFile></PopupFile>
            <Navbar></Navbar>
            <Rules></Rules>
        </div>
    </div>
</template>

<script>
    import $           from 'jquery'
    import Vue         from 'vue'
    import Rules       from "./Rules"
    import LanguageSelect from "./LanguageSelect/index"
    import PopupPosts  from "./PopupPosts"
    import PopupFile   from "./PopupFile"
    import Writeform   from "./Writeform"
    import Helpbar     from "./Helpbar/index"
    import Navbar      from "./Navbar/index"
    import Sidebar     from './Sidebar/index'
    import Content     from './Content/index'
    import Spinner     from '../Spinner'
    import VideoPlayer from './VideoPlayer'
    import Context     from './Context'
    import TopBar      from './TopBar'
    import Huita       from './Dialogs/BanDialog'

    import {Axios} from 'axios'

    export default {
        components: { LanguageSelect, VideoPlayer, Rules, Huita, TopBar, Spinner, PopupFile, Context, Writeform, PopupPosts, Navbar, Helpbar, Sidebar, Content },

        data() {
            return {
                binds: {},

                bgShown: false,
                bgUrl: ''
            }
        },

        methods: {
            setBackgroundProps(blur, brightness) {
                const str = `blur(${blur}px) brightness(${brightness}%)`
                console.log(str)
                $(this.$refs.background).css({
                    'filter'         : str,
                    '-webkit-filter' : str,
                    '-moz-filter'    : str,
                    '-o-filter'      : str,
                    '-ms-filter'     : str
                })
            },

            bind(keyCode, func) {
                this.binds[keyCode] = func
            },

            unbind(keyCode) {
                this.binds[keyCode] = undefined
            },

            updateBackgroundProps(state) {
                if (state.settings.backgroundEnabled) {
                    this.bgShown = true
                    this.bgUrl = `url("${state.settings.backgroundUrl}")`
                    this.setBackgroundProps(state.settings.backgroundBlur, state.settings.backgroundBrightness)
                } else {
                    this.bgShown = false
                }
            }
        },

        mounted() {
            Vue.prototype.$keyBind   = this.bind
            Vue.prototype.$keyUnBind = this.unbind

          

            let _ = this.$store.subscribe((mutation, state) => {
                if (mutation.type === "updateSettings") {
                    this.updateBackgroundProps(state)
                }
            })

            this.$setPreviewSize(this.$store.state.settings.previewSize || 64)

            document.onkeydown = event => {
                const bind = this.binds[event.keyCode]
                return bind ? bind(event) : true
            }

            this.$notify({
                "group": "main",
                "type":  "success",
                "title": this.$t("webClientInfo.welcome.title"),
                "text":  this.$t("webClientInfo.welcome.text")
            })

            this.bgShown = this.$store.state.settings.backgroundEnabled
            this.$nextTick(() => {
                if (this.bgShown) {
                    this.bgUrl = `url("${this.$store.state.settings.backgroundUrl}")`
                    this.setBackgroundProps(this.$store.state.settings.backgroundBlur, this.$store.state.settings.backgroundBrightness)
                }
            }, 100)
        }
    }
</script>
