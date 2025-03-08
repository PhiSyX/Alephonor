<script setup lang="ts">
import type { Service } from "@alephonor/domain/entities/service";
import type { ScreenEmits } from "./index";

import ApplicationService from "../services/application-service.vue";

interface Props {
	onCallBackend<T>(pathname: string, data?: object): Promise<T>;
}

interface Emits extends ScreenEmits {}

type DefaultModel = Array<Service>;

defineProps<Props>();
defineEmits<Emits>();
let servicesModel = defineModel<DefaultModel>();
</script>

<template>
	<section class="app-screen">
		<ApplicationService
			v-for="service of servicesModel"
			:key="service.name"
			:service="service"
			@call-backend="onCallBackend"
		/>
	</section>
</template>

<style scoped>
@import "./application-screen.css" layer(screens);
</style>
