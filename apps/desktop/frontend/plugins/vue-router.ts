import type { App } from "vue";

import { createMemoryHistory, createRouter } from "vue-router";

import { ROUTES as TAURI_ROUTES } from "../src-tauri/routes";
import { ROUTES as WEB_ROUTES } from "../src-web/routes";

let routes = window.__TAURI_INTERNALS__ ? TAURI_ROUTES : WEB_ROUTES;

/**
 * Installe le plugin vue-router.
 */
export function install(app: App<Element>) {
	app.use(
		createRouter({
			history: createMemoryHistory(),
			routes,
		}),
	);
}
