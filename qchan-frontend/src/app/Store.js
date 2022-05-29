import Vue from "vue"
import Vuex from "vuex"
import createPersistedState from "vuex-persistedstate"

Vue.use(Vuex)

export default new Vuex.Store({
	state: {
		visitHistory: [],
		reasons: [],
		admin: {
			approved: false,
			roles: [],
			perms: [],
			token: "",
		},
		favouriteThreads: {langs: {}},
		rulesAgreed: false,
		threadSecrets: {},
		language: '',

		settings: {
			fixedNavbar: true,
			postMultiselection: false,
			activeRouteBar: false,
      captchaKind: "dcaptcha",
			specialTagsLimit: 20,
			imgurPreloadLimit: 20,
			youtubePreloadLimit: 20,
			repliesLimit: 20,
			
			previewSize: 50,

			bakgroundEnabled: true,
			bakgroundUrl: '',
			bakgroundBlur: 0.3,
			bakgroundBrightness: 20,
		},

		myPosts: {},
		cache: {
			langs: {},
			langsList: []
		}
	},

	mutations: {
		changeLanguage (state, code) {
			state.language = code
		},

		registerThreadSecret (state, obj) {
			state.threadSecrets[obj.index] = obj.value
		},

		cacheBoard (state, obj) {
			if (state.cache.langs[obj.lang] === undefined) {
				state.cache.langs[obj.lang] = { boards: {} }
			}
			if (state.cache.langs[obj.lang].boardsFull === undefined) {
				state.cache.langs[obj.lang].boardsFull = {}
			}

			if (typeof state.cache.langs[obj.lang].boards === 'Array') {
				state.cache.langs[obj.lang].boardsFull = {}
			}

			state.cache.langs[obj.lang].boardsFull[obj.short] = obj
			state.cache.langs[obj.lang].boardsFull[obj.short].lastUpdated = new Date().getTime() / 1000
		},

		markMyPost (state, obj) {
			state.myPosts[`${obj.lang}/${obj.dir}/${obj.num}`] = true
		},

		unmarkMyPost (state, obj) {
			delete state.myPosts[`${obj.lang}/${obj.dir}/${obj.num}`]
		},

		cacheLangs (state, obj) {
			state.cache.langsList.list = obj
			state.cache.langsList.lastUpdated = new Date().getTime() / 1000
		},

		cacheLang (state, obj) {
			if (state.cache.langs[obj.code] === undefined) {
				state.cache.langs[obj.code] = { boards: {}, cached: false }
			}
			state.cache.langs[obj.code].boards = obj.boards
			state.cache.langs[obj.code].cached = true
		},

		changeCaptcha (state, val) {
			state.settings.captchaKind = val
		},

		agreeRules (state) {
			state.rulesAgreed = true
		},

		disagreeRules (state) {
			state.rulesAgreed = false
		},

		expandFavouriteBoard (state, obj) {
			state.favouriteThreads.langs[obj.lang].boards[obj.dir].expanded = !state.favouriteThreads.langs[obj.lang].boards[obj.dir].expanded
		},

		updateFavouriteNewCount (state, obj) {
			if (state.favouriteThreads.langs[obj.lang] === undefined) {
				state.favouriteThreads.langs[obj.lang] = {boards: {}}
			}

			if (state.favouriteThreads.langs[obj.lang].boards[obj.dir] === undefined) {
				state.favouriteThreads.langs[obj.lang].boards[obj.dir] = {expanded: true, threads: {}}
			}
			if (state.favouriteThreads.langs[obj.lang].boards[obj.dir].threads[obj.num] !== undefined)
				state.favouriteThreads.langs[obj.lang].boards[obj.dir].threads[obj.num].newCount = obj.count
		},

		setFavouriteThread (state, obj) {
			if (state.favouriteThreads.langs[obj.lang] == undefined) {
				state.favouriteThreads.langs[obj.lang] = {boards: {}}
			}

			if (state.favouriteThreads.langs[obj.lang].boards[obj.dir] == undefined) {
				state.favouriteThreads.langs[obj.lang].boards[obj.dir] = {expanded: true, threads: {}}
			}

			state.favouriteThreads.langs[obj.lang].boards[obj.dir].threads[obj.num] = { title: obj.title, count: obj.count, newCount: obj.count }
		},

		updateFavouriteThread (state, obj) {
			if (state.favouriteThreads.langs[obj.lang] == undefined) {
				return
			}

			if (state.favouriteThreads.langs[obj.lang].boards[obj.dir] == undefined) {
				return
			}

			if (state.favouriteThreads.langs[obj.lang].boards[obj.dir].threads[obj.num]) {
				state.favouriteThreads.langs[obj.lang].boards[obj.dir].threads[obj.num] = { title: obj.title, count: obj.count, newCount: obj.count }
			}
		},

		unsetFavouriteThread (state, obj) {
			if (state.favouriteThreads.langs[obj.lang] == undefined) {
				state.favouriteThreads.langs[obj.lang] = {boards: {}}
			}

			if (state.favouriteThreads.langs[obj.lang].boards[obj.dir] == undefined) {
				state.favouriteThreads.langs[obj.lang].boards[obj.dir] = {expanded: true, threads: {}}
			}

			delete state.favouriteThreads.langs[obj.lang].boards[obj.dir].threads[obj.num]
		},

		setReasons (state, obj) {
			state.reasons = obj
		},

		updateSettings (state, obj) {
			state.settings[obj.key] = obj.value
		},

		authorize (state, obj) {
			state.admin = {
				approved: true,
				...obj
			}
		},

		deauthorize (state, obj) {
			state.admin = {
				approved: false,
				roles: [],
				perms: [],
				token: ""
			}
		},

		visitThread (state, obj) {
			if (obj.at === undefined)
				obj.at = Math.floor(Date.now() / 1000)

			let newA = [];

			for (let wobj of state.visitHistory) {
				if (!(wobj.lang == obj.lang && wobj.dir == obj.dir && wobj.num == obj.num)) 
					newA.push(wobj)
			}

			newA.push(obj)
			state.visitHistory = newA
		}
	},

	plugins: [createPersistedState()],
})
