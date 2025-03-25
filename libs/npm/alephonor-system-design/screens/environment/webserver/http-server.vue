<script setup lang="ts">
import { computed } from "vue";

export interface Config {
	available_list: Array<{
		id: number;
		name: string;
		default_config_file: string;
		default_working_dir: string;
		versions: Array<{ id: number; semver: string }>;
	}>;
	autoindex: boolean;
}

export interface State {
	current_server_id: number;

	[server_id: number]: {
		autoindex?: boolean;
		config_file?: string;
		port?: number;
		semver: number;
		working_dir?: Array<string>;
	};
}

interface Props {
	config: Config;
}

const { config } = defineProps<Props>();
let state = defineModel<State>();

const userHttpServer = computed(() => {
	let server = config.available_list.find(
		(s) => state.value?.current_server_id === s.id
	);

	if (!server) {
		return;
	}

	return {
		...server,
		user_autoindex: state.value?.[server.id]?.autoindex ?? config.autoindex,
		user_config_file: state.value?.[server.id]?.config_file,
		user_port: state.value?.[server.id]?.port,
		user_working_dir: state.value?.[server.id]?.working_dir,
	};
});

const httpServerURL = computed(() => {
	let port = userHttpServer.value?.user_port || 80;
	let s_port = port.toFixed();
	let s = port === 443 ? "s" : "";

	if (port === 80 || port === 443) {
		s_port = "";
	} else {
		s_port = `:${port}`;
	}

	return `http${s}://localhost${s_port}`;
});

function onSelect(e: Event & { target: HTMLSelectElement }) {
	let httpServerId = Number(e.target.value);

	let server = config.available_list.find((w) => w.id === httpServerId);

	if (!server) {
		return;
	}

	if (!state.value) {
		state.value = { current_server_id: server.id };
	}

	if (state.value?.[server.id] == null) {
		state.value[server.id] = {
			semver: server.versions[0].id,
			autoindex: config.autoindex,
		};
	}

	state.value.current_server_id = server.id;
}
</script>

<template>
	<div>
		<fieldset>
			<legend class="h2-like">Configuration du serveur HTTP</legend>

			<div>
				<label for="httpserver"> Choisir le serveur HTTP </label>

				<select name="httpserver" @change="onSelect">
					<option
						v-for="hs of config.available_list"
						:key="hs.name"
						:value="hs.id"
						:selected="hs.id === state?.current_server_id"
					>
						{{ hs.name }}
					</option>
				</select>
			</div>

			<div>
				<label for="httpserver_config_file">
					Emplacement du fichier de configuration du serveur HTTP
				</label>

				<input
					type="text"
					id="httpserver_config_file"
					:placeholder="userHttpServer?.default_config_file"
					:value="userHttpServer?.user_config_file"
				/>
			</div>

			<div>
				<label for="httpserver_port">
					Choisir le port d'accès au serveur HTTP
				</label>

				<input
					type="number"
					name="httpserver_port"
					id="httpserver_port"
					placeholder="80 OR 443 OR 8000..=8999"
					pattern="^(80|443|8[0-9]{3})$"
					required
					min="80"
					max="8999"
					:value="userHttpServer?.user_port"
				/>

				<span class="help">{{ httpServerURL }}</span>
			</div>

			<div>
				<input
					type="checkbox"
					name="httpserver_options_indexes"
					id="httpserver_options_indexes"
					:checked="userHttpServer?.user_autoindex"
				/>
				&nbsp;
				<label for="httpserver_options_indexes">
					Lister les fichiers lorsque le fichier d'index est manquant
					dans un répertoire
				</label>
			</div>
		</fieldset>

		<fieldset>
			<legend class="h3-like">
				Espace de travail
				<button
					type="button"
					class="btn-add"
					title="Nouvel espace de travail"
				>
					+
				</button>
			</legend>

			<div>
				<label for="httpserver_working_directory">
					Emplacement de l'espace de travail
				</label>

				<template v-if="userHttpServer?.user_working_dir">
					<input
						v-for="working_dir of userHttpServer?.user_working_dir"
						type="text"
						name="httpserver_working_directory[]"
						id="httpserver_working_directory"
						:placeholder="userHttpServer?.default_working_dir"
						:value="working_dir"
					/>
				</template>
				<template v-else>
					<input
						type="text"
						name="httpserver_working_directory[]"
						id="httpserver_working_directory"
						:placeholder="userHttpServer?.default_working_dir"
					/>
				</template>
			</div>
		</fieldset>
	</div>
</template>

<style>
.btn-add {
	float: right;
	border-radius: 99px;
	background: transparent;
	cursor: pointer;
}
</style>
