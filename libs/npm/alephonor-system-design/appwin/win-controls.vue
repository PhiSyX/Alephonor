<script setup lang="ts">
import CloseIcon from "../icons/close-icon.vue";
import MaximizeIcon from "../icons/maximize-icon.vue";
import MinimizeIcon from "../icons/minimize-icon.vue";

interface Emits {
	(evtName: "close"): void;
	(evtName: "maximize"): void;
	(evtName: "minimize"): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
	<nav role="window" class="win:controls">
		<button
			type="button"
			title="Réduire l'application"
			@click="emit('minimize')"
		>
			<MinimizeIcon />
		</button>

		<button
			type="button"
			title="Maximiser/réduire l'application"
			@click="emit('maximize')"
		>
			<MaximizeIcon />
		</button>

		<button
			type="button"
			title="Quitter l'application"
			class="win:close-control"
			@click="emit('close')"
		>
			<CloseIcon />
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

.win\:close-control {
	--close-surface: #7e0c0c;
	--close-bordered: #ac2020;

	&:hover {
		--close-surface: #ac2020;
		--close-bordered: #7e0c0c;
	}
}
</style>
