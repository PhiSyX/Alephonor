<script setup lang="ts">
import { invoke, type InvokeArgs } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";

import { emitChangeScreen, type ScreenEmits } from "#screens";
import ApplicationScreen from "#screens/application-screen.vue";

interface Emits extends ScreenEmits {}

defineEmits<Emits>();

let services = ref([]);

onMounted(async () => {
	services.value = await invoke("all_services");
});
</script>

<template>
	<ApplicationScreen
		v-model="services"
		@call-backend="invoke"
		@change-screen="(s) => emitChangeScreen($emit)(s)"
	/>
</template>
