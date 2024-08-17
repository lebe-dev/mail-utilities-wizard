export class AppConfig {
    locale: LocaleConfig = new LocaleConfig();

    locations: Location[] = [];

    historyRecords: HistoryRecord[] = [];
}

export class LocaleConfig {
    // COMMON
    language: string = '';
    title: string = '';
    header: string = '';

    appErrorMsg: string = '';

    // MAIN PAGE

    historyTableText: string = '';
    historyRecordDate: string = '';
    historyRecordValues: string = '';
    historyRecordCounter: string = '';
    historyRecordLocation: string = '';
    historyRecordPeriod: string = '';
    historyRecordValue: string = '';

    // SEND PAGE

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

    alreadySentWarnMsg: string = '';

    sendConfirmMsg: string = '';
    sendButton: string = '';

    showLetterButton: string = '';
    backButton: string = '';

    sendingMsg: string = '';
    sendErrorMsg: string = '';

    // SEND SUCCESS PAGE

    sendSuccessMsg: string = '';
    sendMoreButton: string = '';
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