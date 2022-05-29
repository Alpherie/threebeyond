<template>
    <div id="settings">
        <div class="section section-themes">
            <div class="title" @click="themesSectionEnabled = !themesSectionEnabled">{{ $t('titles.colorSchemes') }}</div>
            <div class="body" v-if="themesSectionEnabled">
                <div class="buttons-wrap">
                    <button class="btn btn-4" :class="{ 'btn-a': themesSectionMenu == 'dark' }" @click="themesSectionMenu = 'dark'">{{ $t('settings.colorSchemes.dark') }}</button>
                    <button class="btn btn-4" :class="{ 'btn-a': themesSectionMenu == 'light' }" @click="themesSectionMenu = 'light'">{{ $t('settings.colorSchemes.light') }}</button>
                    <button class="btn btn-4" :class="{ 'btn-a': themesSectionMenu == 'custom' }" @click="themesSectionMenu = 'custom'">{{ $t('settings.colorSchemes.custom') }}</button>
                </div>


                <div class="themes-list-default" v-if="themesSectionMenu == 'light'">
                    <button v-for="(colors, name) in themesLight" class="btn btn-1 theme" :class="{ 'btn-a': currentTheme.type === 'light' && currentTheme.name === name }" @click="setTheme({ type: 'light', name, colors, save: true })">
                        <div class="name">{{ name }}</div>
                        <div class="blocks">
                            <div class="block" v-for="color in colors" v-bind:style="{ background: color }"></div>
                        </div>
                    </button>
                </div>

                <div class="themes-list-default" v-if="themesSectionMenu == 'dark'">
                    <button v-for="(colors, name) in themesDark" class="btn btn-1 theme" :class="{ 'btn-a': currentTheme.type === 'dark' && currentTheme.name === name }" @click="setTheme({ type: 'dark', name, colors, save: true })">
                        <div class="name">{{ name }}</div>
                        <div class="blocks">
                            <div class="block" v-for="color in colors" v-bind:style="{ background: color }"></div>
                        </div>
                    </button>
                </div>

                <div class="themes-list-default" v-if="themesSectionMenu == 'custom'">
                    <button class="btn btn-4 btn-a" @click="applyCustomScheme">{{ $t('commonButtons.apply') }}</button>
                    <input class="text-input" v-model="customSchemeInput" style="display: inline" :placeholder="$t('settings.colorSchemes.customPlaceholder')" />
                </div>
            </div>
        </div>

        <!-- <div class="section section-fonts">
            <div class="title" @click="fontsSectionEnabled = !fontsSectionEnabled">{{ $t('titles.fonts') }}</div>
            <div class="body" v-if="fontsSectionEnabled">
                <div class="fonts-list">
                    <div class="font" v-for="name in fonts" @click="$setFont({ name })" v-bind:style="{ fontFamily: name }">
                        {{ name }}
                    </div>
                </div>
            </div>
        </div> -->

        <div class="section section-ui">
            <div class="title" @click="uiSectionEnabled = !uiSectionEnabled">{{ $t('titles.ui') }}</div>
            <div class="body" v-if="uiSectionEnabled">
                <div class="options">
                    <Option
                        instance="postMultiselection"
                        label="Multiselection" />
                    <Option
                        instance="activeRouteBar"
                        label="Routebar" />
                    <Range v-bind:title="$t('settings.previewSize')" instance="previewSize" @change="$setPreviewSize" />

                    <!-- <Option
                        instance="options.post.renderAllThumbnails"
                        label="Render all attachments" />

                    <Option
                        instance="options.post.preload"
                        label="Preload posts" /> -->
                </div>
            </div>
        </div>

        <div class="section section-background">
            <div class="title" @click="backgroundSectionEnabled = !backgroundSectionEnabled">{{ $t('titles.background') }}</div>
            <div class="body" v-if="backgroundSectionEnabled">
                <div class="options">
                    <Option
                        instance="backgroundEnabled"
                        v-bind:label="$t('settings.background.enable')" />

                    <div :style="{ 'pointer-events': $store.state.settings.backgroundEnabled ? 'auto' : 'none', opacity: $store.state.settings.backgroundEnabled ? '1.0' : '0.8' }">
                        <Input instance="backgroundUrl" :placeholder="$t('settings.background.url')" />
                        <Range v-bind:title="$t('settings.background.blur')" instance="backgroundBlur" modifier="0.1" />
                        <Range v-bind:title="$t('settings.background.brightness')" instance="backgroundBrightness" />
                    </div>
                </div>
            </div>
        </div>

        <div class="section section-safety">
            <div class="title" @click="safetySectionEnabled = !safetySectionEnabled">{{ $t('titles.safety') }}</div>
            <div class="body" v-if="safetySectionEnabled">
                <Range v-bind:title="$t('settings.safety.specialTagsLimit.title')" v-bind:comment="$t('settings.safety.specialTagsLimit.comment')" instance="specialTagsLimit" />
                <Range v-bind:title="$t('settings.safety.youtubePreloadLimit.title')" v-bind:comment="$t('settings.safety.youtubePreloadLimit.comment')" instance="youtubePreloadLimit" />
                <Range v-bind:title="$t('settings.safety.imgurPreloadLimit.title')" v-bind:comment="$t('settings.safety.imgurPreloadLimit.comment')" instance="imgurPreloadLimit" />
                <Range v-bind:title="$t('settings.safety.repliesLimit.title')" v-bind:comment="$t('settings.safety.repliesLimit.comment')" instance="repliesLimit" />
            </div>
        </div>

        <div class="section section-captcha">
            <div class="title" @click="captchaSectionEnabled = !captchaSectionEnabled">{{ $t('titles.captcha') }}</div>
            <div class="body" v-if="captchaSectionEnabled">
                <Radio :default="$store.state.settings.captchaKind" :variants="[{ name: $t('captchaKinds.hcaptcha'), 'value': 'hcaptcha' }, { name: $t('captchaKinds.dcaptcha'), value: 'dcaptcha' }]" v-on:change="changeCaptcha" />
            </div>
        </div>

        <div class="section section-admin">
            <div class="title" @click="adminSectionEnabled = !adminSectionEnabled">{{ $t('titles.admin') }}</div>
            <div class="body" v-if="adminSectionEnabled">
                <div class="authorization-form">
                    <button class="btn btn-4" @click="request">  REQUEST  </button>
                    <button class="btn btn-1" @click="deauth" v-if="$store.state.admin.approved">  DEAUTHORIZE  </button>
                    <input v-model="sections.admin.token" type="text" class="text-input" placeholder="128-bit UUID">
                </div>

                <div v-if="$store.state.admin.approved" class="if-approved">
                    <pre>{{ $t('settings.admin.guide') }}</pre>
                    <div class="row-sections">
                        <div class="section">
                            <h2>  {{ $t('settings.admin.roles') }}  </h2>
                            <div class="content">
                                <p v-for="role in $store.state.admin.roles">{{ role }}</p>
                            </div>
                        </div>
                        <div class="section">
                            <h2>  {{ $t('settings.admin.perms') }}  </h2>
                            <div class="content">
                                <p v-for="perm in $store.state.admin.perms">{{ perm }}</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <button class="btn btn-1" @click="resetLanguage">  CHANGE THE LANGUAGE  </button>
    </div>
</template>

<script>
    import Radio         from './SettingsRadioSelect'
    import Option        from './SettingsOption'
    import Input         from './SettingsInput'
    import Range         from './SettingsRange'
    import themes        from '../../../../app/themes'
    import fonts         from '../../../../app/fonts'
    import Regexes       from '../../../../app/Regexes'
    import Admin         from '@/services/Admin'
    import Reasons       from '@/services/Reasons'

    export default {
        components: { Input, Range, Option, Radio },

        data() {
            return {
                safetySectionEnabled: false,
                themesSectionEnabled: false,
                fontsSectionEnabled: false,
                uiSectionEnabled: false,
                adminSectionEnabled: false,
                captchaSectionEnabled: false,
                backgroundSectionEnabled: false,
                themesSectionMenu: '',
                customSchemeInput: '',
                currentTheme: { name: '', type: '' },

                sections: {
                    admin: {
                        token: ""
                    },

                    captcha: {
                        kind: ""
                    },

                    safety: {
                        tagLimits: 111
                    }
                }
            }
        },

        computed: {
            themesLight() {
                return themes.light
            },

            themesDark() {
                return themes.dark
            },

            fonts() {
                return fonts
            }
        },

        methods: {
            setTheme (theme) {
                this.currentTheme = theme
                this.$setTheme(theme)
            },

            applyCustomScheme () {
                let colors = this.customSchemeInput.split(',')
                if (colors.length < 8) {
                    return this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("settings.colorSchemes.customMinLength"),
                        "duration": 5000
                    })
                }

                for (const color of colors) {
                    if (!Regexes.hexColor.test(color)) {
                        return this.$notify({
                            "group": "main",
                            "type":  "error",
                            "title": this.$t("settings.colorSchemes.customInvalidFormat"),
                            "duration": 5000
                        })
                    }
                }

                const theme = { type: 'custom', name: 'custom', colors, save: true }
                this.$setTheme(theme)
                this.currentTheme = theme
            },

            resetLanguage () {
                this.$store.commit("changeLanguage", '')
            },

            changeCaptcha (value) {
                this.$store.commit('changeCaptcha', value)
            },

            async request() {
                this.$block();
                try {
                    const admin = await Admin.get(this.sections.admin.token)
                    const reasons = await Reasons.get(this.sections.admin.token)
                    this.$store.commit('authorize', {
                        'token': this.sections.admin.token,
                        'roles': admin.roles,
                        'perms': admin.perms,
                    })
                    this.$store.commit('setReasons', reasons)
                    this.$notify({
                        "group": "main",
                        "type":  "success",
                        "title": this.$t("settings.admin.authorizationSuccess"),
                        "duration": 5000
                    })
                    this.sections.admin.token = ""
                } catch(e) {
                    this.$notify({
                        "group": "main",
                        "type":  "error",
                        "title": this.$t("settings.admin.authorizationFailed"),
                        "text":  this.$t(`apiErrorCodes.${ e.code }`),
                        "duration": 5000
                    })
                }
                this.$unblock();
            },

            deauth() {
                this.$store.commit('deauthorize')
                this.$notify({
                    "group": "main",
                    "type":  "success",
                    "title": this.$t("settings.admin.deauthSuccess"),
                    "duration": 5000
                })
            }
        },

        mounted() {
            if (this.$setTitle) this.$setTitle(this.$t("titles.settings"))
        
            let theme = this.$localStorage.get('theme')
            if (theme) {
                if (theme.type == 'custom') {
                    this.customSchemeInput = theme.colors.join(',')
                }

                this.currentTheme = theme
            }
        }
    }
</script>
