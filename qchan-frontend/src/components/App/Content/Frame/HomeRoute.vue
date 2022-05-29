<template>
    <div id="home">
        <div class="header">{{ $t(`home.header`) }}</div>
        <div class="sub-header">{{ $t(`home.subheader`) }}</div>
        <button class="btn btn-3 rules" @click="showRules">  {{ $t(`home.rules`) }}  </button>
        <button class="btn btn-4 boards" @click="goBoards">  {{ $t(`home.boards`) }}  </button>
        <div class="infos" v-if="infos">
            <div class="section" v-for="section in labeledInfoSections">
                <div class="title">{{ $t(`home.sections.${section.key}.name`) }}</div>
                <div class="content">
                    <div class="info" v-for="info in section.content">
                        <div class="key">{{ $t(`home.sections.${section.key}.content.${info.key}`) }}</div>
                        <div class="value">{{ info.value }}</div>
                    </div>
                </div>
            </div>
        </div>
        <!-- <div class="thanks" v-if="infos">
            <div>{{ $t("home.thanksLabel") }}</div>
            <button class="btn btn-1" @click="guideShown = !guideShown">  {{ $t("home.thanksDonateBtn") }}  </button>
            <div class="guide" v-if="guideShown" v-html="$t('home.thanksGuide')"></div>
            <div class="list" v-if="infos.donations.length">
                <div v-for="donation in infos.donations">
                    <b>{{ donation.name }}</b> - {{ donation.amount }} {{ donation.currency }}
                </div>
            </div>
            <div v-else class="list">
            </div>
        </div> -->
    </div>
</template>

<script>
    import HomeService from '@/services/Home';

    export default {
        computed: {
            labeledInfoSections () {
                return [
                    {
                        key: 'general',
                        content: [
                            // { key: 'total.domains', value: this.infos.total_domains },
                            { key: 'total.boards', value: this.infos.total_boards },
                            // { key: 'total.mods', value: this.infos.total_mods },
                            // { key: 'total.donation', value: `$ ${ this.infos.total_donated }` },
                        ]
                    },

                    {
                        key: 'posts',
                        content: [
                            { key: 'total', value: this.infos.total_posts },
                            // { key: 'last.hour', value: this.infos.total_posts_last_hour },
                            // { key: 'last.day', value: this.infos.total_posts_last_day },
                            // { key: 'last.week', value: this.infos.total_posts_last_week },
                            // { key: 'last.month', value: this.infos.total_posts_last_month },
                            // { key: 'last.year', value: this.infos.total_posts_last_year },
                        ]
                    },

                    {
                        key: 'media',
                        content: [
                            { key: 'total.files', value: this.infos.total_media_files },
                            { key: 'total.size', value: this.$bytesToSize(this.infos.total_media_size) },
                        ]
                    }
                ]
            }
        },

        data () {
            return {
                guideShown: false,
                infos: null
            }
        },

        methods: {
            goBoards () {
                this.$router.push({ name: "boards", params: { lang: this.$i18n.locale } })
            },

            showRules () {
                this.$store.commit('disagreeRules')
            }
        },

        async mounted () {
            this.infos = await HomeService.getStats();
        }
    }
</script>