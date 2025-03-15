<script setup lang="ts">
import IconClose from "../icons/icon-close.vue";
import IconMaximize from "../icons/icon-maximize.vue";
import IconMinimize from "../icons/icon-minimize.vue";

interface Props {
	// TODO
	snapLayout?: boolean;
}

interface Emits {
	(evtName: "close"): void;
	(evtName: "maximize"): void;
	(evtName: "minimize"): void;
}

withDefaults(defineProps<Props>(), {
	snapLayout: false,
});
const emit = defineEmits<Emits>();
</script>

<template>
	<nav class="win:controls">
		<ul>
			<li>
				<button
					type="button"
					title="Réduire l'application"
					@click="emit('minimize')"
				>
					<IconMinimize />
				</button>
			</li>

			<li>
				<button
					type="button"
					title="Maximiser/réduire l'application"
					@click="emit('maximize')"
				>
					<IconMaximize />
				</button>
			</li>

			<li>
				<button
					type="button"
					title="Quitter l'application"
					class="win:close-control"
					@click="emit('close')"
				>
					<IconClose />
				</button>
			</li>
		</ul>
	</nav>
</template>

<style>
.win\:controls {
	flex-shrink: 0;
}
.win\:controls button,
.win\:controls button > svg,
.win\:controls button > svg * {
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
