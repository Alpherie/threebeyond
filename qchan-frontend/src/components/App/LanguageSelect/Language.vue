<template>
	<div class="language" @click="emitClick">
		<country-flag :country='country' size='big'/>
		<div class="title">{{ title }}</div>
	</div>
</template>

<script>
	export default {
		props: [ 'title', 'country' ],

		methods: {
			emitClick (data) {
				this.$emit('click', data)
			}
		}
	}
</script>