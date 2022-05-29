import Api from './Api'

export default {
    get(params) {
        return Api.get(`langs/${ params.lang }/boards/${ params.dir }/threads/${ params.num }`)
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
            headers['X-Captcha-Value'] = params.captchaValue
        }

        return Api.post(`langs/${ params.lang }/boards/${ params.dir }/threads`, formdata, {
            headers
        });
    },

    pinPost(params) {
        return Api.post(`langs/${ params.lang }/boards/${ params.dir }/threads/${ params.thread }/posts/${ params.num }/pin`, '', {
            headers: {
                'X-Thread-Secret': params.secret,
                'Content-Type': 'application/json'
            }
        })
    },

    unpinPost(params) {
        return Api.post(`langs/${ params.lang }/boards/${ params.dir }/threads/${ params.thread }/posts/${ params.num }/unpin`, '', {
            headers: {
                'X-Thread-Secret': params.secret,
                'Content-Type': 'application/json'
            }
        })
    },

    // deletePostsMultiple({ lang, dir, thread, nums, secret }) {
    //     return Api.delete(`langs/${ lang }/boards/${ dir }/threads/${ thread }/posts/[${ nums.join(',') }]`, {
    //         headers: {
    //             'X-Thread-Secret': secret
    //         }
    //     })
    // },

    deletePost({ lang, dir, thread, num, secret }) {
        return Api.delete(`langs/${ lang }/boards/${ dir }/threads/${ thread }/posts/${num}`, {
            headers: {
                'X-Thread-Secret': secret
            }
        })
    },

    getPosts(params) {
        return Api.get(`langs/${ params.lang }/boards/${ params.dir }/threads/${ params.num }/pages/${ params.page || 0 }`, { params: { full: params.full, from: params.from} });
    }
}