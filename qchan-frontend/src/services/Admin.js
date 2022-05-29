import Api from './Api'

export default {
    get(token, params) {
        return Api.get(`auth`, { headers: { Authorization: token } })
    },

    createBoard(token, params) {
        return Api.post(`langs/${ params.lang }/boards`, JSON.stringify(params),  { headers: { 'Content-Type': 'application/json', Authorization: token } })
    },

    editBoard(token, params) {
        return Api.put(`langs/${ params.lang }/boards/${ params.short }`, JSON.stringify(params),  { headers: { 'Content-Type': 'application/json', Authorization: token } })
    },

    deletePost(token, params) {
        return Api.delete(`langs/${ params.lang }/boards/${ params.dir }/posts/${ params.num }`, { headers: { Authorization: token } })
    },

    openThread(token, params) {
        return Api.post(`langs/${ params.lang }/boards/${ params.dir }/threads/${ params.num }/open`, {}, { headers: { Authorization: token } })
    },

    closeThread(token, params) {
        return Api.post(`langs/${ params.lang }/boards/${ params.dir }/threads/${ params.num }/close`, {}, { headers: { Authorization: token } })
    },

    pinThread(token, params) {
        return Api.post(`langs/${ params.lang }/boards/${ params.dir }/threads/${ params.num }/pin`, {}, { headers: { Authorization: token } })
    },

    unpinThread(token, params) {
        return Api.post(`langs/${ params.lang }/boards/${ params.dir }/threads/${ params.num }/unpin`, {}, { headers: { Authorization: token } })
    },

    makeThreadEndless(token, params) {
        return Api.post(`langs/${ params.lang }/boards/${ params.dir }/threads/${ params.num }/endless`, {}, { headers: { Authorization: token } })
    },

    deletePostsMultiple(token, params) {
        return Api.delete(`langs/${ params.lang }/boards/${ params.dir }/posts/[${ params.nums.join(',') }]`, { headers: { Authorization: token } })
    },

    banPoster(token, params) {
        return Api.post(`langs/${ params.lang }/boards/${ params.dir }/posts/${ params.num }/ban`, JSON.stringify({
        	scope: params.scope,
        	minutes: params.minutes,
        	comment: params.comment,
        	reason: params.reason,
        	subnet: params.subnet
        }), { headers: { 'Content-Type': 'application/json', Authorization: token } })
    },

    getPostDetails(token, params) {
        return Api.get(`langs/${ params.lang }/boards/${ params.dir }/posts/${ params.num }/details`, { headers: { 'Content-Type': 'application/json', Authorization: token } })
    },

    solveReport(token, params) {
        return Api.post(`langs/${ params.lang }/boards/${ params.dir }/reports/${ params.id }/solve`, {}, { headers: { 'Content-Type': 'application/json', Authorization: token } })
    }
}