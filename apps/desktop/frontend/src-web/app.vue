<script setup lang="ts">
import { defineAsyncComponent, shallowRef } from "vue";
import { Screen } from "@alephonor/domain/screens/enum";

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
	<main role="main">
		<component :is="currentScreen" @change-screen="changeScreen" />
	</main>
</template>

<style lang="scss">
@import "../assets/styles/main.css";
</style>
