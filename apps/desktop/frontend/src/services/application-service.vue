<script setup lang="ts">
import IconLoader from "#components/icons/icon-loader.vue";
import { computed, onMounted, shallowRef } from "vue";

export interface Service {
	name: string;
	title: string;
	image?: string;
	commands: ServiceCommands;
}

export interface ServiceCommands {
	check: string;
	start: string;
	stop: string;
}

export interface Props {
	service: Service;
	onCallBackend<T>(pathname: string, data?: object): Promise<T>;
}

const { service, onCallBackend } = defineProps<Props>();

const status = shallowRef(false);
const isLoading = shallowRef(false);

const isRunning = computed(() => status.value === true);
const isNotRunning = computed(() => status.value === false);

async function checkService() {
	isLoading.value = true;
	status.value = await onCallBackend(service.commands.check);
	isLoading.value = false;
}
async function startService() {
	isLoading.value = true;
	await onCallBackend(service.commands.start);
	status.value = await onCallBackend(service.commands.check);
	isLoading.value = false;
}
async function stopService() {
	isLoading.value = true;
	await onCallBackend(service.commands.stop);
	status.value = await onCallBackend(service.commands.check);
	isLoading.value = false;
}

onMounted(async () => {
	checkService();
});
</script>

<template>
	<article
		class="app"
		:class="{
			'is-not-running': isNotRunning,
			'is-running': isRunning,
			'is-loading': isLoading,
		}"
	>
		<img v-if="service.image" :src="service.image" alt="" />

		<h1>
			{{ service.title }}
			<span class="dot"></span>
		</h1>

		<button v-if="isNotRunning" :disabled="isLoading" @click="startService">
			Démarrer <IconLoader v-if="isLoading" />
		</button>

		<button v-if="isRunning" :disabled="isLoading" @click="stopService">
			Stopper <IconLoader v-if="isLoading" />
		</button>
	</article>
</template>

<style scoped>
@import "#styles/services/application-service.css" layer(components);
</style>
