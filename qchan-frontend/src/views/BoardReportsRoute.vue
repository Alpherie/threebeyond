<template>
    <div id="boardReports">
        <h1 style="padding-left: 15px;">{{ $route.params.lang.toUpperCase() }}/{{ $route.params.dir }} - Reports</h1>

        <center>
            <button class="next-page btn btn-third" @click="goBack">  {{ $t('labels.goBackToBoard') }}  </button>
            <button class="next-page btn btn-first" v-if="unsolvedOnly" @click="showAll">  {{ $t('labels.reportsShowAll') }}  </button>
            <button class="next-page btn btn-fourth" v-else @click="showUnsovled">  {{ $t('labels.reportsShowUnsolved') }}  </button>
            <button class="next-page btn btn-second" @click="refresh">  {{ $t('labels.refreshPages') }}  </button>
        </center>

        <div class="list">
            <ReportView v-for="(report, index) in reports" v-bind:key="index" v-bind:obj="report" />
        </div>

        <center>
            <button class="next-page btn btn-fourth" v-if="reports.length" @click="getNextPage">  {{ $t('labels.nextPage') }}  </button>
        </center>
    </div>
</template>

<script>
    import Lang   from '@/services/Lang'
    import Board  from '@/services/Board'
    import Report from '@/services/Report'
    import ReportView from '@/components/ReportView'
    import $      from 'jquery'
    
    export default {
        components: { ReportView },
        data() {
            return {
                reports: [],
                page: 0,
                unsolvedOnly: false,
                scroll: null,
                title: null,
                dir: null
            }
        },

        methods: {
            goBack () {
                this.$router.push({ name: "board", params: { lang: this.$route.params.lang, dir: this.$route.params.dir } })
            },

            showAll () {
                this.unsolvedOnly = false
                this.refresh()
            },

            showUnsovled () {
                this.unsolvedOnly = true
                this.refresh()
            },

            async refresh () {
                this.$block()
                this.page = 0
                this.reports = []
                await this.getNextPage()
                this.$unblock()
            },

            async getNextPage () {
                try {
                    let page = await Report.get(this.$store.state.admin.token, { lang: this.$route.params.lang, dir: this.$route.params.dir, page: this.page++, unsolvedOnly: this.unsolvedOnly })
                    for (let report of page) {
                        this.reports.push(report)
                    }
                } catch(e) {
                    console.log(e)
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.reporting.getPageFailed"),
                        "text":  this.$t(`apiErrorCodes.${e.code}`)
                    })
                }
            }
        },

        async mounted() {
            if (!this.$router.restoreStateCache(this)) {
                this.$block()
                await this.getNextPage()
                this.$unblock()
            }
            this.type = this.$route.name
            this.dir = `# /${this.$route.params.lang}/${this.$route.params.dir}`
            this.title = `Reports`
            this.$updateRoutes(this.$router.currentId(this), this)
        },

        beforeRouteUpdate(to, from, next) {
            if (!this.$router.sameRoute(from, to)) {
                this.$router.saveStateCache(this);
            }

            next();
        },

        beforeRouteLeave(to, from, next) {
            if (!this.$router.sameRoute(from, to)) {
                this.$router.saveStateCache(this);
            }

            next();
        },
    }
</script>