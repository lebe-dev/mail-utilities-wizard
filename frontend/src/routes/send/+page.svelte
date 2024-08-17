<script lang="ts">
	import type {PageData} from "./$types";
	import {fetchAppConfig, fetchMailTemplate, sendCounterData} from "$lib/api";
	import {AppConfig, Counter, Location} from "$lib/config";
	import {onMount} from "svelte";

	export let data: PageData;

	let pageLoading = true;

	let counterDataSending = false;
	let unexpectedError = false;

	let showSuccessMessage = false;
	let showErrorMessage = false;

	let rerenderForm: boolean = false;

	let currentLocationName: string = '';
	let currentLocation: Location = new Location();
	let currentCounterName: string = '';
	let currentCounter: Counter = new Counter();

	let year: number = new Date().getFullYear();
	let periodValue: string = '';
	let customPeriodValue: string = '';
	let counterValue: string = '';

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
			currentLocationName = data.config.locale.selectLocationDropdown;
			currentLocation = new Location();
			currentCounterName = data.config.locale.selectCounterDropdown;
			currentCounter = new Counter();
			const now = new Date();
			now.setDate(0);
			year = now.getFullYear();
			customPeriodValue = `${now.getFullYear()}-${now.getUTCMonth() + 1}`;

			periodValue = getPreviousMonth();
			pageLoading = false;

		}).catch((e: any) => {
			console.error('unable to get config:', e);
			pageLoading = false;
			unexpectedError = true;
		})
	}

	function isLocationSelected(): boolean {
		return currentLocationName !== data.config.locale.selectLocationDropdown;
	}

	function onSelectLocation(e: any) {
		let value = e.target?.value;

		if (value !== undefined) {
			if (value === data.config.locale.selectLocationDropdown) {
				loadConfig();
				rerender();

			} else {
				const location: Location|undefined = data.config.locations.find((entity) => entity.name === value);

				if (location !== undefined) {
					console.log('location:', location.name);
					currentLocationName = location.name;
					currentLocation = location;
					currentCounterName = data.config.locale.selectCounterDropdown;
					currentCounter = new Counter();
					periodValue = getPreviousMonth();
					rerender();
				}
			}
		}
	}

	function isCounterSelected(): boolean {
		return currentCounterName !== data.config.locale.selectCounterDropdown;
	}

	function onSelectCounter(e: any) {
		console.log('onSelectCounter', e);
		let value = e.target?.value;

		if (value !== undefined) {
			const counter: Counter|undefined = currentLocation.counters.find((entity) => entity.name === value);

			if (counter !== undefined) {
				console.log('counter:', counter.name);
				currentCounter = counter;
				currentCounterName = counter.name;

			} else {
				currentCounter = new Counter();
				currentCounterName = data.config.locale.selectCounterDropdown;
			}

			rerender();
		}
	}

	function isUnsupportedCounter(counter: Counter): boolean {
		return counter.url.length > 0 && counter.manual.length > 0
	}

	function isPeriodSelected(): boolean {
		return periodValue.length > 0;
	}

	function onPeriodValueUpdate(e: any) {
		let value = e.target?.value;

		if (value !== undefined) {
			customPeriodValue = value;
			const [periodYear, month] = value.split('-');

			year = periodYear;

			const date = new Date(periodYear, month - 1);

			customPeriodValue = `${date.getFullYear()}-${(date.getUTCMonth() + 1).toString().padStart(2, '0')}`;
			console.log('custom period value', customPeriodValue);

			periodValue = getMonthName(date, data.config.locale.language);
			console.log('period value:', customPeriodValue);
			isFormValid();
		}
	}

	function getPreviousMonth(): string {
		let now = new Date();
		now.setDate(0);
		year = now.getFullYear();
		const month = (now.getUTCMonth() + 1).toString().padStart(2, '0');
		customPeriodValue = `${now.getFullYear()}-${month}`;
		return getMonthName(now, data.config.locale.language)
	}

	function getCurrentMonth(): string {
		let now = new Date();
		year = now.getFullYear();
		const month = (now.getUTCMonth() + 1).toString().padStart(2, '0');
		customPeriodValue = `${now.getFullYear()}-${month}`;
		return getMonthName(now, data.config.locale.language)
	}

	function getMonthName(date: Date, lang: string): string {
		return date.toLocaleString(lang,{month:'long'});
	}

	function onSelectPreviousMonth() {
		periodValue = getPreviousMonth();
	}

	function onSelectCurrentMonth() {
		periodValue = getCurrentMonth();
	}

	function onCounterValueUpdate(e: any) {
		let value = e.target?.value;

		if (value !== undefined) {
			counterValue = value;
			console.log('counter value:', counterValue);
			isFormValid();
		}
	}

	function isFormValid() {
		formValid = periodValue.length >0 && counterValue.length > 0 && counterValue != '0';
		console.log('form valid:', formValid);
	}

	function onShowMailTemplate() {
		fetchMailTemplate(currentLocationName, currentCounterName, year, periodValue, counterValue).then((template) => {
			mailTemplateSubject = template.subject
			mailTemplateBody = template.body

		}).catch((e) => {
			console.error(e);
		})
	}

	function onSend() {
		if (confirm(data.config.locale.sendConfirmMsg)) {
			console.log('location:', currentLocation);
			console.log('counter:', currentCounter);
			console.log('year:', year);
			console.log('period:', periodValue);
			console.log('value:', counterValue);

			counterDataSending = true;

			sendCounterData(currentLocationName, currentCounterName,
					year, periodValue, counterValue).then(() => {

				showSuccessMessage = true;
				showErrorMessage = false;
				counterDataSending = false;

				location.href='/success';

			}).catch((e) => {
				console.error(e);

				showSuccessMessage = false;
				showErrorMessage = true;
				counterDataSending = false;
			})
		}
	}

	function rerender() {
		rerenderForm = !rerenderForm;
	}
</script>

<svelte:head>
	<title>{data.config.locale.title}</title>
	<meta name="description" content="Mail Utilities Wizard" />
</svelte:head>

<section class="bg-white container rounded mt-1 pt-3 ps-4 pe-4 pb-4">
	{#if !pageLoading}
		<!-- MODAL -->
		<div id="mailTemplateModal" class="modal" tabindex="-1">
			<div class="modal-dialog">
				<div class="modal-content">
					<div class="modal-header">
						<h5 class="modal-title">{data.config.locale.mailTemplateTitle}</h5>
						<button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
					</div>
					<div class="modal-body">
						<p><strong>{data.config.locale.mailTemplateToLabel}</strong> {currentCounter.email}</p>
						<p><strong>{data.config.locale.mailTemplateSubjectLabel}</strong> {mailTemplateSubject}</p>
						<p><strong>{data.config.locale.mailTemplateBodyLabel}</strong></p>
						<p style="white-space: pre-line;">{mailTemplateBody}</p>
					</div>
					<div class="modal-footer">
						<button type="button" class="btn btn-secondary"
								data-bs-dismiss="modal">{data.config.locale.mailTemplateCloseButton}</button>
					</div>
				</div>
			</div>
		</div>
		<!-- /MODAL -->

		<h3 class="mb-3">{data.config.locale.header}</h3>

		{#key rerenderForm}

		<div class="mb-3">
			<label for="select-location">
				{data.config.locale.selectLocationLabel}
				<select id="select-location" bind:value={currentLocationName}
						class="form-select" on:change={onSelectLocation} disabled={counterDataSending}>
					<option value={data.config.locale.selectLocationDropdown}>
						{data.config.locale.selectLocationDropdown}
					</option>
					{#each data.config.locations as location}
						<option value={location.name}>{location.name}</option>
					{/each}
				</select>
			</label>
		</div>

		{#if isLocationSelected()}
			<div>{data.config.locale.selectCounterLabel}</div>
			<select class="form-select mb-3 w-auto" bind:value={currentCounterName}
					on:change={onSelectCounter} disabled={counterDataSending}>
				<option selected={!isCounterSelected()}
						value={data.config.locale.selectCounterDropdown}>{data.config.locale.selectCounterDropdown}</option>
				{#each currentLocation.counters as counter}
					<option value={counter.name}>{counter.name}</option>
				{/each}
			</select>

			{#if isCounterSelected()}
				<div class="form-group mb-3">
					<label for="account-id">
						{data.config.locale.accountIdLabel}
						<input id="account-id" value={currentCounter.accountId} disabled class="form-control"/>
					</label>
					<div class="form-text">{data.config.locale.accountIdHint}</div>
				</div>

				{#if isUnsupportedCounter(currentCounter)}
					<div class="rounded border border-primary p-3 mb-3">
						<div class="mb-3">{currentCounter.manual}</div>
						<div>
							<a href={currentCounter.url} target="_blank">{currentCounter.url}</a>
						</div>
					</div>

				{:else}
					<div class="mb-3">
						<label for="email">
							{data.config.locale.emailLabel}
							<input id="email" value={currentCounter.email} disabled class="form-control"/>
						</label>
					</div>

					<div class="mb-3">
						<div class="mb-3">{data.config.locale.periodLabel} {periodValue}</div>

						<div class="btn-group mb-3" role="group" aria-label="Select period">
							<button type="button" on:click={onSelectPreviousMonth} disabled={counterDataSending}
									class={periodValue === getPreviousMonth() ? 'btn btn-outline-secondary active' : 'btn btn-outline-secondary'}>{getPreviousMonth()}</button>
							<button type="button" on:click={onSelectCurrentMonth} disabled={counterDataSending}
									class={periodValue === getCurrentMonth() ? 'btn btn-outline-secondary active rounded-end me-3' : 'btn btn-outline-secondary rounded-end me-3'}>{getCurrentMonth()}</button>
						</div>

						<input id="period-value" type="month" bind:value={customPeriodValue} disabled={counterDataSending}
							   class={periodValue !== getCurrentMonth() && periodValue !== getPreviousMonth() ? 'inline-block form-control border-secondary' : 'form-control inline-block '}
							   on:change={onPeriodValueUpdate}>
					</div>

					{#if isPeriodSelected()}
						<div class="mb-3">
							<label for="counter-value">
								{data.config.locale.counterValueLabel}
								<input id="counter-value" type="text"
									   bind:value={counterValue} on:keyup={onCounterValueUpdate}
									   disabled={counterDataSending}
									   class="form-control" required>
							</label>
						</div>

						<hr>

						<div>
							{#if !counterDataSending}
								<div class="row">
									<div class="col col-lg-6">
										<button class="btn btn-primary me-3 mb-3" type="button" on:click={onSend}
												disabled={!formValid}>{data.config.locale.sendButton}</button>

										<button type="button" class="btn btn-light" data-bs-toggle="modal"
												on:click={onShowMailTemplate} disabled={!formValid}
												data-bs-target="#mailTemplateModal">
											{data.config.locale.showLetterButton}
										</button>
									</div>
									<div class="col col-lg-6 text-end">
										<a class="btn btn-light" href="/">{data.config.locale.backButton}</a>
									</div>
								</div>
							{/if}

							{#if counterDataSending}
								<div class="mt-3 mb-3">
									<div role="status" class="mb-3">{data.config.locale.sendingMsg}</div>
									<div class="progress" role="progressbar" aria-label="Animated striped example" aria-valuenow="75" aria-valuemin="0" aria-valuemax="100">
										<div class="progress-bar progress-bar-striped progress-bar-animated" style="width: 100%"></div>
									</div>
								</div>
							{/if}
							{#if showSuccessMessage}
								<div class="mt-3">
									<div class="alert alert-primary" role="alert">
										{data.config.locale.sendSuccessMsg}
									</div>
								</div>
							{/if}
							{#if showErrorMessage}
								<div class="mt-3">
									<div class="alert alert-danger" role="alert">
										{data.config.locale.sendErrorMsg}
									</div>
								</div>
							{/if}
						</div>
					{/if}
				{/if}

			{/if}
		{/if}

	{/key}

	{:else if unexpectedError}
		<div>Unexpected application error</div>
	{/if}
</section>

<style>
	.container {
		max-width: 800px;
	}

	table {
		max-width: 100% !important;
		width: 100% !important;
	}

	.btn {
		min-width: 150px;
	}
</style>