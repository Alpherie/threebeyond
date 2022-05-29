import Api from './Api'
import store from '@/app/Store'

export default {
    async getAllFresh(params) {
        let boards = await Api.get(`langs/${params.lang}/boards`)
        store.commit('cacheLang', { code: params.lang, boards })
        return boards
    },

    async getAll(params) {
        if (
            store.state.cache.langs[params.lang] === undefined ||
            store.state.cache.langs[params.lang].cached === undefined ||
            store.state.cache.langs[params.lang].lastUpdated + 60 * 60 < new Date().getTime() / 1000
        ) {
            return this.getAllFresh(params)
        } else {
            return store.state.cache.langs[params.lang].boards
        }
    },

    getPage(params) {
        return Api.get(`langs/${params.lang}/boards/${ params.dir }/pages/${ params.page }`)
    },

    loadCatalog(params) {
        return Api.get(`langs/${params.lang}/boards/${ params.dir }/threads`)
    },

    async getFresh(params) {
        let board = await Api.get(`langs/${params.lang}/boards/${params.dir}`)
        store.commit('cacheBoard', board)
        return board
    },

    async get(params) {
        if (typeof store.state.cache.langs[params.lang].boards == "array") {
            store.state.cache.langs[params.lang].boards = {}
        }

    	if (
            store.state.cache.langs[params.lang] === undefined ||
            store.state.cache.langs[params.lang].boardsFull === undefined ||
    		store.state.cache.langs[params.lang].boardsFull[params.dir] === undefined ||
    		store.state.cache.langs[params.lang].boardsFull[params.dir].lastUpdated + 300 < new Date().getTime() / 1000
    	) {
    		return this.getFresh(params)
    	} else {
    		return store.state.cache.langs[params.lang].boardsFull[params.dir]
    	}
    },
}