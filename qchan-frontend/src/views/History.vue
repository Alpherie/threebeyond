<template>
	<div id="history">
		<h1>History</h1>

		<div class="visited-threads">
			<div v-for="(thread, idx) in $store.state.visitHistory.slice().reverse()" @click="goThread(thread)" class="visited-thread">
				<div class="header">
					<span class="path">
						<button class="btn btn-third lang">{{ thread.lang }}</button>
						 / <button class="btn btn-third ref">{{ thread.dir }}</button>
						 / <button class="btn btn-fourth num">{{ thread.num }}</button>
					</span>
					<span class="at">{{ $formatDate(thread.at) }}</span>
				</div>
				<div class="content">
					{{ thread.subject }}
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	export default {
		mounted () {
			// this.$store.commit("visitThread", { lang: 'en', dir: 'b', num: '475', subject: 'Test' })
		},

		methods: {
			goThread (params) {
				this.$router.push({ name: 'thread', params: { lang: params.lang, dir: params.dir, num: `${params.num}` } })
			}
		}
	}
</script>