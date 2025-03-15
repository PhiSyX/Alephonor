<script setup lang="ts">
import type { Service } from "@alephonor/domain/entities/service";
import type { ScreenEmits } from "./index";

interface Props {
	onCallBackend<T>(pathname: string, data?: object): Promise<T>;
}

interface Emits extends ScreenEmits {}

interface Slots {
	default: {
		services: Array<Service>;
		onCallBackend: Props["onCallBackend"];
	};
}

type DefaultModel = Array<Service>;

defineProps<Props>();
defineEmits<Emits>();
defineSlots<Slots>();
let servicesModel = defineModel<DefaultModel>();
</script>

<template>
	<section class="dashboard:screen">
		<aside>
			<details open>
				<summary>Environnements</summary>

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

						<li>
							<RouterLink to="/env/webserver">
								Serveur Web
							</RouterLink>
						</li>
					</ul>
				</nav>
			</details>

			<details open hidden>
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

			<details open hidden>
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

			<details open hidden>
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

		<section class="dashboard:screen:content">
			<slot :services="servicesModel" @call-backend="onCallBackend" />
		</section>
	</section>
</template>

<style lang="scss">
@use "./dashboard-screen.root";
</style>

<style lang="scss" scoped>
@use "./dashboard-screen";
</style>
