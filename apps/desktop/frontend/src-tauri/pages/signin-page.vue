<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";

import type { AccountInfo } from "@alephonor/domain/entities/account";
import type { Page } from "@alephonor/domain/pages/enum";
import SigninPage from "@alephonor/system-design/pages/signin-page.vue";

const router = useRouter();

let accountInfo = ref<Partial<AccountInfo>>({});

onMounted(async () => {
	accountInfo.value = await invoke<AccountInfo>("account_info");
});

function changePage(s: Page) {
	router.push({ name: s });
}

function handleSubmit(rawPassword: string): Promise<boolean> {
	return invoke<boolean>("post_account_form", { rawPassword });
}
</script>

<template>
	<SigninPage
		:account-info="accountInfo"
		@submit="handleSubmit"
		@change-page="changePage"
	/>
</template>
