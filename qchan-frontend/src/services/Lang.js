import Api from './Api'
import store from '@/app/Store'

export default {
    async getAll() {
        if (
    		store.state.cache.langsList.list === undefined ||
    		store.state.cache.langsList.list.length == 0 ||
    		store.state.cache.langsList.lastUpdated + 60 * 60 < new Date().getTime() / 1000
    	) {
    		let langs = await Api.get(`langs`)
	    	store.commit('cacheLangs', langs)
    		return langs
    	} else {
    		return store.state.cache.langsList.list
    	}
    },
}