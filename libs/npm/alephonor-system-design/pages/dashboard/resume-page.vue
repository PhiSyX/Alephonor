<script setup lang="ts">
import type { Service } from "@alephonor/domain/entities/service";

import ApplicationService from "./services/application-service.vue";

export interface Props {
	services: Array<Service>;
	onCallBackend<T>(pathname: string, data?: object): Promise<T>;
}

defineProps<Props>();
</script>
<template>
	<div class="dashboard:page:environments">
		<h1>
			Gérer les environnements
			<button type="button">Nouvel environnement</button>
		</h1>

		<div>
			<ApplicationService
				v-for="service of services"
				:key="`^${service.name}`"
				:service="service"
				@call-backend="onCallBackend"
			/>
		</div>
	</div>
</template>

<style lang="scss" scoped>
@use "../../assets/pages/dashboard-resume-page.scss";
</style>
