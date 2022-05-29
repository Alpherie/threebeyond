<template>
	<div id="rules" v-if="!$store.state.rulesAgreed">
		<div class="centered">
			<div v-html="$t('rules')"></div>
			<button class="btn btn-a btn-2" @click="agree">  {{ $t("rulesAgree") }}  </button>
		</div>
	</div>
</template>

<script>
	export default {
		data () {
			return {
				start: null
			}
		},

		mounted () {
			this.start = new Date()
		},

		methods: {
			agree () {
				let end = new Date()
				let seconds = (end.getTime() - this.start.getTime()) / 1000

				// if (seconds < 20) {
				// 	this.start = new Date()
				// 	this.$notify({
    //                     "group": "main",
    //                     "type":  "error",
    //                     "title": this.$t("rulesTime"),
    //                 })
				// } else {
				this.$store.commit('agreeRules')
				// }
			}
		}
	}
</script>