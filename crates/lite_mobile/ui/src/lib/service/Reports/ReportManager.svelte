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

<p class="text-small">{reportManager.currentText}</p>

<svelte:component this={reportManager.getPage}/>


<section class='group-two'>
	<button
		type="button"
		class="blue-button"
		onclick={goToLoading}
		>
		<span class="blue-button-span">
			Меню отчетов
		</span>
	</button>



	<button
		type="button"
		class="blue-button"
		onclick={closeReport}
		>
		<span class="blue-button-span">
			Основной экран
		</span>
	</button>


</section>


