import Api from '@/services/Api'
import Config from '@/app/Config'

export default {
    fRequest(params) {
        return fetch(`${Config.root}/api/captcha/dcaptcha.png`)
    },

    request(params) {
        return Api.get('captcha/dcaptcha.png')
    },
}