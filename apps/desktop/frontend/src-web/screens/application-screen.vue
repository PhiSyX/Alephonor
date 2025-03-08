<script setup lang="ts">
import { ref } from "vue";

import type { Service } from "@alephonor/domain/entities/service";
import {
	emitChangeScreen,
	type ScreenEmits,
} from "@alephonor/system-design/screens";
import ApplicationScreen from "@alephonor/system-design/screens/application-screen.vue";

interface Emits extends ScreenEmits {}

defineEmits<Emits>();

let services = ref<Array<Service>>([
	{
		name: "service1",
		title: "Un service",
		installed: true,
		status: true,
		commands: {
			install: "install_service1",
			check: "check_service1",
			start: "start_service1",
			stop: "stop_service1",
		},
	},
	{
		name: "service2",
		title: "Un deuxième service",
		installed: false,
		status: false,
		commands: {
			install: "install_service2",
			check: "check_service2",
			start: "start_service2",
			stop: "stop_service2",
		},
	},
]);

function sleep(ms: number) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

async function callBackend<T>(name: string, args?: object): Promise<T> {
	console.log(name);

	let _service = services.value.find((s) =>
		Object.values(s.commands).includes(name)
	);
	if (!_service) {
		return false as T;
	}

	await sleep(500);

	switch (name) {
		case "check_service1":
		case "check_service2":
			return _service.status;

		case "install_service1":
		case "install_service2":
			{
				await sleep(1000);
				_service.installed = true;
			}
			break;

		case "start_service1":
		case "start_service2":
			{
				_service.status = true;
			}
			break;

		case "stop_service1":
		case "stop_service2":
			{
				_service.status = false;
			}
			break;
	}

	return false as T;
}
</script>

<template>
	<ApplicationScreen
		v-model="services"
		@call-backend="callBackend"
		@change-screen="(s) => emitChangeScreen($emit)(s)"
	/>
</template>
