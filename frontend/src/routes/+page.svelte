<script lang="ts">
	import type {PageData} from "./$types";
	import {fetchAppConfig} from "$lib/api";
	import {AppConfig} from "$lib/config";
	import {onMount} from "svelte";

	export let data: PageData;

	let pageLoading = true;
	let unexpectedError = false;

	let isMobileDevice: boolean = false;

	onMount(async () => {
		const regex = /Mobi|Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i;
		isMobileDevice = regex.test(navigator.userAgent);

		console.log('mobile device:', isMobileDevice);

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
		<h3 class="mb-3">{data.config.locale.header}</h3>

		<div>
			<a href="/send" class="btn btn-outline-primary mb-4">{data.config.locale.sendButton}</a>
		</div>

		<div class="mb-3">
			{data.config.locale.historyTableText}
		</div>

		{#if isMobileDevice}
			<table class="table table-striped">
				<thead>
				<tr>
					<th>{data.config.locale.historyRecordDate}</th>
					<th>{data.config.locale.historyRecordValues}</th>
				</tr>
				</thead>
				<tbody>
				{#each data.config.historyRecords as record}
					<tr>
						<td class="text-size-mobile">{new Date(record.created)}</td>
						<td class="text-size-mobile">
							<div><strong>{data.config.locale.historyRecordLocation}</strong></div>
							<div>{record.location}</div>
							<div><strong>{data.config.locale.historyRecordCounter}</strong></div>
							<div>{record.counterName}</div>
							<div><strong>{data.config.locale.historyRecordPeriod}</strong></div>
							<div>{record.month} {record.year}</div>
							<div><strong>{data.config.locale.historyRecordValue}</strong></div>
							<div>{record.value}</div>
						</td>
					</tr>
				{/each}
				</tbody>
			</table>

		{:else}
			<table class="table table-striped">
				<thead>
				<tr>
					<th>{data.config.locale.historyRecordDate}</th>
					<th>{data.config.locale.historyRecordLocation}</th>
					<th>{data.config.locale.historyRecordCounter}</th>
					<th>{data.config.locale.historyRecordPeriod}</th>
					<th>{data.config.locale.historyRecordValue}</th>
				</tr>
				</thead>
				<tbody>
				{#each data.config.historyRecords as record}
					<tr>
						<td>{record.created}</td>
						<td>{record.location}</td>
						<td>{record.counterName}</td>
						<td>{record.month} {record.year}</td>
						<td>{record.value}</td>
					</tr>
				{/each}
				</tbody>
			</table>
		{/if}

	{:else if unexpectedError}
		<div>Unexpected application error</div>
	{/if}
</section>

<style>
	.container {
		max-width: 1024px;
	}

	.btn {
		min-width: 150px;
	}

	.text-size-mobile {
		font-size: 0.9rem;
	}
</style>