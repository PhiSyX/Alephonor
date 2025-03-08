<script setup lang="ts">
import { invoke, type InvokeArgs } from "@tauri-apps/api/core";
// import { readTextFile } from "@tauri-apps/plugin-fs";
// import { appDataDir, resolve } from "@tauri-apps/api/path";
import { onMounted, ref } from "vue";

import { emitChangeScreen, type ScreenEmits } from "#screens";
import ApplicationScreen from "#screens/application-screen.vue";

interface Emits extends ScreenEmits {}

defineEmits<Emits>();

let services = ref([]);

onMounted(async () => {
	services.value = await invoke("all_services");

	// services.value = JSON.parse(
	// 	await readTextFile(await resolve(await appDataDir(), "services.json")),
	// );
});

async function callBackend<T>(name: string, args?: InvokeArgs): Promise<T> {
	return invoke(name, args);
}
</script>

<template>
	<ApplicationScreen
		v-model="services"
		@call-backend="callBackend"
		@change-screen="(s) => emitChangeScreen($emit)(s)"
	/>
</template>
