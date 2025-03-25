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
	// Dashboard
	{
		path: "/dashboard",
		component: () => import("../pages/dashboard-page.vue"),
		strict: true,
		meta: {
			title: "Dashboard",
		},
		children: [
			{
				path: "",
				component: () =>
					import(
						"@alephonor/system-design/pages/dashboard/resume-page.vue"
					),
				strict: true,
				name: Page.Dashboard,
			},
		],
	},
	// Env
	{
		path: "/env",
		component: () => import("../pages/dashboard-page.vue"),
		children: [
			{
				path: "webserver",
				component: () =>
					import("../pages/environment/webserver-page.vue"),
				meta: {
					title: "Environnement « Serveur Web »",
				},
			},
		],
	},
];
