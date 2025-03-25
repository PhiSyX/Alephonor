import type { RouteRecordRaw } from "vue-router";

import { Page } from "@alephonor/domain/pages/enum";

// -------- //
// Constant //
// -------- //

export const ROUTES: Array<RouteRecordRaw> = [
	{
		path: "/",
		component: () => import("../pages/signin-page.vue"),
		strict: true,
		name: Page.SignIn,
	},
	{
		path: "/dashboard",
		component: () => import("../pages/dashboard-page.vue"),
		strict: true,
		name: Page.Dashboard,
	},
];
