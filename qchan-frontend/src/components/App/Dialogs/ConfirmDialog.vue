<template>
	<div class="dialog-box">
		<div class="dialog-close" @click="cancel"></div>
		<div class="dialog dialog-confirm">
			<h3>{{ title }}</h3>
			<p>{{ message }}</p>

			<div class="buttons">
				<button class="btn btn-2 confirm" @click="confirm">Confirm</button>
				<button class="btn btn-1 cancel" @click="cancel">Cancel</button>
			</div>
		</div>
	</div>
</template>

<script>
	export default {
		props: ['title', 'message'],

		methods: {
			confirm() {
				this.$emit('confirm')
			},

			cancel() {
				this.$emit('cancel')
			}
		}
	}
</script>