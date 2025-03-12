<script setup lang="ts">
import { invoke, type InvokeArgs } from "@tauri-apps/api/core";
// import { readTextFile } from "@tauri-apps/plugin-fs";
// import { appDataDir, resolve } from "@tauri-apps/api/path";
import { onMounted, ref } from "vue";

import type { Service } from "@alephonor/domain/entities/service";
import type { Screen } from "@alephonor/domain/screens/enum";
import DashboardScreen from "@alephonor/system-design/screens/dashboard-screen.vue";
import { useRouter } from "vue-router";

const router = useRouter();

let services = ref<Array<Service>>([]);

onMounted(async () => {
	services.value = await invoke("all_services");

	// services.value = JSON.parse(
	// 	await readTextFile(await resolve(await appDataDir(), "services.json")),
	// );
});

function changeScreen(s: Screen) {
	router.push({ name: s });
}

async function callBackend<T>(name: string, args?: InvokeArgs): Promise<T> {
	return invoke(name, args);
}
</script>

<template>
	<DashboardScreen
		v-model="services"
		@call-backend="callBackend"
		@change-screen="changeScreen"
	/>
</template>
