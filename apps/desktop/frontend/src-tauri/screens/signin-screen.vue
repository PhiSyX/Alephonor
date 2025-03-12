<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";

import type { AccountInfo } from "@alephonor/domain/entities/account";
import type { Screen } from "@alephonor/domain/screens/enum";
import SigninScreen from "@alephonor/system-design/screens/signin-screen.vue";

const router = useRouter();

let accountInfo = ref<Partial<AccountInfo>>({});

onMounted(async () => {
	accountInfo.value = await invoke<AccountInfo>("account_info");
});

function changeScreen(s: Screen) {
	router.push({ name: s });
}

function handleSubmit(rawPassword: string): Promise<boolean> {
	return invoke<boolean>("post_account_form", { rawPassword });
}
</script>

<template>
	<SigninScreen
		:account-info="accountInfo"
		@submit="handleSubmit"
		@change-screen="changeScreen"
	/>
</template>
