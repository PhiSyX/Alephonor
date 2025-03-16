<script setup lang="ts">
import { appLocalDataDir, join } from "@tauri-apps/api/path";
import { reactive } from "vue";

import Webserver from "@alephonor/system-design/pages/environment/webserver.vue";

// ex data from db
const data_config = {
	db: {
		available_list: [
			{
				id: 1,
				name: "MariaDB",
				versions: [
					{ id: 101, semver: "^11.7" },
					{ id: 100, semver: "^10.11" },
				],
			},
			{
				id: 2,
				name: "MySQL",
				versions: [
					{ id: 202, semver: "^9.2" },
					{ id: 201, semver: "^8.4" },
					{ id: 200, semver: "^8.0" },
				],
			},
			{
				id: 3,
				name: "PostgreSQL",
				versions: [
					{ id: 302, semver: "^17.4" },
					{ id: 301, semver: "^16.8" },
					{ id: 300, semver: "^15.12" },
				],
			},
		],
		webadmin: true,
	},

	http: {
		available_list: [
			{
				id: 1,
				name: "Apache2 HTTPD",
				rel_config_file: "apache2/httpd.conf",
				versions: [{ id: 400, semver: "^2.4" }],
			},

			{
				id: 2,
				name: "Caddy",
				rel_config_file: "caddy/Caddyfile",
				versions: [{ id: 410, semver: "^2.9" }],
			},

			{
				id: 3,
				name: "NGINX",
				rel_config_file: "nginx/nginx.conf",
				versions: [
					{ id: 421, semver: "^1.27" },
					{ id: 420, semver: "^1.26" },
				],
			},
		],
		rel_working_dir: "www",
		autoindex: true,
	},

	php: {
		available_list: [
			{ id: 502, semver: "^8.4" },
			{ id: 501, semver: "^8.3" },
			{ id: 500, semver: "^8.2" },
		],
		default_config_file: "/etc/php/php.ini",
		composer: true,
		xdebug: true,
	},
};

const config = {
	db: data_config.db,
	http: {
		...data_config.http,
		available_list: await Promise.all(
			data_config.http.available_list.map(async (m) => {
				return {
					...m,
					default_config_file: await join(
						await appLocalDataDir(),
						"etc",
						m.rel_config_file,
					),
					default_working_dir: await join(
						await appLocalDataDir(),
						data_config.http.rel_working_dir,
					),
				};
			}),
		),
	},
	php: data_config.php,
};

const state = reactive({
	db: {
		current_db_id: [3],
		3: { semver: 302 },
	},
	http: {
		current_server_id: 2,
		2: {
			semver: 410,
			config_file: "~/Developer/infra/Caddy/Caddyfile",
			working_dir: "~/Developer/www",
		},
	},
	php: {
		current_version: 502,
		502: {
			config_file: "~/Developer/infra/php/php.ini",
		},
	},
});
</script>

<template>
	<Webserver :config v-model="state" />
</template>

