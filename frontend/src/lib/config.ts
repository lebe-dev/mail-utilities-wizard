export class AppConfig {
    page: PageConfig = new PageConfig();

    locations: Location[] = [];
}

export class PageConfig {
    title: string = '';
    header: string = '';

    locale: string = '';

    selectLocationLabel: string = '';
    selectLocationDropdown: string = '';
    selectCounterLabel: string = '';
    selectCounterDropdown: string = '';
    accountIdLabel: string = '';
    accountIdHint: string = '';
    emailLabel: string = '';
    periodLabel: string = '';
    customPeriodLabel: string = '';
    counterValueLabel: string = '';
    counterDefaultValue: string = '';
    sendConfirmMsg: string = '';
    sendConfirmYes: string = '';
    sendConfirmNo: string = '';
    sendButton: string = '';
    sendMoreButton: string = '';
    appErrorMsg: string = '';
    sendingMsg: string = '';
    sendSuccessMsg: string = '';
    sendErrorMsg: string = '';
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