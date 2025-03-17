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

		<div class="app:win:title" dir="ltr">
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

<style lang="scss">
@use "@alephonor/sheets/abstracts/functions" as fn;
@use "@alephonor/sheets/abstracts/mixins" as mx;

.app\:win\:header {
	display: flex;
	justify-content: end;
	gap: fn.space(xl);
	padding: fn.space(sm);

	& > nav ul {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
	}
}

.app\:win\:title {
	margin-top: fn.space(md);
	flex-grow: 1;
	text-align: center;
	pointer-events: none;
}

.app\:searchbar {
	width: 25vw;
	padding: fn.space(sm);

	input {
		width: 100%;
		padding: fn.space(sm);
		border: none;
		border-radius: fn.radius(sm);
		/** fixme */
		background: fn.bg(grey, 700);
		text-overflow: ellipsis;
	}
}
</style>
