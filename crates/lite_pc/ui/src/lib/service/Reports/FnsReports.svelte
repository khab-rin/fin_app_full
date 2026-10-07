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


	let quats = $derived.by<number[]>(() => {
		if (!curType) return [];
		switch (curType) {
			case "Декларация УСН (Доходы 6%)":
			case "Декларация УСН (Доходы - Расходы 15%)":
				return [4];
			default:
				return [1,2,3];
		}
	});

	function selectQuat(numb: number) {
		selectedQuat = numb;
		(document.getElementById('reportsFnsQuates') as HTMLDialogElement)?.close()
	}

	
	let isCmdMakeFnsReportFilesPushed = $state(false);

	let fnsCode = new FieldValidator("Digits4_4", "");

	let isValid = $derived(
		isYearValid && curType != null && selectedQuat != null && fnsCode.isValid
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
	class='dial-top-l'
	id='reportsFnsAllTypes'
	onclick={dialogBackdrop}
>	
	<h4 class='t-bl t-fill w-g100 x-ce'>
		Выберите отчет в налоговую
	</h4>


	{#each allTypes as repType}
		<div class='w-100'>
			<button
				type='button'
				class='but-ye'
				onclick={() => selectType(repType)}
			>
				{repType}
			</button>
		</div>
	{/each}

</dialog>

<dialog
	class="dial-gor"
	id='reportsFnsQuates'
	onclick={dialogBackdrop}
>
	{#each quats as quat}
		<div>
			<button
				type='button'
				class='but-ye'
				onclick={()=>selectQuat(quat)}
			>	
				{quat}
			</button>
		</div>
	{/each}
</dialog>


<section class='w-40'>
	<div class=w-v100>
		<label class='label-close' for='ReportFnsSelectedType'>
			Выбранные тип отчета
		</label>
		<input
			type='text'
			class='input-gr'
			id='ReportFnsSelectedType'
			disabled={true}
			bind:value={curType}
			placeholder='Тип отчета не выбран'
		/>
		<button
			type='button'
			class='but-gr'
			disabled={false}
			onclick={()=>(document.getElementById('reportsFnsAllTypes') as HTMLDialogElement)?.showModal()}
		>
			Открыть список
		</button>
	</div>
</section>

<section class='w-40'>
	<div class='w-v33'>
		<label class='label-close' for='ReportFnsYear'>
			Введите год
		</label>
		<input
			type='number'
			class='input-gr'
			id='ReportFnsYear'
			bind:value={selectedYear}
			placeholder='0000'
			class:input-error={!isYearValid}
		/>
	</div>

	<div class='w-v33'>
		<label class='label-close' for='ReportFnsSelectedQuat'>
			Выберите квартал
		</label>
		<select
			id='ReportFnsSelectedQuat'
			class='input-gr'
			bind:value={selectedQuat}
			disabled={false}
			class:input-error={selectedQuat==null}
		>
			<option value={null} disabled selected>Выберите период</option>
			{#each quats as quat}
				<option value={quat}>
					{quat}
				</option>
			{/each}
		
		</select>
	</div>
	<div class='w-v33'>
		<label class="label-close" for="taxOrgIdent">
            4-значный номер налоговой
            <span class='input-tool' data-input-tool="если вы уверены что не меняли место регистрации организации и в вашей налоговой не происходило слияний\разделений с момента регистрации вашей организации (ип), то это первые 4 цифры инн. В противном случае посмотрите этот код в сданной отчетности">?</span>
        </label>
        <input
            id="taxOrgIdent"
            type="text"
            bind:value={fnsCode.value}
            disabled={false}
            placeholder="Введите 4-значный номер налоговой в которой Вы подаете отчетность"
            class="input-gr"
            class:input-error={!fnsCode.isValid}
        />
	</div>
</section>

<section class='w-40'>
	<div class='w-v100'>
		<button
			type='button'
			class='but-bl'
			disabled={!isValid}
			onclick={cmdMakeFnsReportFiles}
		>
			Сформировать отчет
		</button>
	</div>
</section>