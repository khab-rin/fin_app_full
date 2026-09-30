<script lang='ts'>
	import {invoke} from '@tauri-apps/api/core';
	import {onMount} from 'svelte';
	import type {FnsReportType} from '$lib/models/rustModels/FnsReportType';
	import { dialogBackdrop } from '$lib/rules/dialogBorders';
	import type { ReportStep } from '$lib/models/rustModels/ReportStep';
	import { reportManager } from '$lib/models/Reports/ReportsManager.svelte';
	import { FieldValidator } from '$lib/models/Auth/FieldValidator.svelte';

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

	
	let isCmdMakeFnsReportFilesPushed = $state(false);

	let fnsCode = new FieldValidator("Digits4_4", "");

	let isValid = $derived(
		isYearValid && curType != null && selectQuat != null && fnsCode.isValid
	);

	async function cmdMakeFnsReportFiles() {
		if (isCmdMakeFnsReportFilesPushed || !isValid) {return;}
		try {
			isCmdMakeFnsReportFilesPushed = true;
			let data = {
				reportType: curType,
				year: selectedYear,
				quat: selectedQuat,
				fnsCode: fnsCode.value
			};
			let nextStep: ReportStep = await invoke<ReportStep>(
				'cmd_make_fns_report_files',
				data
			);

			isCmdMakeFnsReportFilesPushed = false;
			reportManager.step = nextStep;
		} catch(err) {
			isCmdMakeFnsReportFilesPushed = false;
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

	<div>
        <label class="green-field-label" for="taxOrgIdent">
            4-значный номер налоговой
            <span class='input-tool' data-input-tool="если вы уверены что не меняли место регистрации организации и в вашей налоговой не происходило слияний\разделений с момента регистрации вашей организации (ип), то это первые 4 цифры инн. В противном случае посмотрите этот код в сданной отчетности">?</span>
        </label>
        <input
            id="taxOrgIdent"
            type="text"
            bind:value={fnsCode.value}
            disabled={false}
            placeholder="Введите 4-значный номер налоговой в которой Вы подаете отчетность"
            class="green-field"
            class:input-error={!fnsCode.isValid}
        />
        {#if !fnsCode.isValid}
            <span class="input-error-span">Некорректный номер</span>
        {/if}
    </div>

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