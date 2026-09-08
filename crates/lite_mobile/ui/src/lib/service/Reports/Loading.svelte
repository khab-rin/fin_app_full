<script lang='ts'>
	import { reportManager } from '$lib/models/Reports/ReportsManager.svelte';
	import type { ReportStep } from '$lib/models/rustModels/ReportStep';
	import {invoke} from '@tauri-apps/api/core';

	function goToFnsReports() {
		const nextStep: ReportStep = {FnsReports: {text: 'Выберите тип отчета в налоговую и период'}};
		reportManager.step = nextStep;
	}

	function goToHomeReports() {
		const nextStep: ReportStep = {HomeReports: {text: 'Выберите тип управленческого отчета и период'}};
		reportManager.step = nextStep;
	}


</script>

<section class='group-one'>

	<button
		class='green-button'
		type='button'
		disabled={false}
		onclick={goToFnsReports}
	>
		Отчеты для налоговой
	</button>

	<button
		class='green-button'
		type='button'
		disabled={false}
		onclick={goToHomeReports}
	>
		Внутренние управленческие отчеты
	</button>

</section>