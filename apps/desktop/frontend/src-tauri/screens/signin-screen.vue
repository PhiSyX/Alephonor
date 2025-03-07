<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";

import { emitChangeScreen, type ScreenEmits } from "#screens";

import SigninScreen, { type AccountInfo } from "#screens/signin-screen.vue";

interface Emits extends ScreenEmits {}

defineEmits<Emits>();

let accountInfo = ref<Partial<AccountInfo>>({});

onMounted(async () => {
	accountInfo.value = await invoke<AccountInfo>("account_info");
});

function handleSubmit(rawPassword: string): Promise<boolean> {
	return invoke<boolean>("post_account_form", { rawPassword });
}
</script>

<template>
	<SigninScreen
		:account-info="accountInfo"
		@submit="handleSubmit"
		@change-screen="(s) => emitChangeScreen($emit)(s)"
	/>
</template>
