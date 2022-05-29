<template>
	<div class="dialog-box">
		<div class="dialog-close" @click="cancel"></div>
		<div class="dialog dialog-report">
			<h3>Report</h3>
			<input type="text" v-model="comment" placeholder="Comment">
			<div class="buttons">
				<button class="btn btn-2 confirm" @click="confirm">Confirm</button>
				<button class="btn btn-1 cancel" @click="cancel">Cancel</button>
			</div>
		</div>
	</div>
</template>

<script>
	import Report from "@/services/Report"

	export default {
		props: ['lang', 'board', 'postNums'],

		data () {
			return {
				comment: ""
			}
		},

		methods: {
			async confirm() {
				this.$block()
                try {
                    await Report.create({ dir: this.board, lang: this.lang, postNums: this.postNums, comment: this.comment, authorization: this.$store.state.admin.token })
                    this.$notify({
                        "group": "main",
                        "type":  "success",
                        "title": this.$t("api.reporting.success"),
                    })
                
                	this.$unblock()
					this.$emit('confirm')
                } catch(e) {
                	console.log(e)
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.reporting.failed"),
                        "text":  this.$t(`apiErrorCodes.${e.code}`)
                    })
                }
                this.$unblock()
				this.$emit('confirm')
			},

			cancel() {
				this.$emit('cancel')
			}
		}
	}
</script>