<template>
	<div id="video-box" v-if="isPlaying">
    	<div class="bg" @click="stop"></div>
    	<div class="window" id="video-window">
    		<!-- <div class="header">
    			<div class="controls">
    				<button class="btn btn-4 btn-a" @click="setWindowed" v-if="!windowed"><icon icon="window-maximize" /></button>
    				<button class="btn btn-3 btn-a" @click="setFullscreen" v-else><icon icon="compress" /></button>
    			</div>
    			<div class="title" @dragStart="drag">{{ title }}</div>
    			<div class="controls">
    				<button class="btn btn-1 btn-a" @click="stop"><icon icon="times" /></button>
    			</div>
    		</div> -->
    		<iframe v-if="type === 'youtube'" :width="windowWidth * 0.7" :height="windowHeight * 0.7" :src="`https://www.youtube.com/embed/${video}`" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
    		<video v-else :src="video" controls autoplay loop></video>
    	</div>
    </div>
</template>

<script>
	import $ from 'jquery'

	export default {
		data () {
			return {
				type: null,
				title: "",
				isPlaying: false,
				video: "",
				onClear: null,
				windowed: false
			}
		},

		methods: {
			setWindowed () {
				this.windowed = true
			},

			setFullscreen () {
				this.windowed = false
			},

			stop () {
				this.isPlaying = false
				if (this.onClear) {
					this.onClear()
				}
			},

			play ({title, type, url}) {
				this.title = title
				this.video = url
				this.type = type
				this.isPlaying = true
			},

			setOnClear (func) {
				this.onClear = func
			}
		},

		mounted () {
			this.$root.$refs.videoPlayer = this
		}
	}
</script>