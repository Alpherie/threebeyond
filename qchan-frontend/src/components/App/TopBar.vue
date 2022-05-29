<template>
	<div id="topbar" v-if="shown">
		<div class="title">
			{{ title }}
		</div>
		<div class="actions">
	        <button class="btn btn-1" v-for="button in buttons" @click="button.action">
	        	<icon :icon="button.icon"/>
	        </button>
		</div>
	</div>
</template>

<script>
	import Vue from 'vue'

	export default {
		data() {
			return {
				shown: false,
				title: String,
				buttons: Array
			}
		},

		methods: {
			show () {
				this.shown = true
			},

			hide () {
				this.shown = false
			},

			setTitle (title) {
				this.title = title
			},

			setButtons (buttons) {
				this.buttons = buttons
			}
		},

		mounted() {
			Vue.prototype.$setTopTitle = this.setTitle
			Vue.prototype.$setTopButtons = this.setButtons
			Vue.prototype.$hideTopBar = this.hide
			Vue.prototype.$showTopBar = this.show
		}
	}
</script>