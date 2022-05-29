<template>
    <input v-model="val" class="text-input" type="text" @change="onchange" :placeholder="placeholder">
</template>

<script>
	import Vue from 'vue'

	export default {
		props: [ "instance", "placeholder" ],

		data() {
			return {
				val: String,

				T: null
			}
		},

		methods: {
			onchange () {
				this.$store.commit('updateSettings', { key: this.instance, value: this.val})
			}
		},

		mounted() {
			this.val = this.$store.state.settings[this.instance]
		}
	}
</script>