export function getMonthName(date: Date, lang: string): string {
    return date.toLocaleString(lang,{month:'long'});
}