<template>
	<div class="dialog-box">
		<div class="dialog-close" @click="cancel"></div>
		<div class="dialog dialog-ban">
			<h3>Ban poster</h3>
			<div class="addr-form">
				<!-- <input type="text" v-bind:placeholder="$t('placeholders.lang')" v-model="inputLang"> -->
				<!-- <input type="text" v-bind:placeholder="$t('placeholders.board')" v-model="inputBoard"> -->
				<!-- <input type="text" v-bind:placeholder="$t('placeholders.postNum')" v-model="inputPostNums"> -->
				<v-select v-model="inputBanReason" v-bind:placeholder="$t('placeholders.banReason')" :options="banReasons"></v-select>
				<input type="text" v-bind:placeholder="$t('placeholders.comment')" v-model="inputComment">
				<input type="text" v-bind:placeholder="$t('placeholders.minutes')" v-model="inputMinutes">
				<div class="options">
					<div class="checkbox" style="width: 200px">
				        <input id='issubnet' type='checkbox' v-model="subnet" />
				            <label for='issubnet'>
				            <span></span>
				            Subnet
				            <ins><i>Subnet</i></ins>
				        </label>
				    </div>
				</div>
			</div>
			<div class="buttons">
				<button class="btn btn-2 confirm" @click="confirm">Confirm</button>
				<button class="btn btn-1 cancel" @click="cancel">Cancel</button>
			</div>
		</div>
	</div>
</template>

<script>
	import Admin from "@/services/Admin"

	export default {
		props: [
			"lang",
			"board",
			"postNum"
		],

		computed: {
			banReasons () {
				let arr = []
				for (let reason of this.$store.state.reasons) {
					arr.push({ label: this.$t(`banReasons.${ reason }`), code: reason })
				}
				return arr
			}
		},

		data () {
			return {
				inputComment: null,
				inputLang: null,
				inputBoard: null,
				inputPostNum: null,
				inputMinutes: null,
				inputComment: null,
				inputBanReason: null,
				subnet: false
			}
		},

		mounted () {
			this.inputLang = this.lang
			this.inputBoard = this.board
			this.inputPostNum = this.postNum
		},

		methods: {
			async confirm () {
				this.$block()
                try {
                    await Admin.banPoster(this.$store.state.admin.token, { dir: this.board, lang: this.lang, num: this.postNum, minutes: parseInt(this.inputMinutes), reason: this.inputBanReason.code, subnet: this.subnet, comment: this.inputComment, scope: "board" })
                    this.$notify({
                        "group": "main",
                        "type":  "success",
                        "title": this.$t("api.admin.posterBan.success"),
                    })
                
                	this.$unblock()
					this.$emit('confirm')
                } catch(e) {
                	console.log(e)
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.admin.posterBan.failed"),
                        "text":  this.$t(`apiErrorCodes.${e.code}`)
                    })
                }
                this.$unblock()
			},

			cancel () {
				this.$emit('cancel')
			}
		}
	}
</script>