<script setup lang="ts">
import { defineAsyncComponent, type HTMLAttributes } from "vue";

interface Props {
	name: string;
	icon?: string;
	rootClass?: HTMLAttributes["class"];
}

defineOptions({
	inheritAttrs: false,
	components: {
		IconPassword: defineAsyncComponent(
			() => import("../components/icons/icon-password.vue")
		),
	},
});
defineProps<Props>();
const model = defineModel();
</script>

<template>
	<div class="input-container" :class="rootClass">
		<label :for="name">
			<component :is="`icon-${icon}`" />
		</label>

		<input :id="name" :name="name" v-bind="$attrs" v-model="model" />

		<slot />
	</div>
</template>

<style lang="scss">
@use "../assets/components/input.root";
</style>

<style lang="scss" scoped>
@use "../assets/components/input";
</style>
