<script lang="ts">
	import '$lib/style/global.css'

	import { page } from '$app/state';
    import favicon from '$lib/assets/favicon.svg';
	import { goTo } from '$lib/rules/navigation';
	import { fade } from 'svelte/transition';

	import SettingsMainMenu from "$lib/service/Settings/SettingsMainMenu.svelte";
	import {pageManager} from "$lib/models/MainManager/MainManager.svelte";
	
	let { children } = $props<{ children: import('svelte').Snippet }>();

	let menuRef: HTMLElement | null = $state(null);

	function handlePointerOutside(event: PointerEvent) {
		if (
			pageManager.settingsOnOff && // Если меню открыто
			menuRef && // И меню уже отрендерилось в DOM
			!menuRef.contains(event.target as Node) && // И тап был НЕ по самому меню
			!(event.target as HTMLElement).closest('.param-button') // И тап был НЕ по кнопке шестеренки
		) {
			pageManager.settingsOnOff = false; // Закрываем меню
		}
	}

</script>

<svelte:document onpointerdown={handlePointerOutside} />

<svelte:head>
    <link rel="icon" href={favicon} />
</svelte:head>

<div class='app-container'>


{#if pageManager.settingsOnOff}
	<div bind:this={menuRef} transition:fade={{ duration: 150 }}>
		<SettingsMainMenu/>
	</div>
{/if}

<header class="top-bar">
	<div class="user-avatar">
		<span class="avatar-icon">👤</span>
	</div>

	<input type="text" class="search-input" placeholder="Поиск...">

	<button 
		class="param-button" 
		onclick={() => pageManager.settingsOnOff = true}
		disabled={pageManager.totalOff}>⚙️
	</button>
</header>



<main class="main-content">
	{#if pageManager.getPage}
		<pageManager.getPage />
	{:else}
		{@render children()}
	{/if}
</main>

<footer class="down-bar">

	<button
		class="bar-button"
		class:active={page.url.pathname === '/' }
		onclick={() => goTo('/')}
	>
		<span class="bar-icon">🏠</span>
		<span class="bar-label">Главная</span>
	</button>

	<button class="bar-button">
		<span class="bar-icon">📊</span>
		<span class="bar-label">Статистика</span>
	</button>

	<button class="bar-button">
		<span class="bar-icon">📅</span>
			<span class="bar-label">Календарь</span>
	</button>

	<button class="bar-button">
		<span class="bar-icon">❓</span>
		<span class="bar-label">Справка</span>
	</button>

</footer>


</div>
