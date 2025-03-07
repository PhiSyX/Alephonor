<script setup lang="ts">
import { defineAsyncComponent, ref } from "vue";

import { Screen } from "../src/screens";

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

let currentScreen = ref(Screen.SignIn);

function changeScreen(s: Screen): void {
	currentScreen.value = s;
}
</script>

<template>
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
