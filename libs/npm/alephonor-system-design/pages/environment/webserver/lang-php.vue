<script setup lang="ts">
import { computed } from "vue";

export interface Config {
	available_list: Array<{ id: number; semver: string }>;
    default_config_file: string;
    composer: boolean;
	xdebug: boolean;
}

export interface State {
	current_version: number;
	[semver: number]: {
		config_file?: string;
		composer?: boolean;
		xdebug?: boolean;
	};
}

interface Props {
	config: Config;
}

const { config } = defineProps<Props>();
let state = defineModel<State>();


const userDbServer = computed(() => {
	let server = config.available_list.find(
		(s) => state.value?.current_version === s.id,
	);

	if (!server) {
		return;
	}

	return {
		...server,
		user_config_file: state.value?.[server.id]?.config_file,
		user_composer: state.value?.[server.id]?.composer,
		user_xdebug: state.value?.[server.id]?.xdebug,
	};
});
</script>

<template>
    <fieldset >
        <legend class="h2-like">Configuration PHP</legend>

        <div>
            <label for="php_version">
                Choisir la version de PHP
            </label>

            <select id="php_version" name="php_version">
                <option
                    v-for="version of config.available_list"
                    :key="version.id"
                    :value="version.id"
                    :selected="userDbServer?.id === version.id"
                >
                    {{ version.semver }}
                </option>
            </select>
        </div>

        <div>
            <label for="php_config_file">
                Emplacement du fichier de configuration de PHP
            </label>
            <input
                type="text"
                id="php_config_file"
                :placeholder="config.default_config_file"
                :value="userDbServer?.user_config_file"
            />
        </div>

        <div>
            <input 
                type="checkbox" 
                name="php_composer" 
                id="composer" 
                :checked="userDbServer?.user_composer ?? config.composer"
            />
            &nbsp;
            <label for="composer">Activer Composer (Gestionnaire de paquet)</label>
        </div>

        <div>
            <input 
                type="checkbox" 
                name="php_ext_xdebug" 
                id="xdebug"
                :checked="userDbServer?.user_xdebug ?? config.xdebug"
            >
            &nbsp;
            <label for="xdebug">Activer l'extension XDebug</label>
        </div>
    </fieldset>
</template>