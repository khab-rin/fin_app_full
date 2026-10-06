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

<section class=w-100>
	<h3 class='w-g100 x-ce'>
		{reportManager.currentText}
	</h3>
</section>


<svelte:component this={reportManager.getPage}/>


<section class='w-40'>
	<button
		type="button"
		class="but-bl"
		onclick={goToLoading}
		>
		<span>
			Меню отчетов
		</span>
	</button>
</section>


