<script setup lang="ts">
if (import.meta.env.PROD) {
	document.addEventListener("contextmenu", (event) => event.preventDefault());
}
</script>

<template>
	<main role="main">
		<RouterView />
	</main>
</template>

<style lang="scss">
@use "../assets/styles/main";
</style>
