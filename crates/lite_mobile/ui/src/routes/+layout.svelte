<script lang="ts">
	import '$lib/style/global.css'
    import favicon from '$lib/assets/favicon.svg';
	import {pageManager} from "$lib/models/MainManager/MainManager.svelte";

	import MainFooter from '$lib/service/MainFooter.svelte';


	let { children } = $props();
	async function logOut() {
		await pageManager.logOut();
	}

</script>


<header>
	<div class='w-g10'>
		<link rel='icon' href={favicon}/>
	</div>
	<div class='w-g90'>
		<span class='t-cut'>
			{pageManager.compName}
		</span>
		
	</div>
	<div class='w-g15'>
		<button
			type='button'
			class='but-bl'
			disabled={pageManager.totalOff}
			onclick={logOut}
		>
			<span class='t-cut'>
				Выход
			</span>
		</button>
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

<footer>
	<MainFooter/>
</footer>

