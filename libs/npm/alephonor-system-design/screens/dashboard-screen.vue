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

			<details open>
				<summary>Applications</summary>

				<nav role="navigation">
					<ul role="tablist">
						<li><a href="#">Design</a></li>
						<li><a href="#">Client Git</a></li>
						<li><a href="#">Gestion base de données</a></li>
						<li><a href="#">Schéma MCD</a></li>
						<li><a href="#">Schéma UML</a></li>
					</ul>
				</nav>
			</details>

			<details open>
				<summary>Formation</summary>

				<nav role="navigation">
					<ul role="tablist">
						<li><a href="#">Formateurs</a></li>
						<li><a href="#">Stagiaires</a></li>
						<li><a href="#">Présences</a></li>
						<li><a href="#">Statistiques</a></li>
					</ul>
				</nav>
			</details>

			<details open>
				<summary>Exercices</summary>

				<nav role="navigation">
					<ul role="tablist">
						<li><a href="#">Mes notes</a></li>
						<li><a href="#">Algorithmie</a></li>
						<li><a href="#">CSS</a></li>
						<li><a href="#">HTML</a></li>
						<li><a href="#">Java</a></li>
						<li><a href="#">JavaScript</a></li>
						<li><a href="#">PHP</a></li>
						<li><a href="#">SQL</a></li>
						<li><a href="#">UML</a></li>
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

<style lang="scss">
@use "./dashboard-screen.root";
</style>

<style lang="scss" scoped>
@use "./dashboard-screen";
</style>
