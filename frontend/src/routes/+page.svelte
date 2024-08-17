<script lang="ts">
	import type {PageData} from "./$types";
	import {fetchAppConfig} from "$lib/api";
	import {AppConfig} from "$lib/config";
	import {onMount} from "svelte";
	import {getMonthName} from "$lib/period.js";

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

	function getHumanDate(timestamp: number): string {
		console.log(timestamp);
		const date = new Date(timestamp * 1000);
		return `${date.getDate()} ${getMonthName(date, data.config.locale.language)} ${date.getFullYear()}`;
	}
</script>

<svelte:head>
	<title>{data.config.locale.title}</title>
	<meta name="description" content="Mail Utilities Wizard" />
</svelte:head>

<section class="bg-white container rounded mt-1 pt-3 ps-4 pe-4 pb-4">
	{#if !pageLoading}
		<h3 class="vertical-align mb-3">
			<svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" fill="currentColor" class="bi bi-envelope-at me-2" viewBox="0 0 16 16">
				<path d="M2 2a2 2 0 0 0-2 2v8.01A2 2 0 0 0 2 14h5.5a.5.5 0 0 0 0-1H2a1 1 0 0 1-.966-.741l5.64-3.471L8 9.583l7-4.2V8.5a.5.5 0 0 0 1 0V4a2 2 0 0 0-2-2zm3.708 6.208L1 11.105V5.383zM1 4.217V4a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v.217l-7 4.2z"/>
				<path d="M14.247 14.269c1.01 0 1.587-.857 1.587-2.025v-.21C15.834 10.43 14.64 9 12.52 9h-.035C10.42 9 9 10.36 9 12.432v.214C9 14.82 10.438 16 12.358 16h.044c.594 0 1.018-.074 1.237-.175v-.73c-.245.11-.673.18-1.18.18h-.044c-1.334 0-2.571-.788-2.571-2.655v-.157c0-1.657 1.058-2.724 2.64-2.724h.04c1.535 0 2.484 1.05 2.484 2.326v.118c0 .975-.324 1.39-.639 1.39-.232 0-.41-.148-.41-.42v-2.19h-.906v.569h-.03c-.084-.298-.368-.63-.954-.63-.778 0-1.259.555-1.259 1.4v.528c0 .892.49 1.434 1.26 1.434.471 0 .896-.227 1.014-.643h.043c.118.42.617.648 1.12.648m-2.453-1.588v-.227c0-.546.227-.791.573-.791.297 0 .572.192.572.708v.367c0 .573-.253.744-.564.744-.354 0-.581-.215-.581-.8Z"/>
			</svg> <div class="d-inline-block">{data.config.locale.header}</div>
		</h3>

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
						<td class="text-size-mobile">{getHumanDate(record.created)}</td>
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
						<td>{getHumanDate(record.created)}</td>
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

	.vertical-align {
		display: flex;
		align-items: center;
	}
</style>