<script setup lang="ts">
import IconTheme from "../icons/icon-theme.vue";

interface Emits {
	(evtName: "toggle-theme"): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
	<nav class="app:controls">
		<ul>
			<li>
				<button
					hidden
					type="button"
					title="Changer le thème de l'application"
					@click="emit('toggle-theme')"
				>
					<IconTheme />
				</button>
			</li>
		</ul>
	</nav>
</template>

<style>
.app\:controls button {
	background: transparent;
}
.app\:controls button,
.app\:controls button > svg,
.app\:controls button > svg * {
	opacity: 0.9;
	cursor: pointer;

	&:hover {
		opacity: 1;
	}
}
</style>
