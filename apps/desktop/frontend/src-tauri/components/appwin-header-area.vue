<script setup lang="ts">
import { Page } from "@alephonor/domain/pages/enum";
import AppWinControlsArea from "@alephonor/system-design/appwin/header-area.vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { type as tauriOsType } from "@tauri-apps/plugin-os";
import { computed, shallowRef, watch } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();
const win = getCurrentWindow();
const winTitle = shallowRef("");
const osType = tauriOsType();

const direction = osType === "macos" ? "rtl" : "auto";

const showTitle = computed(() => route.name !== Page.SignIn);
const showSearchbar = computed(() => route.name !== Page.SignIn);

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

function toggleTheme() {
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
