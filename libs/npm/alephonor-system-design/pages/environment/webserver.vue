<script setup lang="ts">
const webservers = [
	{
		id: 1,
		name: "Apache2",
	},
	{ id: 2, name: "Caddy" },
	{ id: 3, name: "NGINX" },
];
</script>

<template>
	<article class="env:webserver">
		<div class="container">
			<h1>Configuration « Serveur Web »</h1>
		</div>

		<details class="container" open>
			<summary class="h2-like">Configuration du serveur</summary>
			<div>
				<label for="web_server">
					Choisir le serveur web à utiliser
				</label>

				<select name="web_server">
					<option v-for="webserver of webservers" :key="webserver.name" :value="webserver.id">
						{{ webserver.name }}
					</option>
				</select>
			</div>
			
			<div>
				<label for="webserver_config_file">
					Emplacement du fichier de configuration serveur
				</label>

				<input type="text" id="webserver_config_file" placeholder="/etc/apache2/httpd.conf" />
			</div>

			<div>
				<input type="checkbox" name="webserver_options_indexes" id="webserver_options_indexes">
				&nbsp;
				<label for="webserver_options_indexes">
					Lister les fichiers lorsque le fichier d'index est manquant dans un répertoire
				</label>
			</div>
		</details>

		<details class="container" open>
			<summary class="h2-like">Configuration PHP</summary>
			
			<div>
				<label for="php_version">
					Choisir la version de PHP
				</label>

				<select id="php_version" name="php_version">
					<option value="1">8.4</option>
					<option value="2">8.3</option>
					<option value="3">8.2</option>
				</select>
			</div>

			<div>
				<label for="php_config_file">
					Emplacement du fichier de configuration de PHP
				</label>
				<input type="text" id="php_config_file" placeholder="/etc/php/php.ini" />
			</div>

			<div>
				<input type="checkbox" name="php_composer" id="composer">
				&nbsp;
				<label for="composer">Activer Composer (Gestionnaire de paquet)</label>
			</div>

			<div>
				<input type="checkbox" name="php_ext_xdebug" id="xdebug">
				&nbsp;
				<label for="xdebug">Activer l'extension XDebug</label>
			</div>
		</details>


		<details class="container" open>
			<summary class="h2-like">Base de données</summary>

			<label for="db_server">
				Choisir la/les base(s) de données à utiliser
			</label>

			<select id="db_server" name="db_server[]" multiple>
				<option value="1">MariaDB</option>
				<option value="2">MySQL</option>
				<option value="3">PostgreSQL</option>
			</select>
		</details>

		<div class="container">
			<button type="submit">Sauvegarder</button>
		</div>
	</article>
</template>

<style>
.dashboard\:screen\:content:has(.env\:webserver) {
	background-image: url(./amp.png);
	background-origin: border-box;
	background-position: center;
	background-size: cover;
}

.env\:webserver {
	> * + * {
		margin-top: .5rem;
	}
}
</style>

<style scoped>
.container {
	max-width: 80ch;
	margin-inline: auto;
}

details summary {
	outline: none !important;
}

select, input[type="text"] {
	width: 100%;
	padding: .5rem;
	border-radius: 6px;
	background: var(--color-black);
	color: var(--color-white);
	appearance: none;
}
select option {
	color: var(--color-white);
}

button[type="submit"] {
	padding: .5rem;
	border-radius: 6px;
	color: var(--color-white);
	background: var(--color-black);
}
</style>
