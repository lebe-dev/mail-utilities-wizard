<script lang="ts">
	import type {PageData} from "./$types";
	import {fetchAppConfig} from "$lib/api";
	import {AppConfig, Counter, Location} from "$lib/config";
	import {onMount} from "svelte";

	export let data: PageData;

	let pageLoading = true;
	let unexpectedError = false;

	let rerenderForm: boolean = false;

	let currentLocationName: string = '';
	let currentLocation: Location = new Location();
	let currentCounterName: string = '';
	let currentCounter: Counter = new Counter();

	let periodValue: string = '';
	let counterValue: string = '';

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
			pageLoading = false;

		}).catch((e: any) => {
			console.error('unable to get config:', e);
			pageLoading = false;
			unexpectedError = true;
		})
	}

	function isLocationSelected(): boolean {
		return currentLocationName !== data.config.page.selectLocationDropdown;
	}

	function onSelectLocation(e: any) {
		let value = e.target?.value;

		if (value !== undefined) {
			if (value === data.config.page.selectLocationDropdown) {
				loadConfig();
				rerender();

			} else {
				const location: Location|undefined = data.config.locations.find((entity) => entity.name === value);

				if (location !== undefined) {
					console.log('location:', location.name);
					currentLocationName = location.name;
					currentLocation = location;
					currentCounterName = data.config.page.selectCounterDropdown;
					currentCounter = new Counter();
					rerender();
				}
			}
		}
	}

	function isCounterSelected(): boolean {
		return currentCounterName !== data.config.page.selectCounterDropdown;
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
				currentCounterName = data.config.page.selectCounterDropdown;
			}

			rerender();
		}
	}

	function isPeriodSelected(): boolean {
		return periodValue.length > 0;
	}

	function onPeriodValueUpdate(e: any) {
		let value = e.target?.value;

		if (value !== undefined) {
			periodValue = value;
			console.log('period value:', periodValue);
			isFormValid();
			rerender();
		}
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

	function onSend() {
		if (confirm(data.config.page.sendConfirmMsg)) {
			console.log('location:', currentLocation);
			console.log('counter:', currentCounter);
			console.log('value:', counterValue);
		}
	}

	function rerender() {
		rerenderForm = !rerenderForm;
	}
</script>

<svelte:head>
	<title>{data.config.page.title}</title>
	<meta name="description" content="Mail Utilities Wizard" />
</svelte:head>

<section class="bg-white container rounded pt-3 ps-4 pe-4 pb-4">
	{#if !pageLoading}
	<h3 class="mb-3">{data.config.page.header}</h3>

	{#key rerenderForm}

	<div class="mb-3">
		<label for="select-location">
			{data.config.page.selectLocationLabel}
			<select id="select-location" bind:value={currentLocationName}
					class="form-select" on:change={onSelectLocation}>
				<option value={data.config.page.selectLocationDropdown}>
					{data.config.page.selectLocationDropdown}
				</option>
				{#each data.config.locations as location}
					<option value={location.name}>{location.name}</option>
				{/each}
			</select>
		</label>
	</div>

	{#if isLocationSelected()}
		<div>{data.config.page.selectCounterLabel}</div>
		<select class="form-select mb-3 w-auto" bind:value={currentCounterName} on:change={onSelectCounter}>
			<option selected={!isCounterSelected()}
					value={data.config.page.selectCounterDropdown}>{data.config.page.selectCounterDropdown}</option>
			{#each currentLocation.counters as counter}
				<option value={counter.name}>{counter.name}</option>
			{/each}
		</select>

		{#if isCounterSelected()}
			<div class="form-group mb-3">
				<label for="account-id">
					{data.config.page.accountIdLabel}
					<input id="account-id" value={currentCounter.accountId} disabled class="form-control"/>
				</label>
			</div>

			<div class="mb-3">
				<label for="email">
					{data.config.page.emailLabel}
					<input id="email" value={currentCounter.email} disabled class="form-control"/>
				</label>
			</div>

			<div class="mb-3">
				<label for="period-value">
					{data.config.page.periodLabel}
					<input id="period-value" type="month" bind:value={periodValue}
						   on:change={onPeriodValueUpdate}
						   class="form-control" required>
				</label>
			</div>

			{#if isPeriodSelected()}
				<div class="mb-3">
					<label for="counter-value">
						{data.config.page.counterValueLabel}
						<input id="counter-value" type="text"
							   bind:value={counterValue} on:keyup={onCounterValueUpdate}
							   class="form-control" required>
					</label>
				</div>

				<hr>

				<div>
					<input class="btn btn-primary" type="button" on:click={onSend}
						   value={data.config.page.sendButton} disabled={!formValid}>
				</div>
			{/if}
		{/if}
	{/if}

	{/key}

	{:else if unexpectedError}
		<div>Unexpected application error</div>
	{/if}
</section>
