<script setup lang="ts">
import AppWinHeaderArea from "./components/appwin-header-area.vue";

if (import.meta.env.PROD) {
	document.addEventListener("contextmenu", (event) => event.preventDefault());
}
</script>

<template>
	<Suspense>
		<AppWinHeaderArea data-tauri-drag-region />
	</Suspense>

	<main role="main">
		<RouterView />
	</main>
</template>

<style lang="scss">
@import "../assets/styles/main.css";

#app,
main[role="main"] {
	display: flex;
	flex-direction: column;
}
#app {
	height: var(--size-full);
}
main[role="main"] {
	flex-grow: 1;
}
</style>
