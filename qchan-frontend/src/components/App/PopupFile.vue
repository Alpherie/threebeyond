<template>
	<div id="popup-file" v-if="details" @click="close">
		<div class="bar">
			<div class="title">{{ details.label }}</div>
			<div class="controls">
				<!-- <button class="btn btn-third"><icon icon="compress"/></button> -->
				<button class="btn btn-1"><icon icon="times"/></button>
			</div>
		</div>
		
		<div class="file">
			<div class="body">
				<img class="el" :src="details.source" alt="succ">
			</div>
		</div>
	</div>
</template>

<script>
	import $ from 'jquery'
	import Vue from 'vue'

	export default {
		data() {
			return {
				details: null,
			}
		},

		methods: {
			close() {
				this.details = null
			},

			display(details) {
				this.details = details

				this.$nextTick(function () {
					const el = $("#popup-file > .file")
					const bd = $("#popup-file > .file > .body > .el")
					const br = $("#popup-file > .file > .bar")
					const dc = $(document)

					const k    = this.details.width / details.height
	                let width  = this.details.width

	                while (width > window.screen.width * 0.5)
	                    width -= 128
	                
	                let height = width / k

	                while (height > window.screen.height * 0.5) {
	                    width -= 128
	                    height = width / k
	                }

	                bd.width(width)

					el.offset({ left: dc.width() / 2 - el.width() / 2, top: dc.height() / 2 - el.height()/2  })
					el.draggable()
					.bind('mousewheel DOMMouseScroll', function(event){
	                    // yet another css hack
	                    const whd  	   = bd.width()
	                    const before_h = bd.height()
	                    const before_w = bd.width()

	                    const k_w = event.offsetX / before_w
	                    const k_h = event.offsetY / before_h

	                    if (event.originalEvent.wheelDelta > 0 || event.originalEvent.detail < 0) {
	                        // scroll up
	                        bd.width(whd + 128)
	                    }
	                    else {
	                        // scroll down
	                        if (whd > 256) {
	                            bd.width(whd - 128)
	                        }
	                    }

	                    const diff_w = bd.width()  - before_w
	                    const diff_h = bd.height() - before_h

	                    const leftof = diff_w * -k_w
	                    const topof  = diff_h * -k_h
	                    
	                    const pos = el.offset()
	                    el.offset({ left: pos.left + leftof, top: pos.top + topof })
	                })
				})
			}
		},

		mounted() {
			Vue.prototype.$popupFile = this.display
		}
	}
</script>