<script lang="ts">
	import type {PageData} from "./$types";
	import {fetchAppConfig} from "$lib/api";
	import {type AppConfig, Counter, Location} from "$lib/config";

	export let data: PageData;

	let pageLoading = true;
	let unexpectedError = false;

	let rerenderForm: boolean = false;

	let currentLocation: Location = new Location();
	let currentCounter: Counter = new Counter();

	let periodValue: string = '';
	let counterValue: string = '';

	let formValid: boolean = false;

	fetchAppConfig().then((config: AppConfig) => {
		console.log('config:', config);
		data.config = config;
		currentLocation = new Location();
		currentLocation.name = data.config.page.selectLocationDropdown;
		currentCounter = new Counter();
		currentCounter.name = data.config.page.selectCounterDropdown;
		pageLoading = false;

	}).catch((e: any) => {
		console.error('unable to get config:', e);
		pageLoading = false;
		unexpectedError = true;
	})

	function isLocationSelected(): boolean {
		return currentLocation.name !== data.config.page.selectLocationDropdown;
	}

	function onSelectLocation(e: any) {
		let value = e.target?.value;

		if (value !== undefined) {
			const location = data.config.locations.find((entity) => entity.name === value);

			if (location !== undefined) {
				console.log('location:', location.name);
				currentLocation = location;
				currentCounter = new Counter();
				currentCounter.name = data.config.page.selectCounterDropdown;
				rerender();
			}
		}
	}

	function isCounterSelected(): boolean {
		return currentCounter.name !== data.config.page.selectLocationDropdown;
	}

	function onSelectCounter(e: any) {
		let value = e.target?.value;

		if (value !== undefined) {
			const counter = currentLocation.counters.find((entity) => entity.name === value);

			if (counter !== undefined) {
				console.log('counter:', counter.name);
				currentCounter = counter;
				rerender();
			}
		}
	}

	function onPeriodValueUpdate(e: any) {
		let value = e.target?.value;

		if (value !== undefined) {
			periodValue = value;
			console.log('period value:', periodValue);
			isFormValid();
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
		formValid = counterValue.length > 0 && counterValue != '0';
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

<section class="p-3">
	{#if !pageLoading}
	<h1>{data.config.page.header}</h1>


	{#key rerenderForm}

	<label for="select-location">
		{data.config.page.selectLocationLabel}
		<select id="select-location" bind:value={currentLocation.name} class="form-select" on:change={onSelectLocation}>
			<option>{data.config.page.selectLocationDropdown}</option>
			{#each data.config.locations as location}
				<option value={location.name}>{location.name}</option>
			{/each}
		</select>
	</label>

	{#if isLocationSelected()}
		<div>{data.config.page.selectCounterLabel}</div>
		<select class="form-select" bind:value={currentCounter.name} on:change={onSelectCounter}>
			<option value={data.config.page.selectCounterDropdown}>{data.config.page.selectCounterDropdown}</option>
			{#each currentLocation.counters as counter}
				<option value={counter.name}>{counter.name}</option>
			{/each}
		</select>

		{#if isCounterSelected()}
			<div class="form-group">
				<label for="account-id">
					{data.config.page.accountIdLabel}
					<input id="account-id" value={currentCounter.accountId} disabled class="form-control"/>
				</label>
			</div>

			<div class="form-group">
				<label for="email">
					{data.config.page.emailLabel}
					<input id="email" value={currentCounter.email} disabled class="form-control"/>
				</label>
			</div>

			<div class="form-group">
				<label for="period-value">
					{data.config.page.periodLabel}
					<input id="period-value" type="month" bind:value={periodValue} on:keyup={onPeriodValueUpdate}
						   class="form-control" required>
				</label>
			</div>

			<div class="form-group">
				<label for="counter-value">
					{data.config.page.counterValueLabel}
					<input id="counter-value" type="number" bind:value={counterValue} on:keyup={onCounterValueUpdate}
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

	{/key}

	{:else if unexpectedError}
		<div>Unexpected application error</div>
	{/if}
</section>

<style>
</style>
