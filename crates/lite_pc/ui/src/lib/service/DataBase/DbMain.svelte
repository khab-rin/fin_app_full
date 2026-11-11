<script lang='ts'>
	import {invoke} from '@tauri-apps/api/core'

	let isFullSyncPushed = $state(false);

	async function syncDataBase() {
		isFullSyncPushed = true;

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
		>
			{#if isFullSyncPushed}
				<span class='loading-content'><span class='loading-spinner'>Синхронизация...</span></span>
			{:else}
				Синхронизировать
			{/if}
		
		</button>
	</div>
</div>



<div class="sync-progress">
    <div class="sync-progress-info">
        <span>Синхронизация базы данных...</span>
        <span>47%</span>
    </div>

    <div class="sync-progress-track">
        <div class="sync-progress-bar" style="width: 47%"></div>
    </div>
</div>

