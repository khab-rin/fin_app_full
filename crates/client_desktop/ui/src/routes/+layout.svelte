<script lang="ts">
	import { page } from '$app/state';
    import '$lib/style/global.css'
    import favicon from '$lib/assets/favicon.svg';
	import { goTo } from '$lib/rules/navigation';
	import {currAuthStep} from "$lib/models/Auth/AuthStep.svelte";
	import { appState } from '$lib/models/AppState/appState.svelte';
	import SettingsMainMenu from "$lib/service/Settings/SettingsMainMenu.svelte";

	import {AuthStepType} from "$lib/models/Auth/AuthValues";

	let { children } = $props<{ children: import('svelte').Snippet }>();

	$effect(() => {
        if (AuthStepType.SuccessShort in currAuthStep.step) {
            appState.Page = null;
        } else {
			appState.Page = "auth";
		}
    });
</script>


<svelte:head>
    <link rel="icon" href={favicon} />
</svelte:head>


<div class="app-container">
	{#if appState.settingsOnOff}
		<SettingsMainMenu/>
	{/if}
	<header class="top-bar">
		<div class="user-avatar">
			<span class="avatar-icon">👤</span>
		</div>
		<input type="text" class="search-input" placeholder="Поиск...">
		<button 
			class="param-btn" 
			onclick={() => appState.settingsOnOff = true}
			disabled={appState.totalOff}>⚙️
		</button>
	</header>



	<main class="main-content">
		{#if appState.getPage}
			<appState.getPage />
		{:else}
			{@render children()}
		{/if}
	</main>

	<footer class="bottom-nav">

		<button
			class="nav-item"
			class:active={page.url.pathname === '/' }
			onclick={() => goTo('/')}
		>
			<span class="nav-icon">🏠</span>
			<span class="nav-label">Главная</span>
		</button>
		<button class="nav-item">
			<span class="nav-icon">📊</span>
			<span class="nav-label">Статистика</span>
		</button>
		<button class="nav-item">
			<span class="nav-icon">📅</span>
				<span class="nav-label">Календарь</span>
		</button>
		<button class="nav-item">
			<span class="nav-icon">❓</span>
			<span class="nav-label">Справка</span>
		</button>
	</footer>
</div>


