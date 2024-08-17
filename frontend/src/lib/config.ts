export class AppConfig {
    page: PageConfig = new PageConfig();

    locations: Location[] = [];

    historyRecords: HistoryRecord[] = [];
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

    counterValueLabel: string = '';

    mailTemplateTitle: string = '';
    mailTemplateToLabel: string = '';
    mailTemplateSubjectLabel: string = '';
    mailTemplateBodyLabel: string = '';
    mailTemplateCloseButton: string = '';

    sendConfirmMsg: string = '';
    sendButton: string = '';

    showLetterButton: string = '';

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
    url: string = '';
    manual: string = '';
}

export class HistoryRecord {
    id: number = 0;
    location: string = '';
    accountId: string = '';
    counterName: string = '';
    month: string = '';
    year: number = 0;
    value: string = '';
    created: number = 0;
}