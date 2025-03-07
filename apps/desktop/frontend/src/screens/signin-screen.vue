<script setup lang="ts">
import { computed, ref } from "vue";

import Input from "#components/input.vue";
import { Screen } from "#screens";

export interface AccountInfo {
	avatar: string;
	name: string;
	fullname: string;
	preferred_color: string;
}

interface Props {
	accountInfo: Partial<AccountInfo>;
	onSubmit(rawPassword: string): Promise<boolean>;
}

interface Emits {
	// biome-ignore lint/style/useShorthandFunctionType: ;-)
	(event_name: "change-screen", s: Screen): void;
}

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
	return `var(--color-${name}-${colorsBgs[name]?.[0] || def})`;
}

function fgcolor(name: keyof typeof colorsFgs, def: number) {
	if (colorsFgs[name] && typeof colorsFgs[name] === "string") {
		return `var(--color-${colorsFgs[name]})`;
	}
	return `var(--color-${name}-${colorsFgs[name]?.[0] || def})`;
}

function altcolor(name: keyof typeof colorsBgs, def: number) {
	if (colorsBgs[name] && typeof colorsBgs[name] === "string") {
		return `var(--color-${colorsBgs[name]})`;
	}
	return `var(--color-${name}-${colorsBgs[name]?.[1] || def})`;
}

async function post_account_form() {
	let success = await onSubmit(passwordModel.value);

	passwordModel.value = "";

	if (success) {
		emits("change-screen", Screen.Applications);
	}

	passwordInvalid.value = !success;
	setTimeout(() => {
		passwordInvalid.value = false;
	}, 60);
}
</script>

<template>
	<section class="auth-signin">
		<h1 class="auth-title">
			Bonjour
			<small>{{ accountInfo.fullname }}</small
			>, content de te revoir
		</h1>

		<div class="auth-avatar">
			<img
				v-if="accountInfo.avatar"
				:src="accountInfo.avatar"
				alt="User Avatar"
			/>
		</div>

		<form
			action=""
			method="POST"
			class="auth-form"
			@submit.prevent="post_account_form"
		>
			<div
				class="input-group"
				:class="{
					'auth-input-error': passwordInvalid,
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
					required
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

<style>
:root {
	--auth-signin-surface: var(--color-black);
	--auth-signin-on-surface: var(--color-white);
	--auth-signin-signin-on-surface: #d4d4d4;
	--auth-signin-signin-note-on-surface: #999fd1;
	--auth-signin-link: var(--color-red);
}
</style>

<style scoped>
.auth-signin {
	max-width: 70ch;
	margin-inline: auto;
	padding: var(--spacing-8) var(--spacing-10);
	background: var(--auth-signin-surface);
	color: var(--auth-signin-on-surface);

	& > * + * {
		margin-top: var(--spacing-3);
	}
}

.auth-title {
	font-size: 24px;

	small {
		padding: 4px;
		border-radius: 8px;
		background: v-bind(randomBgColor);
		color: v-bind(randomColor);
	}
}

.auth-avatar {
	max-width: 150px;
	max-height: 150px;
	margin-inline: auto;
	& img {
		display: inline-block;
		width: 100%;
		max-height: inherit;
		border-radius: var(--radius-full);
	}
}

.auth-signin > p {
	color: var(--auth-signin-signin-on-surface);
}

.auth-note {
	color: var(--auth-signin-signin-note-on-surface) !important;
}

form {
	display: grid;
	gap: var(--spacing-2);
}

.auth-input-error {
	animation: 250ms shake-anim alternate;
}

@keyframes shake-anim {
	0% {
		transform: translateX(-2%);
		margin-left: 0rem;
	}
	25% {
		margin-left: 0.5rem;
	}
	75% {
		margin-left: -0.5rem;
	}
	100% {
		transform: translateX(2%);
		margin-left: 0rem;
	}
}

button[type="submit"] {
	width: max-content;
	padding-block: var(--spacing-1);
	padding-inline: var(--spacing-5);
	border: none;
	border-radius: var(--radius-m);
}

hr {
	--hr-bordered: #3747b6;
	--hr-surface: var(--auth-signin-surface);
	--hr-on-surface: var(--auth-signin-on-surface);

	width: var(--size-full);
}
a {
	color: var(--auth-signin-link);
	text-align: center;
}

.input-group {
	display: grid;
	gap: 1px;
	overflow: clip;
	border-radius: var(--radius-m);
}
</style>

<style>
@import "../../assets/styles/components/form.css";

.auth-container {
	--input-surface: v-bind(randomBgColor);
	--input-surface-alt: v-bind(randomBgColorAlt);
	--input-on-surface: v-bind(randomColor);
	align-items: center;
}

.bg-gradient-l {
	background: linear-gradient(
		to left,
		var(--input-surface) 70%,
		var(--input-surface-alt)
	);
}
.bg-gradient-r {
	background: linear-gradient(
		to right,
		var(--input-surface),
		var(--input-surface-alt)
	);
}
</style>
