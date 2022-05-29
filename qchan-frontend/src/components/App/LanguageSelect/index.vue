<template>
	<div id="language-select" v-if="$store.state.language == ''">
		<div class="box">
			<b>Choose preferred language:</b>
			<div class="languages">
				<Language @click="selectLanguage(lang.code)" v-for="(lang, index) in list" v-bind:key="index" v-bind:country="lang.country" v-bind:title="lang.title" />
			</div>
			<div class="annotaion">
				You can always change the language in the <icon icon="cog"/> settings menu.
			</div>
		</div>
	</div>
</template>

<script>
	import Language from './Language'

	export default {
		components: { Language },

		methods: {
			selectLanguage(code) {
				this.$store.commit("changeLanguage", code)
				this.$i18n.locale = code
			}
		},

		computed: {
			list () {
				return [{ code: 'en', country: 'gb', title: 'English' }, { code: 'ru', country: 'ru', title: 'Русский' }]
			}
		}
	}
</script>