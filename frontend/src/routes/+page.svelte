<script lang="ts">
	import type {PageData} from "./$types";
	import {fetchAppConfig} from "$lib/api";
	import {AppConfig, Counter, Location} from "$lib/config";
	import {onMount} from "svelte";

	export let data: PageData;

	let pageLoading = true;
	let showMailTemplate = true;
	let counterDataSending = false;
	let unexpectedError = false;

	let showSuccessMessage = false;
	let showErrorMessage = false;

	let rerenderForm: boolean = false;

	let currentLocationName: string = '';
	let currentLocation: Location = new Location();
	let currentCounterName: string = '';
	let currentCounter: Counter = new Counter();

	let periodValue: string = '';
	let customPeriodValue: string = '';
	let counterValue: string = '';
	let year: number = new Date().getFullYear();

	let mailTemplateSubject: string = '';
	let mailTemplateBody: string = '';

	let formValid: boolean = false;

	onMount(async () => {
		loadConfig();
	});

	function loadConfig() {
		pageLoading = true;
		fetchAppConfig().then((config: AppConfig) => {
			console.log('config:', config);
			data.config = config;
			currentLocationName = data.config.page.selectLocationDropdown;
			currentLocation = new Location();
			currentCounterName = data.config.page.selectCounterDropdown;
			currentCounter = new Counter();
			const now = new Date();
			now.setDate(0);
			customPeriodValue = `${now.getFullYear()}-${now.getUTCMonth() + 1}`;

			periodValue = getPreviousMonth();
			pageLoading = false;

		}).catch((e: any) => {
			console.error('unable to get config:', e);
			pageLoading = false;
			unexpectedError = true;
		})
	}

	function getPreviousMonth(): string {
		let now = new Date();
		now.setDate(0);
		year = now.getFullYear();
		const month = (now.getUTCMonth() + 1).toString().padStart(2, '0');
		customPeriodValue = `${now.getFullYear()}-${month}`;
		return getMonthName(now, data.config.page.locale)
	}

	function getMonthName(date: Date, lang: string): string {
		return date.toLocaleString(lang,{month:'long'});
	}

	function isFormValid() {
		formValid = periodValue.length >0 && counterValue.length > 0 && counterValue != '0';
		console.log('form valid:', formValid);
	}

	function rerender() {
		rerenderForm = !rerenderForm;
	}
</script>

<svelte:head>
	<title>{data.config.page.title}</title>
	<meta name="description" content="Mail Utilities Wizard" />
</svelte:head>

<section class="bg-white container rounded mt-1 pt-3 ps-4 pe-4 pb-4">
	{#if !pageLoading}
		<h3 class="mb-3">{data.config.page.header}</h3>

		<div>
			<a href="/send" class="btn btn-outline-primary mb-4">{data.config.page.sendButton}</a>
		</div>

		<div class="mb-3">
			Показания за предыдущие периоды:
		</div>

		<table class="table table-striped">
			<thead>
				<tr>
					<th>Дата отправки</th>
					<th>Адрес</th>
					<th>Счётчик</th>
					<th>Период</th>
					<th>Значение</th>
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

	{:else if unexpectedError}
		<div>Unexpected application error</div>
	{/if}
</section>

<style>
	.btn {
		min-width: 150px;
	}
</style>