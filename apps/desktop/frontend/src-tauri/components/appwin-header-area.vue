<script setup lang="ts">
import { Screen } from "@alephonor/domain/screens/enum";
import AppWinControlsArea from "@alephonor/system-design/appwin/header-area.vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { type as tauriOsType } from "@tauri-apps/plugin-os";
import { computed, onUpdated, onMounted, shallowRef, watch } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();
const win = getCurrentWindow();
const winTitle = shallowRef("");
const osType = tauriOsType();

const direction = osType === "macos" ? "rtl" : "auto";

const showTitle = computed(() => route.name !== Screen.SignIn);
const showSearchbar = computed(() => route.name !== Screen.SignIn);

watch(
	() => route.meta.title as string,
	async (title) => {
		let nTitle = title;

		if (title?.length === 0) {
			nTitle = await win.title();
		}

		winTitle.value = nTitle;
	},
	{ immediate: true }
);

function toggleTheme(): void {
	// TODO : Oui oui un todo et alors ;-)
}
</script>

<template>
	<AppWinControlsArea
		:title="winTitle"
		:show-title="showTitle"
		:show-searchbar="showSearchbar"
		:dir="direction"
		@close="win.close()"
		@drag="win.startDragging()"
		@maximize="win.toggleMaximize()"
		@minimize="win.minimize()"
		@toggle-theme="toggleTheme"
	/>
</template>
