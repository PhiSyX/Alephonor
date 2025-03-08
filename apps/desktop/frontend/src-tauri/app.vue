<script setup lang="ts">
import { defineAsyncComponent, shallowRef } from "vue";

import AppWinHeaderArea from "./components/appwin-header-area.vue";

import { Screen } from "#screens";

defineOptions({
	components: {
		[Screen.Applications]: defineAsyncComponent(
			() => import("./screens/application-screen.vue")
		),
		[Screen.SignIn]: defineAsyncComponent(
			() => import("./screens/signin-screen.vue")
		),
	},
});

let currentScreen = shallowRef(Screen.SignIn);

function changeScreen(s: Screen): void {
	currentScreen.value = s;
}

if (import.meta.env.PROD) {
	document.addEventListener("contextmenu", (event) => event.preventDefault());
}
</script>

<template>
	<Suspense>
		<AppWinHeaderArea class="l-app-win-header" data-tauri-drag-region />
	</Suspense>

	<main role="main">
		<component :is="currentScreen" @change-screen="changeScreen" />
	</main>
</template>

<style lang="scss">
@import "../assets/styles/main.css";

main[role="main"] {
	max-width: 75dvw;
	margin-inline: auto;
	margin-block: 3rem;
}
</style>
