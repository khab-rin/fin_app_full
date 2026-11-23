<script lang='ts'>
	import {dbStep} from '$lib/models/DataBase/DbManager.svelte';
	import {pageManager} from '$lib/models/MainManager/MainManager.svelte';
	import type {DatabaseStep} from '$lib/models/rustModels/DatabaseStep';
	const CurrentPage = $derived(dbStep.getPage);
	

	function closeDataBase() {
		const nextStep: DatabaseStep = {Loading: {text: 'Выберите функционал для работы с базой данных'}};
		dbStep.step = nextStep;
		pageManager.Page = null;
	}

</script>

<section class='w-100'>
	<span class='x-selft-c'>
		{dbStep.currentText}
	</span>
</section>

{#if CurrentPage}
	<CurrentPage />
{/if}


<section class='w-100'>
	<button
		type="button"
		class="but-bl"
		onclick={closeDataBase}
		>
		<span>
			Основное меню
		</span>
	</button>
</section>

