<template>
	<div class="board-information">
		<div class="desc">{{ board.description }}</div>
		<div class="rules">{{ board.rules }}</div>
		<div class="space"></div>
		<div class="divider"></div>
		<div class="settings">
			<p>{{ $t("boardInfo.settings.slowmode") }}: <span>{{ board.slowmode ? `${Math.floor(board.slowmode/60)} ${$t('minutes')}` : $t("disabled") }}</span></p>
			<p>{{ $t("boardInfo.settings.bumplimit") }}: <span>{{ board.bumplimit }}</span></p>
			<!-- <p>{{ $t("boardInfo.count") }}: </p> -->
			<p>{{ $t("boardInfo.settings.last_replies") }}: <span>{{ board.last_replies }}</span></p>
			<p>{{ $t("boardInfo.settings.pages_count") }}: <span>{{ board.pages_count }}</span></p>
			<p>{{ $t("boardInfo.settings.per_page") }}: <span>{{ board.per_page }}</span></p>
			<p>{{ $t("boardInfo.settings.tripcode_enabled") }}: <span>{{ board.tripcode_enabled ? $t("yes") : $t("no") }}</span></p>
			<h2>OP</h2>
			<p>{{ $t("boardInfo.settings.op.delete") }}: <span>{{ board.op_deletion_enabled ? $t("yes") : $t("no") }}</span></p>
			<p>{{ $t("boardInfo.settings.op.oppost") }}: <span>{{ board.op_oppost_enabled ? $t("yes") : $t("no") }}</span></p>
		</div>
		<!-- <div class="controls"> -->
			<!-- <button class="btn btn-2" @click="close">  {{$t('commonButtons.close')}}  </button> -->
		<!-- </div> -->
	</div>
</template>

<script>
	export default {
		props: ["board"],

		methods: {
			close () {
				this.$emit('close')
			}
		}
	}
</script>
