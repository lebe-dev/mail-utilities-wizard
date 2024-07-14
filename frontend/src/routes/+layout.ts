import type {LayoutLoad} from './$types';
import {AppConfig} from "../lib/config";

export const ssr = false;

export const load: LayoutLoad = async () => {
    return {
        config: new AppConfig()
    };
};