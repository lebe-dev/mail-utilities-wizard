<script lang="ts">
	import type {PageData} from "./$types";

	export let data: PageData;
</script>

<svelte:head>
	<title>{data.config.page.title}</title>
	<meta name="description" content="Mail Utilities Wizard" />
</svelte:head>

<section>
	<h1>{data.config.page.header}</h1>
</section>

<style>
</style>
