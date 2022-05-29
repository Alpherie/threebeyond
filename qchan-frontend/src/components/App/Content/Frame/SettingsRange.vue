<template>
	<div class="setting-section">
        <div class="title">{{ title }}</div>
        <div class="comment">{{ comment }}</div>
        <div class="actions">
            <div class="range-with-label">
                <span class="label">{{ Math.floor(val) }}</span>
                <div class="range-box">
                    <input type="range" v-model="val" :min="min ? min : 0" :max="max ? max : 100" v-on:change="onChange">
                </div>
            </div>
        </div>
    </div>
</template>

<script>
	import Vue from 'vue'

	export default {
		props: [ "instance", "title", "comment", "min", "max", "modifier" ],

		data() {
			return {
				val: Number
			}
		},

		methods: {
			onChange () {
				this.$nextTick(() => {
					const val = this.modifier ? this.val * this.modifier : this.val
					this.$store.commit('updateSettings', { key: this.instance, value: val })
					this.$emit('change', val)
				})
			}
		},

		mounted() {
			this.val = this.$store.state.settings[this.instance]
			if (this.val === undefined) {
				this.val = 0
			}
			if (this.modifier) {
				this.val *= 1/this.modifier
			}
		}
	}
</script>