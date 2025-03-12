<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";

import type { AccountInfo } from "@alephonor/domain/entities/account";
import type { Screen } from "@alephonor/domain/screens/enum";
import SigninScreen from "@alephonor/system-design/screens/signin-screen.vue";

const router = useRouter();

let accountInfo = ref<Partial<AccountInfo>>({
	avatar: "/phisyx.png",
	name: "phisyx",
	fullname: "Mike 'PhiSyX' S."
});

function changeScreen(s: Screen) {
	router.push({ name: s });
}

async function handleSubmit(_rawPassword: string): Promise<boolean> {
	return true;
}
</script>

<template>
	<SigninScreen
		:account-info="accountInfo"
		@submit="handleSubmit"
		@change-screen="changeScreen"
	/>
</template>
