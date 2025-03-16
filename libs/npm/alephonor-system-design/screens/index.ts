import type { Screen } from "@alephonor/domain/screens/enum";

export interface ScreenEmits {
	(evtName: "change-screen", s: Screen): void;
}
export const emitChangeScreen = ($emit: ScreenEmits) => (screen: Screen) =>
	$emit("change-screen", screen);
