<script setup lang="ts">
import { computed, onMounted, shallowRef } from "vue";

import type { Service } from "@alephonor/domain/entities/service";

import IconLoader from "../../../components/icons/icon-loader.vue";

export interface Props {
	service: Service;
	onCallBackend<T>(pathname: string, data?: object): Promise<T>;
}

const { service, onCallBackend } = defineProps<Props>();

const status = shallowRef(false);
const isLoading = shallowRef(false);

const isInstalled = computed(() => service.installed);
const isRunning = computed(() => service.installed && status.value === true);
const isNotRunning = computed(
	() => service.installed && status.value === false
);

async function checkService() {
	isLoading.value = true;

	// Maybe the command doesn't exist
	try {
		status.value = await onCallBackend(service.commands.check);
	} finally {
		isLoading.value = false;
	}
}
async function startService() {
	isLoading.value = true;

	// Maybe the command doesn't exist
	try {
		await onCallBackend(service.commands.start);
		status.value = await onCallBackend(service.commands.check);
	} finally {
		isLoading.value = false;
	}
}
async function stopService() {
	isLoading.value = true;

	// Maybe the command doesn't exist
	try {
		await onCallBackend(service.commands.stop);
		status.value = await onCallBackend(service.commands.check);
	} finally {
		isLoading.value = false;
	}
}
async function installService() {
	isLoading.value = true;

	// Maybe the command doesn't exist
	try {
		await onCallBackend(service.commands.install);
		status.value = await onCallBackend(service.commands.check);
	} finally {
		isLoading.value = false;
	}
}

onMounted(async () => await checkService());
</script>

<template>
	<article
		:id="service.name"
		:class="{
			'is-not-running': isNotRunning,
			'is-not-installed': !isInstalled,
			'is-running': isRunning,
			'is-loading': isLoading,
		}"
		class="app:service"
	>
		<img v-if="service.image" :src="service.image" alt="" />

		<h1>
			{{ service.title }}
			<span class="dot"></span>
		</h1>

		<template v-if="isInstalled">
			<button
				v-if="isNotRunning"
				:disabled="isLoading"
				@click="startService"
			>
				Démarrer <IconLoader v-if="isLoading" />
			</button>

			<button v-if="isRunning" :disabled="isLoading" @click="stopService">
				Stopper <IconLoader v-if="isLoading" />
			</button>
		</template>
		<template v-else>
			<button :disabled="isLoading" @click="installService">
				Installer <IconLoader v-if="isLoading" />
			</button>
		</template>
	</article>
</template>

<style lang="scss" scoped>
@use "../../../assets/pages/dashboard-resume-application-service.scss";
</style>
