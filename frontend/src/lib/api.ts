import {type AppConfig, LocaleConfig} from "./config";

export async function doLogin(password: string): Promise<string> {
    const request = {
        password: password
    };

    console.log('login attempt..');

    const response = await fetch('/api/login', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        credentials: 'include',
        body: JSON.stringify(request)
    });

    if (response.status === 200) {
        return response.text()

    } else {
        throw new Error('login error')
    }
}

export async function fetchAppConfig(): Promise<AppConfig> {
    const response = await fetch('/api/config', {
        method: 'GET',
        credentials: 'include',
    });

    if (response.status === 200) {
        return response.json()

    } else if (response.status === 401) {
        location.href = '/login';
        throw new Error('auth required')

    } else {
        throw new Error('config load error')
    }
}

export async function fetchLocaleConfig(): Promise<LocaleConfig> {
    const response = await fetch('/api/config/locale', {
        method: 'GET',
    });

    if (response.status === 200) {
        return response.json()

    } else {
        throw new Error('config load error')
    }
}

export async function fetchMailTemplate(locationName: string, counterName: string, year: number,
                                        month: string, counterValue: string): Promise<MailTemplateResponse> {
    const counterData = new CounterData(locationName, counterName, year, month, counterValue);

    const response = await fetch('/api/mail/template', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        credentials: 'include',
        body: JSON.stringify(counterData)
    });

    if (response.status === 200) {
        return response.json()

    } else if (response.status === 401) {
        location.href = '/login';
        throw new Error('auth required')

    } else {
        throw new Error('mail template fetch error')
    }
}

class CounterDataCheckRequest {
    constructor(email: string, accountId: string, year: string, month: string) {
        this.email = email;
        this.accountId = accountId;
        this.year = parseInt(year, 0);
        this.month = month;
    }

    email: string;
    accountId: string;
    year: number;
    month: string;
}

export class CounterDataAlreadySent {
    exist: boolean = false;
}

export async function checkIfCounterDataAlreadySent(email: string, accountId: string,
                                                    year: string, month: string): Promise<CounterDataAlreadySent> {
    const counterData = new CounterDataCheckRequest(email, accountId, year, month);

    console.log('check request:', counterData);

    const response = await fetch('/api/counter/check', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        credentials: 'include',
        body: JSON.stringify(counterData)
    });

    if (response.status === 200) {
        return response.json()

    } else if (response.status === 401) {
        location.href = '/login';
        throw new Error('auth required')

    } else {
        throw new Error('mail template fetch error')
    }
}

export async function sendCounterData(locationName: string, counterName: string, year: number,
                                      month: string, counterValue: string): Promise<string> {
    const report = new CounterData(locationName, counterName, year, month, counterValue);

    const response = await fetch('/api/counter', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        credentials: 'include',
        body: JSON.stringify(report)
    });

    if (response.status === 200) {
        return response.statusText

    } else if (response.status === 401) {
        location.href = '/login';
        throw new Error('auth required')

    } else {
        throw new Error('config load error')
    }
}

export class MailTemplateResponse {
    subject: string = '';
    body: string = '';
}

export class CounterData {
    constructor(locationName: string, counterName: string, year: number, month: string, counterValue: string) {
        this.locationName = locationName;
        this.counterName = counterName;
        this.year = year;
        this.month = month;
        this.counterValue = counterValue;
    }
    locationName: string;
    counterName: string;
    year: number;
    month: string;
    counterValue: string;
}