<script lang='ts'>
	import {pageManager} from '$lib/models/MainManager/MainManager.svelte';

</script>




<div class='w-v25'>
	<button 
		class="but-footer"
		onclick={pageManager.Page = null}
		disabled={pageManager.totalOff}
	>
		<span class="bar-icon">🏠</span>
		<span class="bar-label">Главная</span>
	</button>
</div>

<div class='w-v25'>
	<button class="but-footer"
	>
		<span class="bar-icon">📊</span>
		<span class="bar-label">Статистика</span>
	</button>
</div>

<div class='w-v25'>
	<button class="but-footer"
		onclick={pageManager.Page = null}
	>
		<span class="bar-icon">📅</span>
		<span class="bar-label">Календарь</span>
	</button>
</div>

<div class='w-v25'>
	<button class="but-footer"
	>
	<span class="bar-icon">❓</span>
	<span class="bar-label">Справка</span>
	</button>
</div>

