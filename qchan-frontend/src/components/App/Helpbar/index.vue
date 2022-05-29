<template>
    <div id="helpbars">
        <div id="helpbar-right">
            <button class="btn btn-1" @click="scrollUp"><icon icon="chevron-up"/></button>        
            <button class="btn btn-1" v-if="fullscreenAvailable" @click="fullscreen"><icon icon="compress"/></button>
            <button class="btn btn-1" @click="scrollDown"><icon icon="chevron-down"/></button>
        </div>

        <!-- <div id="helpbar-bottom"> -->
        <!-- </div> -->
    </div>
</template>

<script>
    export default {
        computed: {
            fullscreenEnabled() {
                return document.fullscreenEnabled
            }
        },

        data() {
            return {
                fullscreenAvailable: false,
                fullscreenElement: null
            }
        },

        methods: {
            fullscreen() {
                let elem = document.documentElement
                let promise

                if (elem.requestFullscreen) {
                    promise = elem.requestFullscreen();
                } else if (elem.mozRequestFullScreen) { /* Firefox */
                    promise = elem.mozRequestFullScreen();
                } else if (elem.webkitRequestFullscreen) { /* Chrome, Safari and Opera */
                    promise = elem.webkitRequestFullscreen();
                } else if (elem.msRequestFullscreen) { /* IE/Edge */
                    promsie = elem.msRequestFullscreen();
                }
            },

            scrollDown() {
                const target = document.getElementById("content")
                target.scrollTop = target.scrollHeight 
            },

            scrollUp() {
                document.getElementById("content").scrollTop = 0
            }
        },

        mounted() {
            document.documentElement.addEventListener('fullscreenchange', event => {
                this.fullscreenAvailable = document.fullscreenElement == null

                // if (this.fullscreenAvailable) {
                //     this.$notify({
                //         "group": "main",
                //         "type":  "success",
                //         "title": this.$t("other.fullscreen.leaved.title"),
                //         "text":  this.$t("other.fullscreen.leaved.text")
                //     })
                // } else {
                //     this.$notify({
                //         "group": "main",
                //         "type":  "success",
                //         "title": this.$t("other.fullscreen.entered.title"),
                //         "text":  this.$t("other.fullscreen.entered.text")
                //     })
                // }
            })

            this.fullscreenAvailable = this.fullscreenEnabled
        }
    }
</script>