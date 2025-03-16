<script setup lang="ts">
// ---- //
// Type //
// ---- //

export interface Config {
	available_list: Array<{
		id: number;
		name: string;
		versions: Array<{ id: number; semver: string }>;
	}>;
	webadmin: boolean;
}

export interface State {
	current_db_id: Array<number>;
	[db_id: number]: { semver: number };
}

interface Props {
	config: Config;
}

// --------- //
// Composant //
// --------- //
const { config } = defineProps<Props>();

let state = defineModel<State>();
</script>

<template>
    <fieldset>
        <legend class="h2-like">Base de données</legend>

        <div>
            <label for="dbserver">
                Choisir les bases de données à utiliser
            </label>

            <select id="dbserver" name="dbserver[]" multiple>
                <option
                    v-for="dbserver of config.available_list"
                    :key="dbserver.name"
                    :value="dbserver.id"
                    :selected="state?.current_db_id.includes(dbserver.id)"
                >
                    {{ dbserver.name }}
                </option>
            </select>
        </div>

        <div>
            <input
                type="checkbox"
                name="dbserver_webadmin"
                id="dbserver_webadmin"
                :checked="config.webadmin"
            >
            &nbsp;
            <label for="dbserver_webadmin">Interface d'administration</label>
        </div>
    </fieldset>
</template>
