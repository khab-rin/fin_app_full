<script lang="ts">
	import '$lib/style/global.css'

	import {pageManager} from '$lib/models/MainManager/MainManager.svelte'

	let { children } = $props();
	
	async function logOut() {
		await pageManager.logOut();
	}

</script>



<header>
	<span>{pageManager.compName}</span>
	<button 
		class="but-header" 
		onclick={logOut}
		disabled={pageManager.totalOff}> 
			выход
	</button>
</header>


<aside>
	{@render children()}
</aside>

<main>
	{#key pageManager.Page}
		{#if pageManager.getPage}
			{@const CurrentPage = pageManager.getPage}
			<CurrentPage />
		{/if}
	{/key}
</main>

<footer>
	<span>footer</span>
</footer>

