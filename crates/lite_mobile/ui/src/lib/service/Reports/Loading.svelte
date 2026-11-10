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

<div class='w-100'>
	<div class='w-v100'>
		<button
			class='but-bl'
			type='button'
			disabled={false}
			onclick={goToFnsReports}
		>
			Отчеты для налоговой
		</button>
	</div>
</div>

<div class='w-100'>
	<div class='w-v100'>
		<button
			class='but-bl'
			type='button'
			disabled={false}
			onclick={goToHomeReports}
		>
			Внутренние управленческие отчеты
		</button>
	</div>
</div>