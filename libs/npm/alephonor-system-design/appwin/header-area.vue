<script setup lang="ts">
import AppControls from "./app-controls.vue";
import WinControls from "./win-controls.vue";

interface Props {
	showSearchbar?: boolean;
	showTitle?: boolean;
	title?: string;
}

interface Emits {
	(evtName: "close"): void;
	(evtName: "drag"): void;
	(evtName: "maximize"): void;
	(evtName: "minimize"): void;
	(evtName: "toggle-theme"): void;
}

withDefaults(defineProps<Props>(), {
	showSearchbar: false,
	showTitle: false,
	title: "",
});
const emit = defineEmits<Emits>();
</script>

<template>
	<header role="application, window" class="app:win:header">
		<div class="app:searchbar">
			<input
				v-if="showSearchbar"
				dir="ltr"
				type="search"
				placeholder="Chercher un service, une application, cours, etc."
			/>
		</div>

		<div class="app:win:title">
			<span v-if="showTitle">{{ title }}</span>
		</div>

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

	& > nav ul {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
	}
}

.app\:win\:title {
	margin-top: var(--spacing-2);
	flex-grow: 1;
	text-align: center;
}

.app\:searchbar {
	width: 25vw;
	padding: var(--spacing-1);

	input {
		width: 100%;
		padding: var(--spacing-1);
		border: none;
		border-radius: var(--radius-s);
		background: var(--color-grey-700);
		text-overflow: ellipsis;
	}
}
</style>
