<script lang="ts">
	import '$lib/style/global.css'

	let { children } = $props<{ children: import('svelte').Snippet }>();

	import {pageManager} from '$lib/models/MainManager/MainManager.svelte'
	import { PageType } from "$lib/models/MainManager/PageValues";

	function goToOperation() {
		pageManager.Page = PageType.Operation;
	}

	function goToMchd() {
		pageManager.Page = PageType.Mchd;
	}

	function goToReports() {
		pageManager.Page = PageType.Report;
	}

	async function logOut() {
		await pageManager.logOut();
	}

</script>





<div class='zone-global'>

	<header class='zone-top'>
		<div>
			<span class='t-base t-italic'>{pageManager.compName}</span>
		</div>

		<button 
			class="zone-top-button" 
			onclick={logOut}
			disabled={pageManager.totalOff}> выход
		</button>
	</header>

	<aside class='zone-left'>
		{#if !pageManager.totalOff}
			<div class='w-100'>
				<div class='w-v100'>
					<label
						class='label-close x-self-l'
						for='mainOperationButton'
					>
						Раздел операций
					</label>
					<button
						type='button'
						class='but-bl'
						id='mainOperationButton'
						disabled={pageManager.totalOff}
						onclick={goToOperation}
					>
						<span class='t-base x-self-c  t-bold'>
							Операции
						</span>
					</button>
				</div>
			</div>

			<div class='w-100'>

				<div class='w-v100'>
					<label
						class='label-close x-self-l'
						for='mainReportsButton'
					>
						Раздел отчетов
					</label>
					<button
						type='button'
						class='but-bl'
						id='mainReportsButton'
						disabled={pageManager.totalOff}
						onclick={goToReports}
					>
						<span class='t-base x-self-c t-bold'>
							Отчеты
						</span>
					</button>
				</div>
			</div>

			<div class='w-100'>
				<div class='w-v100'>
					<label
						class='label-close x-self-l'
						for='mainMchdButton'
					>
						Раздел доверенностей
					</label>
					<button
						type='button'
						class='but-bl'
						id='mainMchdButton'
						disabled={pageManager.totalOff}
						onclick={goToMchd}
					>
						<span class='t-base x-self-c t-bold'>
							Доверенности
						</span>
					</button>
				</div>
			</div>
		{/if}
	</aside>

	<main class='zone-main'>

		{#if pageManager.getPage}
			<pageManager.getPage />
		{:else}
			{@render children()}
		{/if}

	</main>

	<footer class="zone-footer">
		<span>footer</span>
	</footer>


</div>






<!-- <script lang="ts">
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
			pageManager.settingsOnOff &&
			menuRef &&
			!menuRef.contains(event.target as Node) &&
			!(event.target as HTMLElement).closest('.param-button') 
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


</div> -->
