<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

import type { Screen } from "#screens";
import ApplicationScreen, {
	type AmpStatus,
} from "#screens/application-screen.vue";

interface Emits {
	// biome-ignore lint/style/useShorthandFunctionType: .-)
	(event_name: "change-screen", s: Screen): void;
}

defineEmits<Emits>();

function checkAmpServiceStatus(): Promise<AmpStatus> {
	return invoke<AmpStatus>("check_amp_service_status");
}

function startAmpService(): Promise<AmpStatus> {
	return invoke<AmpStatus>("start_amp_service");
}

function stopAmpService(): Promise<AmpStatus> {
	return invoke<AmpStatus>("stop_amp_service");
}
</script>

<template>
	<ApplicationScreen
		@check-amp-service-status="checkAmpServiceStatus"
		@start-amp-service="startAmpService"
		@stop-amp-service="stopAmpService"
		@change-screen="(s) => $emit('change-screen', s)"
	/>
</template>
