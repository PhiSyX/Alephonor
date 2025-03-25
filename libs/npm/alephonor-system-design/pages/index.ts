import type { Page } from "@alephonor/domain/pages/enum";

export interface PageEmits {
	(evtName: "change-page", p: Page): void;
}
export const emitChangePage = ($emit: PageEmits) => (page: Page) =>
	$emit("change-page", page);
