/* libraries */
import Config        from './app/Config'
import moment        from 'moment'
import Vue           from 'vue'
import VueWindowSize from 'vue-window-size';
import VueRouter     from 'vue-router'
import Notifications from 'vue-notification'
import VueI18n       from 'vue-i18n'
import Storage       from 'vue-web-storage'
import themes        from './app/themes'
import VueDraggableResizable from 'vue-draggable-resizable'
import Viewer from 'v-viewer'
import persistentState from 'vue-persistent-state'
import VueVirtualScroller from 'vue-virtual-scroller'
import store from '@/app/Store'
import vClickOutside from 'v-click-outside'
import vSelect from "vue-select"
import CountryFlag from 'vue-country-flag'
import VueObserveVisibility from 'vue-observe-visibility'
import "./assets/fonts/Futura Book.ttf"

/* vue */
import router       from './app/Router'
import messages     from './app/Messages'

/* components */
import App          from './components/App/index'

/* styles */
import './assets/styles/global.scss'
import './assets/styles/checkboxes.scss'
import './assets/styles/radiobuttons.scss'
import 'vue-draggable-resizable/dist/VueDraggableResizable.css'
import 'viewerjs/dist/viewer.css'

// = error catcher
import Raven from 'raven-js';
import RavenVue from 'raven-js/plugins/vue';

let initialState = {
    persisted: {
        'options.post.renderAllThumbnails': false,
        'options.post.preload': false
    }
}

Vue.use(persistentState, initialState)

if (Vue.$localStorage == undefined)
    Vue.use(Storage)

window.$ = window.jQuery = require('jquery')

Vue.component('country-flag', CountryFlag)
Vue.use(VueWindowSize)
Vue.use(vClickOutside)
Vue.use(Viewer)
Vue.use(VueI18n)
Vue.use(VueVirtualScroller)
Vue.use(VueRouter)
Vue.use(VueObserveVisibility)
Vue.use(Notifications)
Vue.component("v-select", vSelect)
Vue.component('vue-draggable-resizable', VueDraggableResizable)

if (process.env.NODE_ENV === "production" && process.env.SENTRY_FRONTEND) {
    Raven
        .config(process.env.SENTRY_FRONTEND)
        .addPlugin(RavenVue, Vue)
        .install();
}

String.prototype.trunc = String.prototype.trunc ||
      function(n){
          return (this.length > n) ? this.substr(0, n-1) + '...' : this;
      };

String.prototype.noTags = function() {
    return this.replace(/[\r\n]+/gm," ").replace(/<[^>]*>?/gm, '')
}

Vue.config.productionTip = false

Vue.prototype.$setTheme = ({ type, name, colors, save }) => {
    const html = document.getElementsByTagName('html')[0]

    for (const index in colors) {
        html.style.setProperty(`--color${index}`, colors[index])
    }

    html.style.setProperty('--background', colors[0])
    html.style.setProperty('--foreground', colors[7])

    if (save) {
        Vue.$localStorage.set('theme', { type, name, colors })
    }
}

Vue.prototype.$setPreviewSize = (size) => {
    const html = document.getElementsByTagName('html')[0]
    html.style.setProperty('--previewSize', `${size}px`)
}

Vue.prototype.$setFont = args => {
    const html = document.getElementsByTagName('html')[0]

    html.style.setProperty('--font', args.name)

    if (args.save) {
        Vue.$localStorage.set('font', args.name)
    }
}

const mobile = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent)

Vue.prototype.$formatDate = (timestamp) => {
    const date = moment.utc(timestamp * 1000)
    date.local()
    return date.format("DD.MM HH:mm:ss")
}

Vue.prototype.$getSelectionText = () => {
    var text = "";
    
    if (window.getSelection)
        text = window.getSelection().toString()
    else if (document.selection && document.selection.type != "Control")
        text = document.selection.createRange().text

    return text;
}

Vue.prototype.$copyText = (text) => {
    var input = document.createElement("input")
    input.value = text
    input.select()
    input.setSelectionRange(0, 99999)
    document.execCommand("copy")
}

Vue.prototype.$mobile = () => {return mobile}

Vue.prototype.$host = () => Config.root

Vue.prototype.$bytesToSize = (bytes) => {
    var sizes = ['B', 'KiB', 'MiB', 'GiB', 'TiB'];
    if (bytes == 0) return '0 Byte';
    var i = parseInt(Math.floor(Math.log(bytes) / Math.log(1024)));
    return Math.round(bytes / Math.pow(1024, i), 2) + ' ' + sizes[i];
}

// icons
import { faStar, faAngleDoubleDown, faInfinity, faExternalLinkAlt, faBackward, faForward, faList, faPlus, faQuestionCircle, faInfoCircle, faCodeBranch, faArrowLeft, faArrowRight, faArrowDown, faArrowUp, faChevronLeft, faSearch, faPlay, faThumbtack, faLockOpen, faLock, faGavel, faBookmark, faInfo, faTrash, faExclamationCircle, faHistory, faEye, faUserSlash, faEyeSlash, faUser, faBurn, faFire, faBars, faTags, faHome, faCog, faHeart, faReply, faChevronUp, faChevronDown, faWindowMaximize, faCompress, faTimes, faSync, faSyncAlt } from '@fortawesome/free-solid-svg-icons'
import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
library.add( faStar, faAngleDoubleDown, faInfinity, faBackward, faExternalLinkAlt, faForward, faList, faPlus, faQuestionCircle, faInfoCircle, faCodeBranch, faArrowLeft, faArrowRight, faArrowDown, faArrowUp, faChevronLeft, faSearch, faPlay, faThumbtack, faLockOpen, faLock, faGavel, faBookmark, faInfo, faTrash, faExclamationCircle, faHistory, faEye, faUserSlash, faEyeSlash, faUser, faBurn, faFire, faBars, faTags, faHome, faCog, faHeart, faReply, faChevronUp, faChevronDown, faWindowMaximize, faCompress, faTimes, faSync, faSyncAlt)
// import { faBookmark as farBookmark } from "@fortawesome/free-regular-svg-icons"
// library.add( farBookmark )

Vue.component('icon', FontAwesomeIcon)

const i18n = new VueI18n({
    locale: store.state.language, // set locale
    messages,     // set locale messages
})

new Vue({
    router,
    i18n,
    store,

    mounted() {
        let theme = Vue.$localStorage.get('theme')
        if (theme) {
            if (theme.type === undefined) {
                theme = { type: 'dark', name: 'dracula', colors: themes.dark['dracula'] }
            }
        } else {
            theme = { type: 'dark', name: 'dracula', colors: themes.dark['dracula'] }
        }
        this.$setTheme(theme)
        this.$setFont({ name: Vue.$localStorage.get('font') || 'Roboto' })
    },

    render: h => h(App)
}).$mount('#application-flow')
