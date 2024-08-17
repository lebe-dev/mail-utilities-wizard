<script lang="ts">
    import type {PageData} from "./$types";
    import {onMount} from "svelte";
    import {fetchAppConfig} from "$lib/api";
    import {AppConfig} from "$lib/config";

    export let data: PageData;

    let pageLoading = true;
    let unexpectedError = false;

    onMount(async () => {
        loadConfig();
    });

    function loadConfig() {
        pageLoading = true;
        fetchAppConfig().then((config: AppConfig) => {
            console.log('config:', config);
            data.config = config;
            pageLoading = false;

        }).catch((e: any) => {
            console.error('unable to get config:', e);
            pageLoading = false;
            unexpectedError = true;
        })
    }
</script>

<svelte:head>
    <title>{data.config.locale.title}</title>
    <meta name="description" content="Mail Utilities Wizard" />
</svelte:head>

<section class="bg-white container rounded mt-1 pt-3 ps-4 pe-4 pb-4">
    {#if !pageLoading}
        <h3 class="mb-4">{data.config.locale.sendSuccessMsg}</h3>
        <div>
            <hr>
            <a class="btn btn-outline-primary" href="/">{data.config.locale.sendMoreButton}</a>
        </div>

    {:else if unexpectedError}
        <div class="text-danger">Unexpected application error</div>
    {/if}
</section>