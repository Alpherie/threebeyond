<template>
	<div id="board-create-edit" v-if="loaded">
		<div class="block" @click="cancel"></div>
		<div class="form">
			<div class="head">
				<div class="title"><span v-if="defaultValue">Edit board</span><span v-else>Create new board</span></div>
				<div class="buttons-wrap">
					<button class="btn btn-1 btn-a" @click="cancel">Cancel</button>
					<button class="btn btn-4 btn-a" @click="save">Save</button>
				</div>
			</div>
			<input type="text" placeholder="Short" v-model="short" v-if="!defaultValue">
			<input type="text" placeholder="Name" v-model="name">
			<input type="text" placeholder="Slowmode timeout" v-model="slowmode">
			<input type="text" placeholder="Pages count"  v-model="pages_count">
			<input type="text" placeholder="Threads per page" v-model="per_page">
			<input type="text" placeholder="Show last replies" v-model="last_replies">
			<input type="text" placeholder="Bumplimit" v-model="bumplimit">
			<input type="text" placeholder="Description" v-model="description">
			<textarea name="" id="" cols="30" rows="5" placeholder="Rules" v-model="rules"></textarea>
			<div class="options">
				<Checkbox id="thread_creation_limited" label="Thread creation limited" :default="thread_creating_limited" @input="(v) => thread_creating_limited = v" />
				<Checkbox id="op_can_delete_posts" label="OP can delete posts" :default="op_deletion_enabled" @input="(v) => op_deletion_enabled = v" />
				<Checkbox id="op_can_oppost" label="OP post enabled" :default="op_oppost_enabled" @input="(v) => op_oppost_enabled = v" />
				<Checkbox id="tripcode_enabled" label="Tripcode enabled" :default="tripcode_enabled" @input="(v) => tripcode_enabled = v" />
			</div>
		</div>
	</div>
</template>

<script>
	import Checkbox from "@/components/Checkbox"
	import Admin from "@/services/Admin"

	export default {
		components: { Checkbox },

		props: ["lang", "defaultValue"],

		data () {
			return {
				loaded: false,

        short: "",
				name: "",
				slowmode: "",
				pages_count: "",
				per_page: "",
				last_replies: "",
				bumplimit: "",
				description: "",
				rules: "",
				thread_creating_limited: false,
				op_deletion_enabled: false,
				op_oppost_enabled: false,
			  tripcode_enabled: false
      }
		},

		methods: {
			async save () {
				let query = {
					lang: this.lang,
					short: this.short,
					name: this.name,
					slowmode: this.slowmode ? parseInt(this.slowmode) : undefined,
					pages_count: parseInt(this.pages_count),
					per_page: parseInt(this.per_page),
					last_replies: parseInt(this.last_replies),
					bumplimit: parseInt(this.bumplimit),
					description: this.description,
					rules: this.rules,
					thread_creating_limited: this.thread_creating_limited ? true : false,
					op_oppost_enabled: this.op_oppost_enabled ? true : false,
					op_deletion_enabled: this.op_deletion_enabled ? true : false,
					tripcode_enabled: this.tripcode_enabled ? true : false
				}

				if (!this.defaultValue) {
					try {
						await Admin.createBoard(this.$store.state.admin.token, query)

						this.$notify({
	                        "group": "main",
	                        "type": "success",
	                        "title": this.$t("api.admin.creatingNewBoard.success"),
	                    })

	                    this.$emit('done')
					} catch (e) {
						this.$notify({
	                        "group": "main",
	                        "type":  "error",
	                        "title": this.$t("api.admin.creatingNewBoard.failed"),
	                        "text":  this.$t(`apiErrorCodes.${e.code}`)
	                    })
					}
				} else {
					try {
						let w = await Admin.editBoard(this.$store.state.admin.token, query)
						console.log(w)

						this.$notify({
	                        "group": "main",
	                        "type": "success",
	                        "title": this.$t("api.admin.editingBoard.success"),
	                    })

	                    this.$emit('done')
					} catch (e) {
						this.$notify({
	                        "group": "main",
	                        "type":  "error",
	                        "title": this.$t("api.admin.editingBoard.failed"),
	                        "text":  this.$t(`apiErrorCodes.${e.code}`)
	                    })
					}
				}
			},

			cancel () {
				this.$emit('cancel')
			}
		},

		mounted () {
			if (this.defaultValue) {
				let props = ["short", "name", "slowmode", "pages_count", "per_page", "last_replies", "bumplimit", "rules", "thread_creating_limited", "op_deletion_enabled", "op_oppost_enabled", "tripcode_enabled", "description"]

				for (const prop of props) {
          this[prop] = this.defaultValue[prop]
				}
			}
      this.loaded = true
		}
	}
</script>
