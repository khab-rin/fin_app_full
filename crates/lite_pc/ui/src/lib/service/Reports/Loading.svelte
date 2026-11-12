<script lang='ts'>
	import { reportManager } from '$lib/models/Reports/ReportsManager.svelte';
	import type { ReportStep } from '$lib/models/rustModels/ReportStep';
	import {invoke} from '@tauri-apps/api/core';


	let isFullSyncPushed = $state(false);

	async function syncDataBase() {
		if (isFullSyncPushed) {return;}
		isFullSyncPushed = true;
		try {
			let nextStep: ReportStep = await invoke<ReportStep>("cmd_sync_data_base", {});
			isFullSyncPushed = false;
			reportManager.step = nextStep;
		} catch(err) {
			console.error("cmd_sync_data_base FAILED, err = ", err);
			const nextStep: ReportStep = {TryLater: {text: 'Критическая ошибка на устройстве...'}};
			isFullSyncPushed = false;
			reportManager.step = nextStep;
		}
	}

	function goToFnsReports() {
		const nextStep: ReportStep = {FnsReports: {text: 'Выберите тип отчета в налоговую и период'}};
		reportManager.step = nextStep;
	}

	function goToHomeReports() {
		const nextStep: ReportStep = {HomeReports: {text: 'Выберите тип управленческого отчета и период'}};
		reportManager.step = nextStep;
	}


</script>

<section class='w-40'>
	<button
		class='but-gr'
		type='button'
		disabled={false}
		onclick={goToFnsReports}
	>
		Отчеты для налоговой
	</button>
</section>

<section class='w-40'>
	<button
		class='but-gr'
		type='button'
		disabled={false}
		onclick={goToHomeReports}
	>
		Внутренние управленческие отчеты
	</button>
</section>


<div class='w-40'>
	<div class='w-v100'>
		<label class='label-close' for='DbMainSync'>
			Синхронизация всей базы данных
		</label>
		<button
			class='but-gr'
			type='button'
			disabled={isFullSyncPushed}
			onclick={syncDataBase}
		>
			{#if isFullSyncPushed}
				<span class='loading-content'><span class='loading-spinner'>Синхронизация...</span></span>
			{:else}
				Синхронизировать
			{/if}
		
		</button>
	</div>
</div>