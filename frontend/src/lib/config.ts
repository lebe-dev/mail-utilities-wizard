export class AppConfig {
    page: PageConfig = new PageConfig();

    locations: Location[] = [];
}

export class PageConfig {
    title: string = '';
    header: string = '';

    selectLocationText: string = '';
}

export class Location {
    name: string = '';
    counters: Counter[] = [];
}

export class Counter {
    name: string = '';
    accountId: string = '';
    email: string = '';
}