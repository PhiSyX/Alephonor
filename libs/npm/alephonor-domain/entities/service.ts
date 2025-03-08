export interface Service {
	name: string;
	title: string;
	image?: string;
	installed: boolean;
	commands: ServiceCommands;
}

export interface ServiceCommands {
	install: string;
	check: string;
	start: string;
	stop: string;
}
