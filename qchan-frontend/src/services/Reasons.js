import Api from './Api'

export default {
    get(token, params) {
        return Api.get(`reasons`, { headers: { Authorization: token } })
    },
}