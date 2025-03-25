<script setup lang="ts">
import { invoke, type InvokeArgs } from "@tauri-apps/api/core";
// import { readTextFile } from "@tauri-apps/plugin-fs";
// import { appDataDir, resolve } from "@tauri-apps/api/path";
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";

import type { Service } from "@alephonor/domain/entities/service";
import type { Page } from "@alephonor/domain/pages/enum";
import DashboardPage from "@alephonor/system-design/pages/dashboard-page.vue";

const router = useRouter();

let services = ref<Array<Service>>([]);

onMounted(async () => {
	services.value = await invoke("all_services");

	// services.value = JSON.parse(
	// 	await readTextFile(await resolve(await appDataDir(), "services.json")),
	// );
});

function changePage(s: Page) {
	router.push({ name: s });
}

async function callBackend<T>(name: string, args?: InvokeArgs): Promise<T> {
	return invoke(name, args);
}
</script>

<template>
	<DashboardPage
		v-model="services"
		@call-backend="callBackend"
		@change-page="changePage"
	>
		<template v-slot="{ services, onCallBackend }">
			<Suspense>
				<RouterView
					:services="services"
					:onCallBackend="onCallBackend"
				/>
			</Suspense>
		</template>
	</DashboardPage>
</template>
