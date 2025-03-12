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
	{
		path: "/dashboard",
		component: () => import("../screens/dashboard-screen.vue"),
		strict: true,
		name: Screen.Dashboard,
		meta: {
			title: "Dashboard",
		},
	},
];
