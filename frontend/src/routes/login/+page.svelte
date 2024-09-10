<script lang="ts">
    import {onMount} from "svelte";
    import {doLogin, fetchLocaleConfig} from "$lib/api";
    import {LocaleConfig} from "$lib/config";

    let passwordInput: HTMLInputElement;

    let config: LocaleConfig = new LocaleConfig();

    let password: string = '';

    let showLoginError = false;

    onMount(async () => {
        loadConfig();
    });

    function loadConfig() {
        fetchLocaleConfig().then((data: LocaleConfig) => {
            console.log('config:', data);
            config = data;

        }).catch((e: any) => {
            console.error('unable to get config:', e);
        })
    }

    function onLogin() {
        doLogin(password).then(() => {
            location.href='/';

        }).catch((e) => {
            console.error(e);
            showLoginError = true;
            passwordInput.focus();
        })
    }
</script>

<div id="login-form" class="container bg-white ps-4 pe-4 pt-5 pb-5 rounded-bottom-4">
    <div class="mb-2">{config.loginText}</div>

    <div class="mb-4">
        <input bind:this={passwordInput} type="password"
               class="form-control" bind:value={password} autofocus={true}>
    </div>

    <button class="btn btn-primary w-100 align-middle">
        <a class="text-white text-decoration-none" on:click={onLogin}>
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" fill="currentColor"
                 class="bi bi-box-arrow-in-right me-1" viewBox="0 0 16 16">
                <path fill-rule="evenodd" d="M6 3.5a.5.5 0 0 1 .5-.5h8a.5.5 0 0 1 .5.5v9a.5.5 0 0 1-.5.5h-8a.5.5 0 0 1-.5-.5v-2a.5.5 0 0 0-1 0v2A1.5 1.5 0 0 0 6.5 14h8a1.5 1.5 0 0 0 1.5-1.5v-9A1.5 1.5 0 0 0 14.5 2h-8A1.5 1.5 0 0 0 5 3.5v2a.5.5 0 0 0 1 0z"/>
                <path fill-rule="evenodd" d="M11.854 8.354a.5.5 0 0 0 0-.708l-3-3a.5.5 0 1 0-.708.708L10.293 7.5H1.5a.5.5 0 0 0 0 1h8.793l-2.147 2.146a.5.5 0 0 0 .708.708z"/>
            </svg> {config.loginButton}
        </a>
    </button>

    {#if showLoginError}
    <div id="error-msg" class="mt-4 rounded border border-danger text-danger ps-3 pe-3 pt-2 pb-2">
        {config.loginErrorMsg}
    </div>
    {/if}
</div>

<style>
    #login-form {
        max-width: 400px;
    }

    #error-msg {
        font-size: 0.9rem;
    }
</style>