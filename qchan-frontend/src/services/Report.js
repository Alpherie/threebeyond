import Api from './Api'

export default {
    create(params) {
        let headers = {'Content-Type': 'application/json'}

        if (typeof params.authorization == "string") {
            headers['Authorization'] = params.authorization;
        }

        return Api.post(`langs/${ params.lang }/boards/${ params.dir }/posts/[${ params.postNums }]/report`, JSON.stringify({ "comment": params.comment }), {
            headers
        })
    },

    solve(token, params) {
        return Api.post(`langs/${ params.lang }/boards/${ params.dir }/reports/${ params.id }/solve`, "", {
            headers: {'Authorization': token, 'Content-Type': 'application/json'}
        })
    },

    get(token, params) {
        return Api.get(`langs/${ params.lang }/boards/${ params.dir }/reports/pages/${ params.page }${ params.unsolvedOnly ? '?unsolved=true' : '' }`, { headers: { Authorization: token } })
    },
}