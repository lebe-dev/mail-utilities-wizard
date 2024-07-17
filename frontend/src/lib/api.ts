import type {AppConfig} from "./config";

export async function fetchAppConfig(): Promise<AppConfig> {
    const response = await fetch('/api/config', {
        method: 'GET'
    });

    if (response.status === 200) {
        return response.json()

    } else {
        throw new Error('config load error')
    }
}

export async function sendCounterData(locationName: string, counterName: string, month: string, counterValue: string): Promise<AppConfig> {
    const report = new ReportCounterData(locationName, counterName, month, counterValue);

    const response = await fetch('/api/counter', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(report)
    });

    if (response.status === 200) {
        return response.json()

    } else {
        throw new Error('config load error')
    }
}

export class ReportCounterData {
    constructor(locationName: string, counterName: string, month: string, counterValue: string) {
        this.locationName = locationName;
        this.counterName = counterName;
        this.month = month;
        this.counterValue = counterValue;
    }
    locationName: string;
    counterName: string;
    month: string;
    counterValue: string;
}