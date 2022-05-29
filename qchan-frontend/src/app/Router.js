import Router from 'vue-router'

import SettingsRoute from '../components/App/Content/Frame/SettingsRoute.vue'
import BoardRoute    from '../components/App/Content/Frame/BoardRoute.vue'
import ThreadRoute   from '../components/App/Content/Frame/ThreadRoute.vue'
import LangsList     from '@/views/LangsList.vue'
import BoardsList    from '@/views/BoardsList.vue'
import HistoryRoute  from '@/views/History.vue'
import BoardReportsRoute  from '@/views/BoardReportsRoute.vue'
import HomeRoute     from '../components/App/Content/Frame/HomeRoute.vue'

let routes = [
    {
        path: '/',
        name: 'home',
        component: HomeRoute,
    },

    {
        path: '/settings',
        name: 'settings',
        component: SettingsRoute,
    },

    {
        path: '/langs',
        name: 'langs',
        component: LangsList,
    },

    {
        path: '/history',
        name: 'history',
        component: HistoryRoute,
    },

    {
        path: '/:lang([a-zA-Z]+)',
        name: 'boards',
        component: BoardsList,
    },

    {
        path: '/:lang([a-zA-Z]+)/:dir([a-z0-9]+)',
        name: 'board',
        component: BoardRoute,
    },

    {
        path: '/:lang([a-zA-Z]+)/:dir([a-z0-9]+)/reports',
        name: 'board-reports',
        component: BoardReportsRoute,
    },

    {
        path: '/:lang([a-zA-Z]+)/:dir([a-z0-9]+)/:num([0-9]+)',
        name: 'thread',
        component: ThreadRoute,
    }
]

const stateCache = {
    historyMax: 20,
    states: {},
    stateSavingPrevent: {},
    makeId: route => JSON.stringify({
        name:   route.name,
        params: route.params,
        query:  route.query,
    }),
    set(route, state) {
        state._date = new Date();
        const id = this.makeId(route);
        this.states[id] = state;

        // push out old states
        const history = [];
        for (let [ id, state ] of Object.entries(this.states)) {
            history.push(state._date);
        }
        if (history.length > this.historyMax) {
            history.sort().reverse();
            const maxDate = history[ this.historyMax - 1 ];
            for (let [ id, state ] of Object.entries(this.states)) {
                if (state._date < maxDate) {
                    //console.log('cleanup state' + id);
                    delete this.states[id];
                }
            }
        }
    },
    get(route) {
        const id = this.makeId(route);
        return this.states[id] || null;
    },
    drop(route) {
        const id = this.makeId(route);
        //console.log('drop state ' + id);
        delete this.states[id];
    },
};

window.dumpStateCache = () => {
    console.log(stateCache.states)
}

const router = new Router({
    mode: 'history',
    routes: routes,
    base: process.env.BASE_URL,
    history: true
});

router.stateCache = stateCache;

router.saveStateCache = (vm) => {
    const key = stateCache.makeId(vm.$route)

    if (stateCache.stateSavingPrevent[key] === true)
        delete stateCache.stateSavingPrevent[key]
    else
        stateCache.set(vm.$route, {
            data: Object.assign({}, vm.$data),
            scroll: {
                x: window.pageXOffset,
                y: window.pageYOffset
            }
        })
};

router.currentId = (vm) => {
    return stateCache.makeId(vm.$route)
}

router.destroyStateById = (vm, id) => {
    delete stateCache.states[id]
    stateCache.stateSavingPrevent[id] = true

    if (stateCache.makeId(vm.$route) == id) {
        let maxDate = null
        let newKey  = null

        for (const key in stateCache.states) {
            const state = stateCache.states[key]

            if (maxDate == null || state._date > maxDate) {
                maxDate = state._date
                newKey = key
            }
        }

        if (newKey)
            vm.$router.push(JSON.parse(newKey))
        else
            vm.$router.push({ name: 'home' })
    }
}

router.restoreStateCache = (vm) => {
    const state = stateCache.get(vm.$route);

    if (!state) {
        return false;
    }
    Object.assign(vm, state.data)
    stateCache.drop(vm.$route);
    return true;
};

router.sameRoute = (a, b) => {
    return (
        JSON.stringify([a.name, a.params || {}, a.query || {}])
        ===
        JSON.stringify([b.name, b.params || {}, b.query || {}])
    );
};

export default router;
