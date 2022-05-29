<template>
	<div class="checkbox">
		<input :id="id" type="checkbox" v-model="model" @input="input" />
        <label :for="id">
            <span></span>
            {{ label }}
            <ins><i>{{ label }}</i></ins>
        </label>
    </div>
</template>

<script>
	export default {
		props: ["label", "id", "default"],

        data () {
            return {
                model: false
            }
        },

        methods: {
            input () {
                this.$emit("input", !this.model)
            }
        },

        mounted () {
            this.model = this.default
        }
	}
</script>
