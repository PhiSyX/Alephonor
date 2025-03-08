<script setup lang="ts">
import { ref } from "vue";

import type { AccountInfo } from "@alephonor/domain/entities/account";
import { emitChangeScreen, type ScreenEmits } from "@alephonor/system-design/screens";
import SigninScreen from "@alephonor/system-design/screens/signin-screen.vue";

interface Emits extends ScreenEmits {}

defineEmits<Emits>();

let accountInfo = ref<Partial<AccountInfo>>({
	avatar: "/phisyx.png",
	name: "phisyx",
	fullname: "Mike 'PhiSyX' S."
});

async function handleSubmit(_rawPassword: string): Promise<boolean> {
	return true;
}
</script>

<template>
	<SigninScreen
		:account-info="accountInfo"
		@submit="handleSubmit"
		@change-screen="(s) => emitChangeScreen($emit)(s)"
	/>
</template>
