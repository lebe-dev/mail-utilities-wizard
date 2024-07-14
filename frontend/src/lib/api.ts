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