<script setup lang="ts">
import ThemeIcon from "../icons/theme-icon.vue";

interface Emits {
	// biome-ignore lint/style/useShorthandFunctionType: ;-)
	(evtName: "toggle-theme"): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
	<nav role="application" class="app:controls">
		<button
			hidden
			type="button"
			title="Changer le thème de l'application"
			@click="emit('toggle-theme')"
		>
			<ThemeIcon />
		</button>
	</nav>
</template>

<style scoped>
button,
button > svg,
button > svg * {
	opacity: 0.9;
	cursor: pointer;

	&:hover {
		opacity: 1;
	}
}
</style>
