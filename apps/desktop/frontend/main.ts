import { createApp } from "vue";

let app = createApp(
	typeof window.__TAURI_INTERNALS__ !== "undefined"
		? (await import("./src-tauri/app.vue")).default
		: (await import("./src-web/app.vue")).default,
);

app.mount("#app");
