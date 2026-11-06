<script lang="ts">
	import '$lib/style/global.css'
	
	let { children } = $props();
    import favicon from '$lib/assets/favicon.svg';

	import {pageManager} from "$lib/models/MainManager/MainManager.svelte";

	async function logOut() {
		await pageManager.logOut();
	}

</script>


<header class='top-bar'>
	<div class="w-g50"></div>
	<div class="w-g50" style="width: 100%;">
		<div class='w-g10'>
			<link rel='icon' href={favicon}/>
		</div>
		<div class='w-g80'>
			{pageManager.compName}
		</div>
		<div class='w-g10'>
			<button
				type='button'
				class='but-bl'
				disabled={pageManager.totalOff}
				onclick={logOut}
			>
				Выход
			</button>
		</div>
		<div class='style="width: 0px;'></div>

	</div>
</header>

<main>
	{#key pageManager.Page}
		{#if pageManager.getPage}
			{@const CurrentPage = pageManager.getPage}
			<CurrentPage/>
		{:else}
			{@render children()}
		{/if}
	{/key}
</main>

<footer class='footer'>
	<div class="w-100">
		<button class="w-v25"
			onclick={pageManager.Page = null}
		>
			<span class="bar-icon">🏠</span>
			<span class="bar-label">Главная</span>
		</button>

		<button class="w-v25"
		>
			<span class="bar-icon">📊</span>
			<span class="bar-label">Статистика</span>
		</button>

		<button class="w-v25"
			onclick={pageManager.Page = null}
		>
			<span class="bar-icon">📅</span>
			<span class="bar-label">Календарь</span>
		</button>

		<button class="w-v25"
		>
			<span class="bar-icon">❓</span>
			<span class="bar-label">Справка</span>
		</button>
	</div>
</footer>

