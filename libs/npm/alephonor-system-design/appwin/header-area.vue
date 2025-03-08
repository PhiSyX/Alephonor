<script setup lang="ts">
import AppControls from "./app-controls.vue";
import WinControls from "./win-controls.vue";

interface Emits {
	(evtName: "close"): void;
	(evtName: "drag"): void;
	(evtName: "maximize"): void;
	(evtName: "minimize"): void;
	(evtName: "toggle-theme"): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
	<header role="application, window" class="app:win:header">
		<AppControls @toggle-theme="emit('toggle-theme')" />
		<WinControls
			@close="emit('close')"
			@maximize="emit('maximize')"
			@minimize="emit('minimize')"
		/>
	</header>
</template>

<style>
.app\:win\:header {
	display: flex;
	justify-content: end;
	gap: var(--spacing-4);
	padding: var(--spacing-1);
}
</style>
