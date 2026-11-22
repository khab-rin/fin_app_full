<script lang='ts'>
	import {invoke} from '@tauri-apps/api/core'
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { dbStep } from '$lib/models/DataBase/DbManager.svelte';
	import type {DatabaseStep} from '$lib/models/rustModels/DatabaseStep';
	import type {ProgressBar} from '$lib/models/rustModels/ProgressBar';


	let isFullSyncPushed = $state(false);
	let syncLabel = $state('Подготовка к синхронизации..');
	let syncPercent = $state(0);

	async function syncDataBase() {
		if (isFullSyncPushed) {return;}
		isFullSyncPushed = true;
		syncLabel = 'Подготовка к синхронизации..';
		syncPercent = 0;

		let unlisten: UnlistenFn | undefined;

		try {
			unlisten = await listen<ProgressBar>('db_sync_progress', (event) => {
				syncLabel = event.payload.label;
				syncPercent = event.payload.percent;
			})

			const nextStep: DatabaseStep = await invoke<DatabaseStep>("cmd_sync_data_base", {});
			dbStep.step = nextStep;
		} catch(err) {
			console.error("cmd_sync_data_base FAILED, err = ", err);
			const nextStep: DatabaseStep = {TryLater: {text: 'Критическая ошибка на устройстве...'}};
			dbStep.step = nextStep;
		} finally {
			
			if (unlisten) {unlisten()}
			isFullSyncPushed = false;
		}
	}
</script>

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
			id='DbMainSync'
		>
			{#if isFullSyncPushed}
				<span class='loading-content'><span class='loading-spinner'></span>Синхронизация...</span>
			{:else}
				Синхронизировать
			{/if}
		
		</button>
	</div>
</div>





<div class="sync-progress">
    <div class="sync-progress-info">
        <span>{syncLabel}</span>
        <span>{syncPercent}%</span>
    </div>

    <div class="sync-progress-track">
        <div class="sync-progress-bar" style:width="{syncPercent}%"></div>
    </div>
</div>

