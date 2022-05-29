<template>
	<div id="loader" v-if="focused">
		<div class="lds-dual-ring"></div>
	</div>
</template>

<script>
	import Vue from 'vue'

	export default {
		data() {
			return {
				focused: false
			}
		},

		mounted() {
			Vue.prototype.$block = () => { this.focused = true }
			Vue.prototype.$unblock = () => { this.focused = false }
		}
	}
</script>