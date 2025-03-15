import type { RouteRecordRaw } from "vue-router";

import { Screen } from "@alephonor/domain/screens/enum";

// -------- //
// Constant //
// -------- //

export const ROUTES: Array<RouteRecordRaw> = [
	{
		path: "/",
		component: () => import("../screens/signin-screen.vue"),
		strict: true,
		name: Screen.SignIn,
	},
	// Dashboard
	{
		path: "/dashboard",
		component: () => import("../screens/dashboard-screen.vue"),
		strict: true,
		meta: {
			title: "Dashboard",
		},
		children: [
			{
				path: "",
				component: () =>
					import("@alephonor/system-design/dashboard/resume.vue"),
				strict: true,
				name: Screen.Dashboard,
			},
		],
	},
	// Env
	{
		path: "/env",
		component: () => import("../screens/dashboard-screen.vue"),
		children: [
			{
				path: "webserver",
				component: () =>
					import(
						"@alephonor/system-design/pages/environment/webserver.vue"
					),
			},
		],
	},
];
