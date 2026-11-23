<script lang='ts'>
	import {dbStep} from '$lib/models/DataBase/DbManager.svelte';
	import {pageManager} from '$lib/models/MainManager/MainManager.svelte';
	import type {DatabaseStep} from '$lib/models/rustModels/DatabaseStep';
	

	function goToLoading() {
		const nextStep: DatabaseStep = {Loading: {text: 'Выберите функционал для работы с базой данных'}};
		dbStep.step = nextStep;
	}

	function closeReport() {
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

<svelte:component this={dbStep.getPage}/>


