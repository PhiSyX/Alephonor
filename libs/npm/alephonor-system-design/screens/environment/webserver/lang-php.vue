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

const timezones = {
	Afrique: [
		"Africa/Abidjan",
		"Africa/Bamako",
		"Africa/Brazzaville",
		"Africa/Casablanca",
		"Africa/Dakar",
		"Africa/Djibouti",
		"Africa/Johannesburg",
		"Africa/Kigali",
		"Africa/Kinshasa",
		"Africa/Libreville",
		"Africa/Nairobi",
		"Africa/Ouagadougou",
		"Africa/Tripoli",
		"Africa/Tunis",
	],

	Amérique: [
		"America/Anguilla",
		"America/Argentina/Buenos_Aires",
		"America/Argentina/Cordoba",
		"America/Belize",
		"America/Bogota",
		"America/Cancun",
		"America/Caracas",
		"America/Cayenne",
		"America/Cayman",
		"America/Chicago",
		"America/Costa_Rica",
		"America/Denver",
		"America/Detroit",
		"America/El_Salvador",
		"America/Guadeloupe",
		"America/Guatemala",
		"America/Guyana",
		"America/Havana",
		"America/Indiana/Indianapolis",
		"America/Indiana/Petersburg",
		"America/Jamaica",
		"America/Los_Angeles",
		"America/Martinique",
		"America/Mexico_City",
		"America/Montserrat",
		"America/New_York",
		"America/Panama",
		"America/Puerto_Rico",
		"America/Santiago",
		"America/Toronto",
		"America/Vancouver",
	],

	Asie: [
		"Asia/Bangkok",
		"Asia/Hong_Kong",
		"Asia/Jakarta",
		"Asia/Seoul",
		"Asia/Shanghai",
		"Asia/Singapore",
		"Asia/Taipei",
		"Asia/Tokyo",
	],

	Australie: ["Australia/Melbourne", "Australia/Sydney"],

	Europe: [
		"Europe/Amsterdam",
		"Europe/Berlin",
		"Europe/Brussels",
		"Europe/Budapest",
		"Europe/Dublin",
		"Europe/Helsinki",
		"Europe/Istanbul",
		"Europe/Lisbon",
		"Europe/London",
		"Europe/Luxembourg",
		"Europe/Madrid",
		"Europe/Moscow",
		"Europe/Oslo",
		"Europe/Paris",
		"Europe/Prague",
		"Europe/Rome",
		"Europe/Stockholm",
		"Europe/Vatican",
		"Europe/Vienna",
		"Europe/Zagreb",
		"Europe/Zurich",
	],
};

const userLangPhp = computed(() => {
	let server = config.available_list.find(
		(s) => state.value?.current_version === s.id
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
	<fieldset>
		<legend class="h2-like">Configuration du PHP</legend>

		<div>
			<label for="php_version"> Choisir la version de PHP </label>

			<select id="php_version" name="php_version">
				<option
					v-for="version of config.available_list"
					:key="version.id"
					:value="version.id"
					:selected="userLangPhp?.id === version.id"
				>
					{{ version.semver }}
				</option>
			</select>
		</div>

		<div>
			<label for="php_timezone">Fuseaux Horaires</label>

			<select id="php_timezone_list" name="php_timezone">
				<optgroup
					v-for="timezone_name of Object.keys(timezones)"
					:label="timezone_name"
				>
					<option
						v-for="timezone of timezones[timezone_name]"
						:value="timezone"
					>
						{{ timezone }}
					</option>
				</optgroup>
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
				:value="userLangPhp?.user_config_file"
			/>
		</div>

		<div>
			<input
				type="checkbox"
				name="php_composer"
				id="composer"
				:checked="userLangPhp?.user_composer ?? config.composer"
			/>
			&nbsp;
			<label for="composer"
				>Activer Composer (Gestionnaire de paquet)</label
			>
		</div>

		<div>
			<input
				type="checkbox"
				name="php_ext_xdebug"
				id="xdebug"
				:checked="userLangPhp?.user_xdebug ?? config.xdebug"
			/>
			&nbsp;
			<label for="xdebug">Activer l'extension XDebug</label>
		</div>
	</fieldset>
</template>
