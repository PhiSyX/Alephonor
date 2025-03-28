<script setup lang="ts">
import { computed, ref } from "vue";

import type { AccountInfo } from "@alephonor/domain/entities/account";
import type { PageEmits } from "./index";

import { Page } from "@alephonor/domain/pages/enum";

import Input from "../components/input.vue";

interface Props {
	accountInfo: Partial<AccountInfo>;
	onSubmit(rawPassword: string): Promise<boolean>;
}

interface Emits extends PageEmits {}

const { accountInfo, onSubmit } = defineProps<Props>();
const emits = defineEmits<Emits>();

/** Compte */
let passwordModel = ref("");
let passwordInvalid = ref(false);

/** Couleur aléatoire */
let colors = [
	"red",
	"pink",
	"purple",
	"indigo",
	"blue",
	"light-blue",
	"cyan",
	"teal",
	"green",
	"yellow",
	"amber",
	"orange",
	"blue-grey",
];
let colorsFgs = {
	amber: "black",
	orange: "amber-a100",
	yellow: "black",
};

let colorsBgs = {
	red: [700, 900],
	pink: [400, 700],
	purple: [700, 800],
	indigo: [500, 700],
	"light-blue": [700, 800],
	teal: [700, 500],
	yellow: [600, 800],
	amber: [500, 700],
};
let variantName = computed(
	() =>
		accountInfo.preferred_color ||
		colors[Math.floor(Math.random() * colors.length)]
);
let randomColor = computed(() =>
	fgcolor(variantName.value as keyof typeof colorsFgs, 50)
);
let randomBgColor = computed(() =>
	bgcolor(variantName.value as keyof typeof colorsBgs, 900)
);
let randomBgColorAlt = computed(() =>
	altcolor(variantName.value as keyof typeof colorsBgs, 700)
);

function bgcolor(name: keyof typeof colorsBgs, def: number) {
	return `hsl(var(--color-${name}-${colorsBgs[name]?.[0] || def}))`;
}

function fgcolor(name: keyof typeof colorsFgs, def: number) {
	if (colorsFgs[name] && typeof colorsFgs[name] === "string") {
		return `hsl(var(--color-${colorsFgs[name]}))`;
	}
	return `hsl(var(--color-${name}-${colorsFgs[name]?.[0] || def}))`;
}

function altcolor(name: keyof typeof colorsBgs, def: number) {
	if (colorsBgs[name] && typeof colorsBgs[name] === "string") {
		return `hsl(var(--color-${colorsBgs[name]}))`;
	}
	return `hsl(var(--color-${name}-${colorsBgs[name]?.[1] || def}))`;
}

async function post_account_form() {
	let success = await onSubmit(passwordModel.value);

	passwordModel.value = "";

	if (success && Page) {
		emits("change-page", Page.Dashboard);
	}

	passwordInvalid.value = !success;
	setTimeout(() => {
		passwordInvalid.value = false;
	}, 60);
}
</script>

<template>
	<section class="app:signin auth:signin">
		<h1 class="auth:title">
			Bonjour
			<small>{{ accountInfo.fullname }}</small
			>, content de te revoir
		</h1>

		<div class="auth:avatar">
			<img
				v-if="accountInfo.avatar"
				:src="accountInfo.avatar"
				alt="User Avatar"
			/>
		</div>

		<form
			action=""
			method="POST"
			class="auth:form"
			@submit.prevent="post_account_form"
		>
			<div
				class="input:group"
				:class="{
					'auth:input--error': passwordInvalid,
				}"
			>
				<Input
					v-model="passwordModel"
					name="password"
					icon="password"
					root-class="auth-container"
					class="bg-gradient-l"
					aria-label="Mot de passe"
					placeholder="Entre le mot de passe de ton compte local"
					type="password"
				>
					<button
						type="submit"
						name="signin-user"
						class="bg-gradient-r"
					>
						Ok
					</button>
				</Input>
			</div>

			<hr text="ou" />
			<a href="#">Choisir un compte en ligne</a>
		</form>
	</section>
</template>

<style lang="scss">
@use "../assets/pages/signin-page.root";
</style>

<style lang="scss">
// NOTE: la syntaxe "#styles/..." ne fonctionne pas avec scss...
// NOTE: comment appliquer un layer avec scss ?
@use "../assets/pages/signin-page.inherit" with (
	$input-surface: v-bind(randomBgColor),
	$input-surface-alt: v-bind(randomBgColorAlt),
	$input-on-surface: v-bind(randomColor)
);
</style>

<style scoped lang="scss">
// NOTE: la syntaxe "#styles/..." ne fonctionne pas avec scss...
// NOTE: comment appliquer un layer avec scss ?
@use "../assets/pages/signin-page" with (
	$auth-title-small-surface: v-bind(randomBgColor),
	$auth-title-small-on-surface: v-bind(randomColor)
);
</style>
