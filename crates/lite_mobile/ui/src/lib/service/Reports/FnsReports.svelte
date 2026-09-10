<script lang='ts'>
	import {invoke} from '@tauri-apps/api/core';
	import {onMount} from 'svelte';
	import type {FnsReportType} from '$lib/models/rustModels/FnsReportType';
	import { dialogBackdrop } from '$lib/rules/dialogBorders';
	import type { ReportStep } from '$lib/models/rustModels/ReportStep';
	import { reportManager } from '$lib/models/Reports/ReportsManager.svelte';

	let allTypes = $state<FnsReportType[]>([]);
	let curType = $state<FnsReportType | null>(null);

	function selectType(repType: FnsReportType) {
		curType = repType;
		(document.getElementById('reportsFnsAllTypes') as HTMLDialogElement)?.close()
	}

	let selectedYear = $state<number | null>(null);
	let curYear = 2026;
	let minYear = 2000;

	let isYearValid = $derived(
        selectedYear !== null && selectedYear >= minYear && selectedYear <= curYear
    );

	let selectedQuat = $state<number | null>(null);
	const quats: number[] = [1, 2, 3, 4];

	function selectQuat(numb: number) {
		selectedQuat = numb;
		(document.getElementById('reportsFnsAllQuats') as HTMLDialogElement)?.close()
	}

	let isValid = $derived(
		isYearValid && curType != null && selectQuat != null
	);
	let isCmdMakeFnsReportFilesPushed = $state(false);

	async function cmdMakeFnsReportFiles() {
		if (isCmdMakeFnsReportFilesPushed || !isValid) {return;}
		try {
			let data = {
				reportType: curType,
				year: selectedYear,
				quat: selectedQuat
			};
			let nextStep: ReportStep = await invoke<ReportStep>(
				'cmd_make_fns_report_files',
				data
			);
			reportManager.step = nextStep;
		} catch(err) {
			console.error("cmdMakeFnsReportFiles FAILED, err = ", err);
			const nextStep: ReportStep = {TryLater: {text: 'Критическая ошибка на устройстве...'}};
			reportManager.step = nextStep;
		}
	}


	onMount(async() => {
		allTypes = await invoke<FnsReportType[]>(
			'cmd_get_all_fns_report_types',
			{}
		);
		curYear = await invoke<number>(
			'cmd_get_current_year',
			{}
		);
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

<dialog
	class='dialog-top-left'
	id='reportsFnsAllQuats'
	onclick={dialogBackdrop}
>
	<section class='group-one'>
		<span class='yellow-field-span'>
			Выберите квартал
		</span>
		{#each quats as quat}
			<button
				type='button'
				class='yellow-button'
				onclick={() => selectQuat(quat)}
			>
				{quat}
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
			disabled={true}
			bind:value={curType}
			placeholder='Тип отчета не выбран'
		/>
	</div>
	
	<button
		type='button'
		class='green-button'
		disabled={false}
		onclick={()=>(document.getElementById('reportsFnsAllTypes') as HTMLDialogElement)?.showModal()}
	>
		Открыть список
	</button>
</section>

<section class='group-one'>
	<div>
		<label class='green-field-label' for='ReportFnsYear'>
			Введите год
		</label>
		<input
			type='number'
			class='green-field'
			id='ReportFnsYear'
			bind:value={selectedYear}
			placeholder='0000'
			class:input-error={!isYearValid}
		/>
	</div>
</section>


<section class='group-one'>
	<div>
		<label class='green-field-label' for='ReportFnsSelectedQuat'>
			Выберите квартал
		</label>
		<input
			type='text'
			class='green-field'
			id='ReportFnsSelectedQuat'
			disabled={true}
			bind:value={selectedQuat}
			placeholder='0'
		/>
	</div>
	
	<button
		type='button'
		class='green-button'
		disabled={false}
		onclick={()=>(document.getElementById('reportsFnsAllQuats') as HTMLDialogElement)?.showModal()}
	>
		Открыть список
	</button>
</section>

<section class='group-one'>
	<div>
		<label class='green-field-label' for='ReportFnsYear'>
			Формирование отчета
		</label>
		<button
			type='button'
			class='blue-button'
			disabled={!isValid}
			onclick={cmdMakeFnsReportFiles}
		>
			Сформировать отчет
		</button>
	</div>
</section>