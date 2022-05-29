<template>
	<div class="file" @click="play">
		<img :src="`https://img.youtube.com/vi/${video}/0.jpg`" />
        <div class="video-play-overlay youtube">
            <icon icon="play" />
        </div>
	</div>
</template>

<script>
	export default {
		props: ['video'],

		methods: {
			play () {
				this.$root.$refs.videoPlayer.play({
					url: this.video,
					title: `https://youtu.be/${ this.video }`,
					type: 'youtube'
				})
			},
		}
	}
</script>