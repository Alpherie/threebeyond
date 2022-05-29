import Api from './Api'

export default {
    getStats() {
        return Api.get(`stats`)
    },
}