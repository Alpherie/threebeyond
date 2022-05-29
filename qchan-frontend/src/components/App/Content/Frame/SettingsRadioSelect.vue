<template>
	<div style="height: 40px">
		<div class="radio" v-for="variant in variants" @click="onChange(variant.value)">
			<input type='radio' v-model="value" :value="variant.value" :name="variant.name" />
	        <label for='captcha-hcaptcha'>
	            <span></span>
	            {{ variant.name }}
	            <ins><i>{{ variant.name }}</i></ins>
	        </label>
        </div>
	</div>
</template>

<script>
	export default {
		props: ['variants', 'default'],

		data () {
			return {
				value: null
			}
		},

		mounted () {
			this.value = this.default
		},

		methods: {
			onChange (value) {
				this.value = value
				this.$emit('change', value)
			}
		}
	}
</script>