import Vue from 'vue'
import Storage from 'vue-web-storage'

if (Vue.$localStorage == undefined)
    Vue.use(Storage)

const hidden = {
    list: {},

    sync() {
        Vue.$localStorage.set('hidden', this.list)
    },

    hide(dir, num) {
        if (hidden.list[dir] == undefined) {
            hidden.list[dir] = {}
        }

        hidden.list[dir][num] = true
        this.sync()
    },

    show(dir, num) {
        if (hidden.list[dir] == undefined)
            return false

        hidden.list[dir][num] = undefined
        this.sync()
    },

    isHidden(dir, num) {
        if (hidden.list[dir] == undefined)
            return false

        return hidden.list[dir][num] || false
    }
}

hidden.list = Vue.$localStorage.get('hidden') || {}

export default hidden