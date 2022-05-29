import Api from './Api'

export default {
    get(params) {
        return Api.get(`langs/${params.lang}/boards/${params.dir}/posts/${params.num}`)
    },

    create(params) {
    	let formdata = new FormData()
        
        for (const param in params) {
            if (param == "files") {
                let i = 0;

                for (const file of params[param])
                    formdata.append(`file${i++}`, file)

                continue
            }

            if (params[param] != null)
                formdata.append(param, params[param])
        }

        let headers = {'Content-Type': 'multipart/form-data'}

        if (typeof params.authorization == "string") {
            headers['Authorization'] = params.authorization;
        }

        if (typeof params.captchaKind === "string") {
            headers['X-Captcha-Kind'] = params.captchaKind
            headers[params.captchaKind === 'op' ? 'X-Thread-Secret' : 'X-Captcha-Value'] = params.captchaValue
        }

        return Api.post(`langs/${ params.lang }/boards/${ params.dir }/threads/${ params.thread }/posts`, formdata, {
            headers
        });
    }
}