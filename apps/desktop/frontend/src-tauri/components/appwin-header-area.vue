<script setup lang="ts">
import AppWinControlsArea from "@alephonor/system-design/appwin/header-area.vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { type as tauriOsType } from "@tauri-apps/plugin-os";

const win = getCurrentWindow();
const winTitle = await win.title();
const osType = tauriOsType();

const direction = osType === "macos" ? "rtl" : "auto";

function toggleTheme(): void {
	// TODO : Oui oui un todo et alors ;-)
}
</script>

<template>
	<AppWinControlsArea
		:title="winTitle"
		:dir="direction"
		@close="win.close()"
		@drag="win.startDragging()"
		@maximize="win.toggleMaximize()"
		@minimize="win.minimize()"
		@toggle-theme="toggleTheme"
	/>
</template>
