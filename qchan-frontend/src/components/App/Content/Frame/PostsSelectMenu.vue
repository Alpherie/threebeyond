<template>
	<div id="posts-select-menu" v-if="Object.keys(selection).length">
		<div class="count">
			{{ Object.keys(selection).length }} posts selected.
		</div>
		<div class="buttons">
			<ReportDialog v-if="shownReportDialog" :lang="$route.params.lang" :board="$route.params.dir" :postNums="postNums" v-bind:title="$t('dialogs.report.title')" v-bind:message="$t('dialogs.report.message')"  v-on:cancel="shownReportDialog = false" />
			<button class="btn btn-4" @click="shownReportDialog = true">
	        	<icon icon="exclamation-circle"/>
	        	Report
	        </button>

			<button class="btn btn-4" @click="hide">
	        	<icon icon="eye-slash"/>
	        	Hide
	        </button>

            <!-- <ConfirmDialog v-if="shownDeleteDialog" v-bind:title="$t('dialogs.deleteMultiple.title')" v-bind:message="$t('dialogs.deleteMultiple.message')" v-on:confirm="confirmPostsDeletion" v-on:cancel="shownDeleteDialog = false" />

			<button class="btn btn-third" @click="deletePosts" v-if="hasPermTo('delete') || isOp">
	        	<icon icon="trash"/>
	        	Delete
	        </button> -->

	       <!--  <button class="btn btn-1" @click="banPoster" v-if="hasPermTo('ban')">
	        	<icon icon="trash"/>
	        	Ban poster
	        </button> -->
		</div>
	</div>
</template>

<script>
	import Vue from 'vue'
    import ConfirmDialog from '@/components/App/Dialogs/ConfirmDialog'
    import ReportDialog from '@/components/App/Dialogs/ReportDialog'
    import Admin from '@/services/Admin'
    import Thread  from '@/services/Thread'

	export default {
        components: { ReportDialog, ConfirmDialog },

		data() {
			return {
				selection: {},
				shownDeleteDialog: false,
				shownReportDialog: false,
				shownBanDialog: false,

				thread: null
			}
		},

		computed: {
			postNums () {
				// this.selection.map((post) => post.id).join(',')
				let out = []
				for (let index in this.selection) {
					out.push(this.selection[index].post.num)
				}
				return out.join(',')
			},

			isOp () {
				return this.thread !== null ? this.thread.secret : false
			}
		},

		methods: {
			hasPermTo(perm) {
                let params = this.$route.params
                let label = `langs::${params.lang}::boards::${params.dir}::posts::${perm}`

                return this.$store.state.admin.perms.indexOf(label) != -1
            },

			async confirmPostsDeletion () {
                let params = this.$route.params
				this.shownDeleteDialog = false
				this.$block()

				if (this.isOp) {
					try {
	                    let res = await Thread.deletePostsMultiple({
	                    	'lang': params.lang,
	                    	'dir': params.dir,
	                    	'thread': this.thread.num,
	                    	'nums': Object.keys(this.selection),
	                    	'secret': this.thread.secret
	                    })
	                    this.$notify({
	                        "group": "main",
	                        "type":  "success",
	                        "title": this.$t("api.admin.postMultipleDelete.success"),
	                    })
	                } catch(e) {
	                	console.error(e)
	                    this.$notify({
	                        "group": "main",
	                        "type":  "error",
	                        "title": this.$t("api.admin.postMultipleDelete.failed"),
	                        "text":  this.$t(`apiErrorCodes.${e.code}`)
	                    })
	                }
				} else {
	                try {
	                    let res = await Admin.deletePostsMultiple(this.$store.state.admin.token, { 'lang': params.lang, 'dir': params.dir, 'nums': Object.keys(this.selection) })
	                    this.$notify({
	                        "group": "main",
	                        "type":  "success",
	                        "title": this.$t("api.admin.postMultipleDelete.success"),
	                    })
	                } catch(e) {
	                	console.error(e)
	                    this.$notify({
	                        "group": "main",
	                        "type":  "error",
	                        "title": this.$t("api.admin.postMultipleDelete.failed"),
	                        "text":  this.$t(`apiErrorCodes.${e.code}`)
	                    })
	                }
	            }
	            for (let key in this.selection) {
                	let obj = this.selection[key]
                	obj.vm.source.isDeleted = true
                	obj.vm.$forceUpdate()
                }
				this.clear()
                this.$unblock()
			},

			deletePosts () {
				this.shownDeleteDialog = true
			},

			hide () {
				for (let key in this.selection) {
					let obj = this.selection[key]
					obj.vm.hide()
				}

				this.clear()
			},

			select(vm, thread, post) {
				this.thread = thread

				if (this.selection[post.num]) {
					delete this.selection[post.num]
				} else {
					this.selection[post.num] = { vm, post }
				}
				this.$forceUpdate()
			},

			clear() {
				this.selection = {}
			}
		},

		mounted() {
			Vue.prototype.$selectPost = this.select
			Vue.prototype.$clearPostsSelection = this.select
		}
	}
</script>