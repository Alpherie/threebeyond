import Api from './Api'

export default {
    get(list) {
        return Api.get(`counting/[${list}]`)
    },
}