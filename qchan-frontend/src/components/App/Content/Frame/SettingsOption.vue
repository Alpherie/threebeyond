<template>
	<div class="checkbox">
        <input :id='label' type='checkbox' v-model="val" v-on:change="onClick" />
            <label :for='label'>
            <span></span>
            {{ label }}
            <ins><i>{{ label }}</i></ins>
        </label>
    </div>
</template>

<script>
	import Vue from 'vue'

	export default {
		props: [ "instance", "label" ],

		data() {
			return {
				val: Boolean
			}
		},

		methods: {
			onClick () {
				// lol, vue
				setTimeout(() => {
					this.$store.commit('updateSettings', { key: this.instance, value: this.val})
				}, 50)
			}
		},

		mounted() {
			this.val = this.$store.state.settings[this.instance]
		}
	}
</script>