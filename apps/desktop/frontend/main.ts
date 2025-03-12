import { createApp } from "vue";

let app = createApp(
	typeof window.__TAURI_INTERNALS__ !== "undefined"
		? (await import("./src-tauri/app.vue")).default
		: (await import("./src-web/app.vue")).default,
);

// 1. Setup plugins
const plugins = import.meta.glob<{ install: VuePluginInstall }>(
	"./plugins/*.ts",
	{ eager: true },
);

for (let plugin of Object.values(plugins)) {
	plugin.install(app);
}

app.mount("#app");
