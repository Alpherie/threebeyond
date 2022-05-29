<template>
    <div class="entry" v-if="report">
        <div v-bind:class="{ head: true, solved: report.solved_by !== null }">
            <span class="id">{{ report.id }}</span>
            <span class="at">{{ $formatDate(report.at) }}</span>
            <span class="status" v-if="report.solved_by === null">{{ $t('labels.reportPending') }}</span>
            <span class="status SOLVED" v-else>{{ $t('labels.reportSolved') }}</span>
            <div class="related-posts">
                <button class="btn btn-4">>>{{ report.thread_num }}</button>
                <button v-for="(link, index) in report.post_nums" class="btn btn-second">>>{{ link }}</button>
            </div>
        </div>
        <div class="comment">
            {{ report.comment }}
        </div>
        <div class="options" v-if="report.solved_by === null">
            <button class="btn btn-4" @click="openThread(report.thread_num)">  {{ $t('labels.openThread') }}  </button>
            <button class="btn btn-1" @click="markSolved(report.id)">  {{ $t('labels.markSolved') }}  </button>
        </div>
    </div>
</template>

<script>
    import Report from '@/services/Report'

    export default {
        props: ["obj"],

        data () {
            return {
                report: null
            }
        },

        mounted () {
            this.report = this.obj
        },

        methods: {
            openThread () {
                this.$router.push({ name: "thread", params: { lang: this.$route.params.lang, dir: this.$route.params.dir, num: `${this.report.thread_num}` } })
            },
            async markSolved (id) {
                try {
                    await Report.solve(this.$store.state.admin.token, { lang: this.$route.params.lang, dir: this.$route.params.dir, id })
                    this.report.solved_by = 1
                } catch(e) {
                    console.log(e)
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("api.reporting.solveFailed"),
                        "text":  this.$t(`apiErrorCodes.${e.code}`)
                    })
                }
            },
        }
    }
</script>