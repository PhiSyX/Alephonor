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
	<section class="dashboard:screen">
		<aside>
			<details open>
				<summary>Services</summary>

				<nav role="navigation">
					<ul role="tablist">
						<template
							v-for="service of servicesModel"
							:key="`${service.name}$`"
						>
							<li
								v-if="service.installed"
								:aria-controls="service.name"
							>
								<a href="#">
									{{ service.title }}
								</a>
							</li>
						</template>
					</ul>
				</nav>
			</details>
		</aside>

		<div class="dashboard:screen:environnements">
			<h1>
				Gérer les environnements
				<button type="button">Ajouter un nouvel environnement</button>
			</h1>

			<div>
				<ApplicationService
					v-for="service of servicesModel"
					:key="`^${service.name}`"
					:service="service"
					@call-backend="onCallBackend"
				/>
			</div>
		</div>
	</section>
</template>

<style>
@import "./dashboard-screen.vars.css";
</style>

<style scoped>
@import "./dashboard-screen.css" layer(screens);
</style>
