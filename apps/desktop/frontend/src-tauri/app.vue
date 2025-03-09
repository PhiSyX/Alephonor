<script setup lang="ts">
import { defineAsyncComponent, shallowRef } from "vue";
import { Screen } from "@alephonor/domain/screens/enum";

import AppWinHeaderArea from "./components/appwin-header-area.vue";

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
		<AppWinHeaderArea data-tauri-drag-region />
	</Suspense>

	<main role="main">
		<component :is="currentScreen" @change-screen="changeScreen" />
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
