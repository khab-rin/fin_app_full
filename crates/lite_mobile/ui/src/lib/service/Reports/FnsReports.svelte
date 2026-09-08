<script lang='ts'>
	import {invoke} from '@tauri-apps/api/core';
	import {onMount} from 'svelte';
	import type {FnsReportType} from '$lib/models/rustModels/FnsReportType';
	import { dialogBackdrop } from '$lib/rules/dialogBorders';

	let allTypes = $state<FnsReportType[]>([]);
	let curType = $state<FnsReportType | null>(null);


	function selectType(repType: FnsReportType) {
		curType = repType;
		(document.getElementById('reportsFnsAllTypes') as HTMLDialogElement)?.close()
	}


	onMount(async() => {
		allTypes = await invoke<FnsReportType[]>(
			'cmd_get_all_all_fns_report_types',
			{}
		)
	});
</script>


<dialog
	class='dialog-top-left'
	id='reportsFnsAllTypes'
	onclick={dialogBackdrop}
>
	<section class='group-one'>
		<span class='yellow-field-span'>
			Выберите отчет в налоговую
		</span>
		{#each allTypes as repType}
			<button
				type='button'
				class='yellow-button'
				onclick={() => selectType(repType)}
			>
				{repType}
			</button>
		{/each}
	</section>
</dialog>

<section class='group-one'>
	<div>
		<label class='green-field-label' for='ReportFnsSelectedType'>
			Выбранные тип отчета
		</label>
		<input
			type='text'
			class='green-field'
			id='ReportFnsSelectedType'
			bind:value={curType}
			placeholder='Тип отчета не выбран'
		/>
		<button
			type='button'
			class='green-button'
			disabled={false}
			onclick={()=>(document.getElementById('reportsFnsAllTypes') as HTMLDialogElement)?.showModal()}
		>
			Открыть список
		</button>

	</div>
</section>