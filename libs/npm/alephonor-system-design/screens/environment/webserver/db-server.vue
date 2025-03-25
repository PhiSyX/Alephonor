<script setup lang="ts">
import IconMariadb from "../../../components/icons/icon-mariadb.vue";
import IconMysql from "../../../components/icons/icon-mysql.vue";
import IconPostgresql from "../../../components/icons/icon-postgresql.vue";

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
		<legend class="h2-like">Configuration de la base de données</legend>

		<div>
			<label for="dbserver">
				Choisir les bases de données à utiliser
			</label>

			<div class="dbserver-select">
				<button type="button" title="MariaDB">
					<IconMariadb />
				</button>

				<button type="button" title="MySQL">
					<IconMysql />
				</button>

				<button type="button" title="PostgreSQL">
					<IconPostgresql />
				</button>
			</div>
		</div>

		<div>
			<label for="db_user">Changer l'utilisateur « root »</label>
			<input type="text" name="db_user" id="db_user" placeholder="root" />
		</div>

		<div>
			<label for="db_pass"
				>Changer le mot de passe de l'utilisateur « root »</label
			>
			<input
				type="password"
				name="db_pass"
				id="db_pass"
				placeholder="root"
			/>
		</div>

		<div>
			<input
				type="checkbox"
				name="dbserver_webadmin"
				id="dbserver_webadmin"
				:checked="config.webadmin"
			/>
			&nbsp;
			<label for="dbserver_webadmin">Interface d'administration</label>
		</div>
	</fieldset>
</template>

<style lang="scss" scoped>
@use "@alephonor/sheets/abstracts/functions" as fn;

.dbserver-select {
	display: flex;
	justify-content: space-evenly;
	height: fn.space(10);
	margin-block: fn.space(3);
}

.dbserver-select button {
	width: fn.space(10);
	border: 3px solid transparent;
	border-radius: fn.radius(md);
	background: transparent;
	transition: border-color 250ms ease-in-out;

	&,
	& * {
		cursor: pointer;
	}

	&:hover {
		border-color: fn.color(orange, 200);
	}
}
</style>
