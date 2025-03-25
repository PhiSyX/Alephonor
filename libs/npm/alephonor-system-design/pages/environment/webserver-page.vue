<script setup lang="ts">
import { shallowRef } from "vue";

import HttpServer, {
	type Config as HttpServerConfig,
	type State as HttpServerState,
} from "../../screens/environment/webserver/http-server.vue";
import LangPhp, {
	type Config as LangPHPConfig,
	type State as LangPHPState,
} from "../../screens/environment/webserver/lang-php.vue";
import DbServer, {
	type Config as DbServerConfig,
	type State as DbServerState,
} from "../../screens/environment/webserver/db-server.vue";

interface Props {
	config: {
		db: DbServerConfig;
		http: HttpServerConfig;
		php: LangPHPConfig;
	};
}

interface State {
	db?: DbServerState;
	http?: HttpServerState;
	php?: LangPHPState;
}

const { config } = defineProps<Props>();

let state = defineModel<State>({ required: true });
let currentTab = shallowRef("http-server");

function changeTab(evt: MouseEvent, tab: "http-server" | "db-server" | "php") {
	evt.preventDefault();
	currentTab.value = tab;
}
</script>

<template>
	<article class="env:webserver">
		<nav role="navigation" class="breadcrumb">
			<ul role="tablist">
				<li>
					<a
						href="#http-server"
						@click="changeTab($event, 'http-server')"
					>
						Serveur HTTP
					</a>
				</li>

				<li>
					<a href="#php" @click="changeTab($event, 'php')"> PHP </a>
				</li>

				<li>
					<a
						href="#db-server"
						@click="changeTab($event, 'db-server')"
					>
						Base de données
					</a>
				</li>
			</ul>

			<button type="submit" form="form_webserver">Sauvegarder</button>
		</nav>

		<form class="container" action="#" method="post" id="form_webserver">
			<HttpServer
				v-show="currentTab === 'http-server'"
				v-model="state.http"
				:config="config.http"
				class="form-webserver-group"
			/>

			<LangPhp
				v-show="currentTab === 'php'"
				v-model="state.php"
				:config="config.php"
				class="form-webserver-group"
			/>

			<DbServer
				v-show="currentTab === 'db-server'"
				v-model="state.db"
				:config="config.db"
				class="form-webserver-group"
			/>
		</form>
	</article>
</template>

<style lang="scss">
@use "../../assets/pages/environment-webserver-page.root";
</style>

<style lang="scss">
@use "../../assets/pages/environment-webserver-page.inherit";
</style>

<style lang="scss" scoped>
@use "../../assets/pages/environment-webserver-page";
</style>
