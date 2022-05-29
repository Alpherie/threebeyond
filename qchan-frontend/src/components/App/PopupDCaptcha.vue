<template>
	<div class="dcaptcha-container">
		<div v-if="src" class="dcaptcha-form">
			<h1>{{ $t('dcaptcha.title') }}</h1>
			<h3>{{ $t('dcaptcha.description') }}</h3>
			<div>
				<img width="200" :src="src">
			</div>
			<div class="lifebar">
				<div class="life" ref="life"></div>
			</div>
			<input type="text" focus v-model="input">
			<p class="warning">{{ $t('dcaptcha.warning') }}</p>
			<div class="buttons">
				<div class="btn btn-3" @click="load">  {{ $t('dcaptcha.requestAnother') }}  </div>
				<div class="btn btn-1" @click="cancel">  {{ $t('commonButtons.cancel') }}  </div>
			</div>
		</div>
		<div v-else class="dcaptcha-form">
			<h1>
				{{ $t('dcaptcha.generating') }}
			</h1>
			<div class="btn btn-1">  {{ $t('commonButtons.cancel') }}  </div>
		</div>
	</div>
</template>

<script>
	import DCaptcha from '@/services/DCaptcha'
	import sjcl from 'sjcl'

	export default {
		data () {
			return {
				src: null,
				prefix: null,
				hash: null,
				lifetime: null,
				life: null,
				input: null,
				inputPrev: null,
				interval: null
			}
		},

		methods: {
			cancel () {
				this.$emit('cancel')
			},

			revokeUrl () {
				if (this.src !== null) {
					window.URL.revokeObjectURL(this.src)
					this.src = null
				}
			},

			createUpdateInterval () {
				this.interval = setInterval(() => {
					if (this.prefix) {
						this.life -= 0.25

						if (this.input !== this.inputPrev) {
							// generate hash
							let hash = sjcl.codec.hex.fromBits(sjcl.hash.sha256.hash(this.prefix + this.input))
							// console.log(hash, this.hash)
							if (hash == this.hash) {
								this.$emit('accepted', this.input)
							}

							this.inputPrev = this.input
						}

						if (this.life == 0) {
							this.load()
						}
						let percent = this.life / this.lifetime * 100 

						if (this.$refs.life) {
							this.$refs.life.style.width = `${percent}%`
						}
					}
				}, 250)
			},

			clearUpdateInterval () {
				if (this.interval !== null) {
					clearInterval(this.interval)
				}
			},

			load () {
				this.revokeUrl()
				this.src = null
				let vm = this
				DCaptcha.fRequest().then((res) => {
					vm.prefix = res.headers.get('x-dcaptcha-prefix')
					vm.life = vm.lifetime = parseInt(res.headers.get('x-dcaptcha-lifetime'))
					vm.hash = res.headers.get('x-dcaptcha-hash')
					res.blob().then((blob) => {
						vm.src = window.URL.createObjectURL(blob)
					})
				}).catch((e) => {
					console.error(e)
	                this.$notify({
	                    "group": "main",
	                    "type":  "error",
	                    "title": this.$t("api.dcaptcha.loadFailed"),
	                    "text":  this.$t(`apiErrorCodes.${e.code}`)
	                })
	                vm.cancel()
				})
			}
		},

		beforeDestroy () {
			this.clearUpdateInterval()
			this.revokeUrl()
		},

		mounted () {
			this.load()
			this.createUpdateInterval()
		}
	}
</script>