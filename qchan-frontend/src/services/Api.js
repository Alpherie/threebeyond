import Config from '@/app/Config'
import {Axios} from 'axios'
import bignumJSON from 'json-bignum'

// let API_URL;
// if (process.env.NODE_ENV == "production") {
//     API_URL = document.location.protocol + '//' + document.location.hostname + '/api/';
// } else {
//     API_URL = '//127.0.0.1:7777/api/';
// }

const API_URL = `${Config.root}/api/`

const axios = new Axios({
    baseURL: API_URL
});

axios.interceptors.response.use(
    response => {
        if (response.headers['content-type'] === 'application/json') {
            if (response.data) {
                const j = bignumJSON.parse(response.data)

                if (j.response)
                    return j.response
                else
                    throw j.error
            } else if (response.status == 204) {
                return {}
            } else {
                throw response
            }
        } else {
            return response
        }
    },

    error => {
        console.error(error)
        Promise.reject({ ...error })
    }
)

export default axios;