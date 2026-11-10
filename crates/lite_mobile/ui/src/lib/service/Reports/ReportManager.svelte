<script lang='ts'>
	import {reportManager} from '$lib/models/Reports/ReportsManager.svelte';
	import {pageManager} from '$lib/models/MainManager/MainManager.svelte';

	import type {ReportStep} from '$lib/models/rustModels/ReportStep';

	function goToLoading() {
		const nextStep: ReportStep = {Loading: {text: 'Выберите функционал для работы с отчетами'}};
        reportManager.step = nextStep;
	}

	function closeReport() {
		const nextStep: ReportStep = {Loading: {text: 'Выберите функционал для работы с отчетами'}};
        reportManager.step = nextStep;
		pageManager.Page = null;
	}

</script>

{#if reportManager.currentText}
	<div class='w-100'>
		<span class='x-self-c x-cont-c with-info'>
			{reportManager.currentText}
		</span>
	</div>
{/if}


<svelte:component this={reportManager.getPage}/>


<section class='w-100'>
	<div class='w-v50'>
		<button
			type="button"
			class="but-pu"
			onclick={goToLoading}
			>
			<span>
				Меню отчетов
			</span>
		</button>
	</div>

	<div class='w-v50'>
		<button
			type="button"
			class="but-pu"
			onclick={closeReport}
			>
			<span>
				Основной экран
			</span>
		</button>
	</div>
</section>


