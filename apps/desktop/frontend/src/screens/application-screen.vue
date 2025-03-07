<script setup lang="ts">
import { computed, onMounted, ref } from "vue";

import type { Screen } from "#screens";

import IconLoader from "#components/icons/icon-loader.vue";

export interface AmpStatus {
	apache2: string;
	mysql: string;
}

interface Props {
	onCheckAmpServiceStatus(): Promise<AmpStatus>;
	onStartAmpService(): Promise<AmpStatus>;
	onStopAmpService(): Promise<AmpStatus>;
}

interface Emits {
	// biome-ignore lint/style/useShorthandFunctionType: .-)
	(event_name: "change-screen", s: Screen): void;
}

const { onCheckAmpServiceStatus, onStartAmpService, onStopAmpService } =
	defineProps<Props>();

defineEmits<Emits>();

const amp_status = ref<AmpStatus>({
	apache2: "apache not running",
	mysql: "mysql not running",
});
const amp_loading = ref(false);

const amp_status_is_not_running = computed(
	() =>
		amp_status.value.apache2 === "apache not running" ||
		amp_status.value.mysql === "mysql not running"
);

const amp_status_is_already_running = computed(
	() =>
		amp_status.value.apache2 === "apache already running" &&
		amp_status.value.mysql === "mysql already running"
);

async function check_amp_service_status() {
	amp_status.value = await onCheckAmpServiceStatus();
}

async function start_amp_service() {
	amp_loading.value = true;
	amp_status.value = await onStartAmpService();
	amp_loading.value = false;
}

async function stop_amp_service() {
	amp_loading.value = true;
	amp_status.value = await onStopAmpService();
	amp_loading.value = false;
}

onMounted(() => {
	check_amp_service_status();
});
</script>

<template>
	<section class="apps">
		<article
			class="app app-amp"
			:class="{
				'is-not-running': amp_status_is_not_running,
				'is-running': amp_status_is_already_running,
				'is-loading': amp_loading,
			}"
		>
			<h1>
				Apache, PHP et MySQL
				<span class="dot"></span>
			</h1>

			<button
				v-if="amp_status_is_not_running"
				:disabled="amp_loading"
				@click="start_amp_service"
			>
				Démarrer <IconLoader v-if="amp_loading" />
			</button>

			<button
				v-if="amp_status_is_already_running"
				:disabled="amp_loading"
				@click="stop_amp_service"
			>
				Stopper <IconLoader v-if="amp_loading" />
			</button>
		</article>
	</section>
</template>

<style lang="scss" scoped>
.apps {
	display: flex;
	gap: 1rem;
}

.app {
	padding: 2rem;
	border: 2px solid var(--status-color);
	border-radius: 8px;
	box-shadow: 1px 1px 2px var(--color-ultra-black);

	> * + * {
		margin-top: 1.5rem;
	}

	h1 {
		margin: 0;
	}

	button {
		width: 100%;
		padding: 0.5rem;
		border: none;
		border-radius: 8px;
		background-color: var(--color-white);
		color: var(--color-black);
		cursor: pointer;
		transition: background 250ms ease;

		&[disabled] {
			opacity: 0.5;
			pointer-events: none;
			cursor: default;
		}

		&:hover {
			background-color: var(--color-ultra-white);
		}
	}
}

.dot {
	display: inline-block;
	width: 0.5rem;
	height: 0.5rem;
	margin-left: 0.5rem;
	border-radius: 999px;
	background-color: var(--status-color);
	box-shadow: 1px 1px 10px var(--status-color);
}

.is-loading {
	--status-color: var(--color-orange-800) !important;
}
.is-not-running {
	--status-color: var(--color-red-800);
}
.is-running {
	--status-color: var(--color-green-600);
}
</style>
