<script setup lang="ts">
import { computed, shallowRef } from "vue";

import HttpServer, {
	type Config as HttpServerConfig,
	type State as HttpServerState,
} from "./webserver/http-server.vue";
import LangPhp, {
	type Config as LangPHPConfig,
	type State as LangPHPState,
} from "./webserver/lang-php.vue";
import DbServer, {
	type Config as DbServerConfig,
	type State as DbServerState,
} from "./webserver/db-server.vue";

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
					<a href="#http-server" @click="changeTab($event, 'http-server')">
						Serveur HTTP
					</a>
				</li>

				<li>
					<a href="#php" @click="changeTab($event, 'php')">
						PHP
					</a>
				</li>

				<li>
					<a href="#db-server" @click="changeTab($event, 'db-server')">
						Base de données
					</a>
				</li>
			</ul>

			<button type="submit" form="form_webserver">Sauvegarder</button>
		</nav>

		<form class="container" action="" method="post" id="form_webserver">
			<HttpServer
				v-if="currentTab === 'http-server'"
				v-model="state.http"
				:config="config.http"
				class="form-webserver-group"
			/>

			<LangPhp 
				v-if="currentTab === 'php'" 
				v-model="state.php"
				:config="config.php"
				class="form-webserver-group"
			/>

			<DbServer
				v-if="currentTab === 'db-server'" 
				v-model="state.db"
				:config="config.db"
				class="form-webserver-group"
			/>
		</form>
	</article>
</template>

<style lang="scss">
@use "@alephonor/sheets/abstracts/functions" as fn;
@use "@alephonor/sheets/abstracts/mixins" as mx;

.dashboard\:screen\:content:has(.env\:webserver) {
	/* --dashboard-screen: url(./amp.png); */
	padding: 0;
}

.form-webserver-group {
	display: grid;
	gap: 1rem;

	> div > * + * {
		margin-top: .25rem;
	}

	select,
	input[type="number"],
	input[type="text"] {
		width: 100%;
		padding: .5rem;
		border-radius: 6px;
		background: var(--color-black);
		color: var(--color-white);
		appearance: none;
	}

	input[type="checkbox"] {
		@include mx.size(24);

		outline: none !important;
		border: 1px solid var(--color-blue-grey-200);
		border-radius: 4px;
		background: var(--color-blue-grey-100);
		appearance: none;

		&:hover {
			border-style: inset;
		}

		&:checked {
			border-style: inset;
			background: var(--color-blue-grey-200);
			background-image: fn.svg('<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M9.00001 20.42L2.79001 14.21L5.62001 11.38L9.00001 14.77L18.88 4.88L21.71 7.71L9.00001 20.42Z" fill="currentColor" /></svg>');
		}

		&:active {
			border-style: outset;
		}
	}

	select option {
		color: var(--color-white);
	}
}
</style>

<style scoped>
.container {
	max-width: 80ch;
	margin-inline: auto;
}

.breadcrumb {
	padding: .5rem;
}

.breadcrumb {
	display: flex;
	align-items: center;
	margin-bottom: 3rem;
	border-bottom: 1px inset var(--color-blue-grey-100);
}

.breadcrumb ul {
	display: flex;
	gap: 8px;
	flex-grow: 1;
}

.breadcrumb ul li {
	padding: 4px;
	border: 1px solid var(--color-blue-grey-200);
	border-radius: 4px;
	background: var(--color-blue-grey-100);
}

button[type="submit"] {
	margin-left: auto;

	padding: .5rem;
	border-radius: 6px;
	color: var(--color-white);
	background: var(--color-black);
}
</style>
