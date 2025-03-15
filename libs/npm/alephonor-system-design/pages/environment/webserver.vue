<template>
	<article class="env:webserver">
		<h1>Configuration « Serveur Web »</h1>

		<!-- <details open>
			<summary>Serveur</summary>

			<div >
				<select name="web_server">
					<option value="1">NGINX</option>
					<option value="2">Apache2</option>
					<option value="3">Caddy</option>
				</select>
			</div>
		</details>

		<details open>
			<summary>PHP</summary>

			<form action="" method="post">
				<div>
					<label for="php_version">Version</label>
					<select id="php_version" name="php_version">
						<option value="1">8.4</option>
						<option value="2">8.3</option>
						<option value="3">8.2</option>
					</select>
				</div>

				<div>
					<label for="php_config_file"
						>Fichier de configuration</label
					>
					<input type="text" id="php_config_file" />
				</div>
			</form>
		</details>

		<details open>
			<summary>Base de données</summary>

			<form action="" method="post">
				<div>
					<label for="db_server">SGBD</label>
					<select id="db_server" name="db_server">
						<option value="1">MariaDB</option>
						<option value="2">MySQL</option>
						<option value="3">PostgreSQL</option>
					</select>
				</div>
			</form>
		</details> -->
	</article>
</template>

<style>
.dashboard\:screen\:content:has(.env\:webserver) {
	background-image: url(./amp.png);
	background-origin: border-box;
	background-position: center;
	background-size: cover;
}
</style>

<style scoped>
details summary {
	outline: none !important;
}

select, input {
	background: var(--color-deep-purple-100);
	appearance: none;
	padding: 1rem;
	width: 100%;
}
</style>
