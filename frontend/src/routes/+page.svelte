<script lang="ts">
	import type {PageData} from "./$types";
	import {fetchAppConfig} from "$lib/api";
	import type {AppConfig} from "$lib/config";

	export let data: PageData;

	let pageLoading = true;
	let unexpectedError = false;

	fetchAppConfig().then((config: AppConfig) => {
		console.log('config:', config);
		data.config = config;
		pageLoading = false;

	}).catch((e: any) => {
		console.error('unable to get config:', e);
		pageLoading = false;
		unexpectedError = true;
	})
</script>

<svelte:head>
	<title>{data.config.page.title}</title>
	<meta name="description" content="Mail Utilities Wizard" />
</svelte:head>

<section class="p-3">
	{#if !pageLoading}
	<h1>{data.config.page.header}</h1>
	<div>{data.config.page.selectLocationText}</div>
	{:else if unexpectedError}
		<div>Unexpected application error</div>
	{/if}
</section>

<style>
</style>
