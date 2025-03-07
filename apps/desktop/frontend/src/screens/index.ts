export enum Screen {
	Applications = "Applications",
	SignIn = "SignIn",
}

export interface ScreenEmits {
	// biome-ignore lint/style/useShorthandFunctionType: .-)
	(event_name: "change-screen", s: Screen): void;
}

export const emitChangeScreen = ($emit: ScreenEmits) => (screen: Screen) =>
	$emit("change-screen", screen);
