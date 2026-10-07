<script lang='ts'>
	import {invoke} from '@tauri-apps/api/core';
	import {dialogBackdrop} from '$lib/rules/dialogBorders';
	import {openDialogRight} from '$lib/rules/dialogBorders';
	import {fitText} from '$lib/rules/text';
	import { FieldValidator } from '$lib/models/Auth/FieldValidator.svelte';
	import { OperationSvelte } from '$lib/models/Operation/OperationSvelte.svelte';
	import {operStep} from '$lib/models/Operation/OperationManager.svelte';
	import type { Operation } from '$lib/models/rustModels/Operation';
	import type { OperationStep } from '$lib/models/rustModels/OperationStep';
	import type { Company } from '$lib/models/rustModels/Company';
	import type {Contract} from '$lib/models/rustModels/Contract';
	import { onMount } from 'svelte';


	let curOper = $state<OperationSvelte>(new OperationSvelte());
	let rustOperations: Operation[] = $state<Operation[]>([]);


	let isChangeCtrPtyPushed = $state(false);
	let kpp = new FieldValidator('Kpp', '');
	let compInn = new FieldValidator('CompInn', '');

	async function changeCtrpty() {
		if (isChangeCtrPtyPushed || !kpp.isValid || !compInn.isValid) {return;}
		isChangeCtrPtyPushed = true;
		try {
			await curOper.cmdChangeCtrPty(compInn.value, kpp.value);
			(document.getElementById('OperManualNewCtrptyDialog') as HTMLDialogElement)?.close();

		} catch(err) {
			const next_step: OperationStep = {
				TryLater:{text:'Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение'}
			}
			console.error("cmdChangeCtrPty FAILED, err = ", err);
			isChangeCtrPtyPushed = false;
			operStep.step = next_step;
		}
	}

	async function selectCtrPty(ctrPty: Company) {
		await curOper.selectCtrPty(ctrPty);
		(document.getElementById('operManualAllCompanys') as HTMLDialogElement)?.close();
	}

	let isNewContractPushed = $state(false);

	async function changeContract(contract: Contract) {
		await curOper.changeContract(contract);
		(document.getElementById('OperManualAllContracts') as HTMLDialogElement)?.close();
	}

	async function cmdAddNewContract() {
		if (isNewContractPushed) {return;}
		try {
			isNewContractPushed = true;
			await curOper.cmdAddNewContract();
			isNewContractPushed = false;
			(document.getElementById('OperManualNewContractDialgo') as HTMLDialogElement)?.close();
		} catch(err) {
			const nextStep: OperationStep = {
				TryLater: {text:'Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение'}
			};
			isNewContractPushed = false;
			operStep.step = nextStep;
		}	
	}


	let isAddOperationPushed = $state(false);
	async function addOperation() {
		if (isAddOperationPushed) {return;}
		isAddOperationPushed = true;

		try {
			let operation = curOper.makeRust();
			if (operation != null) {
				rustOperations.push(operation);
				await curOper.reset();
				compInn.asyncSet('');
				kpp.asyncSet('');
				isAddOperationPushed = false;
			}
		} catch(err) {
			console.error("addOperation FAILED, err = ", err);
			let nextStep: OperationStep = {TryLater :{text: 'Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение'}}
			isAddOperationPushed = false;
			operStep.step = nextStep;
		}
		
	}

	let isProcessOperationsPushed = $state(false);
	async function cmdProcessOperations() {
		if (isProcessOperationsPushed) {return;}
		isProcessOperationsPushed = true;
		try {
			const nextStep = await invoke<OperationStep>(
				'cmd_process_operations', 
				{optionOperations: rustOperations}
			);
			isProcessOperationsPushed = false;
			operStep.step = nextStep;
		} catch(err) {
			console.error('cmdProcessOperations FAILED, err = ', err);
			const next_step: OperationStep = {TryLater: {text: 'Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение'}};
			isProcessOperationsPushed = false;
			operStep.step = next_step;

		}
	}

	onMount(async() => {
		try {
			await Promise.all([
				curOper.cmdGetUserCompId(),
				curOper.cmdGetToday(),
				curOper.cmdGetAllCompanys(),
			]);

		} catch(err) {
			let nextStep:OperationStep = {
				TryLater: {text: 'Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение'}
				
			}
			console.error("cmdGetUserCompId FAILED, err = ", err);
			operStep.step = nextStep;
		}
	});


	
</script>

<section class='w-50'>
	<div class='w-v40'>
		<div>
			<label class='label-close' for='operManualCtrPtyName'>
				Выбранный контрагент
			</label>
			<input
				id='operManualCtrPtyName'
				class='input-gr'
				type='text'
				disabled={true}
				placeholder='Контрагент не выбран'
				value={curOper.ctrPty?.metadata.comp_name?.short_egrul_name ?? ''}
			/>
		</div>
	</div>
	<div class='w-v30'>
		<button
			type='button'
			class='but-ye'
			disabled={false}
			onclick={(e)=>openDialogRight(e, 'OperManualNewCtrptyDialog')}
		>
			<span class='t-fill' use:fitText>
				Добавить нового контрагента
			</span>
			
		</button>
	</div>
	<div class='w-v30'>
		<button
			type='button'
			class='but-ye'
			disabled={false}
			onclick={(e) => openDialogRight(e, "operManualAllCompanys")}
		>	
			<span class='t-fill' use:fitText>
				Выбрать контрагента
			</span>
		</button>
	</div>
</section>

<section class='w-50'>
	<div class='w-v30'>
		<label class='label-close' for='operManualDebet'>
			Дебет {curOper.debetStr}
		</label>
		<input
			class ='input-gr'
			type='text'
			id='operManualDebet'
			bind:value={curOper.data.debet.value}
			disabled={false}
			placeholder='Номер счета'
			class:input-error={!curOper.data.debet.isValid}
		/>
	</div>

	<div class='w-v30'>
		<label class='label-close' for='operManualCred'>
			Кредит {curOper.creditStr}
		</label>
		<input
			class ='input-gr'
			type='text'
			id='operManualCred'
			bind:value={curOper.data.credit.value}
			disabled={false}
			placeholder='Номер счета'
			class:input-error={!curOper.data.credit.isValid}
		/>
	</div>

	<div class='w-v20'>
		<label class='label-close' for='operManualAmnt'>
			Сумма операции
		</label>
		<input
			class ='input-gr'
			type='text'
			id='operManualAmnt'
			bind:value={curOper.data.amount.value}
			disabled={false}
			placeholder='Номер счета'
			class:input-error={!curOper.data.amount.isValid}
		/>
	</div>
	<div class='w-v20'>
		<label
			class='label-close' 
			for='OperManualOperDate'
		>
			Дата операции
		</label>
		<input
			type='text'
			class='input-gr'
			id='OperManualOperDate'
			placeholder='00.00.0000'
			bind:value={curOper.data.operDate.value}
			class:input-error={!curOper.data.operDate.isValid}
		/>
	</div>
</section>


<div class='w-50'>
	<div class='w-v50'>
		<label class='label-close' for='operManualContrInfo'>Информация о договоре</label>
		<input
			class='input-gr'
			type='text'
			id='operManualContrInfo'
			disabled={true}
			placeholder='без договора'
			bind:value={curOper.contrStr}
		/>
	</div>

	<div class='w-v25'>
		<button
			type='button'
			class='but-ye'
			disabled={false}
			onclick={(e)=>openDialogRight(e, 'OperManualAllContracts')}
		>
			Список договоров
		</button>
	</div>

	<div class='w-v25'>
		<button
			type='button'
			class='but-ye'
			disabled={false}
			onclick={(e)=> openDialogRight(e, 'OperManualNewContractDialgo')}
		>
			Создать договор
		</button>
	</div>
</div>

<div class='w-50'>
	<div class='w-v50'>
		<label
			class='label-close' 
			for='OperManuelDocType'
		>
			Тип первичного документа
		</label>
		<input
			type='text'
			class='input-gr'
			id='OperManuelDocType'
			placeholder='строка до 50 знаков'
			bind:value={curOper.data.docType.value}
			class:input-error={!curOper.data.docType.isValid}
		/>
	</div>

	<div class='w-v25'>
		<label
			class='label-close' 
			for='OperManuelDocNum'
		>
			Номер первичного документа
		</label>
		<input
			type='text'
			class='input-gr'
			id='OperManuelDocNum'
			placeholder='строка до 50 знаков'
			bind:value={curOper.data.docNum.value}
			class:input-error={!curOper.data.docNum.isValid}
		/>
	</div>

	<div class='w-v25'>
		<label
			class='label-close' 
			for='OperManuelDocDate'
		>
			Дата первичного документа
		</label>
		<input
			type='text'
			class='input-gr'
			id='OperManuelDocDate'
			placeholder='00.00.0000'
			bind:value={curOper.data.docDate.value}
			class:input-error={!curOper.data.docDate.isValid}
		/>
	</div>

</div>

<div class='w-25'>
	<div class='w-v100'>
		<label for='OperManualIsDupl'>
			Признак дуприката
		</label>
		<input
			class='input-gr'
			type='text'
			id='OperManualIsDupl'
			bind:value={curOper.isDuplicateStr}
			disabled={true}
		/>

	</div>
</div>

<section class='w-50'>
	<div class='w-v50'>
		<button
			type='button'
			class='but-bl'
			disabled={curOper.isValid || isAddOperationPushed}	
			onclick={addOperation}
		>
			сформировать операцию
		</button>
	</div>

	<div class='w-v50'>
		<button
			type='button'
			class='but-bl'
			disabled={false}	
			onclick={cmdProcessOperations}
		>
			Загрузить операции
		</button>
	</div>
</section>


<dialog 
	class='dial-ver'
	id='operManualAllCompanys'
>
	<section class='w-100'>
		<div class='w-v100'>
			<h4 class='t-fill w-g100 x-ce'>
				Выберите контрагента
			</h4>
		</div>
	</section>

	{#each curOper.allCtrPtys as ctrPty}
		<section class='w-100'>
			<div class='w-v100'>
				<button
					type='button'
					class='but-ye'
					disabled={false}
					onclick={()=> selectCtrPty(ctrPty)}

				>
					<span class='t-fill' use:fitText>
						{ctrPty.metadata.comp_name?.short_egrul_name ?? ""}
					</span>
					
				</button>
			</div>
		</section>	
	{/each}

	<section class='w-100'>
		<div class='w-v100'>
			<button
				type='button'
				class='but-bl'
				disabled={false}
				onclick={()=>(document.getElementById('operManualAllCompanys') as HTMLDialogElement)?.close()}
			>
				Закрыть окно
			</button>
		</div>
	</section>
</dialog>


<dialog
	class='dial-gor'
	id='OperManualNewCtrptyDialog'
>
	<div class='w-v40'>
		<label class='label-close' for='operManualNewCtrPryInn'>
			Инн организации
		</label>
		<input
			class='input-ye'
			id='operManualNewCtrPryInn'
			type='text'
			placeholder='10 | 12 цифр'
			bind:value={compInn.value}
			class:input-error={!compInn.isValid}
		/>
	</div>

	<div class='w-v40'>
		<label class='label-close' for='operManualNewCtrPryKpp'>
			Кпп орназизации
		</label>
		<input
			class='input-ye'
			id='operManualNewCtrPryKpp'
			type='text'
			placeholder='10 | 12 цифр'
			bind:value={kpp.value}
			class:input-error={!kpp.isValid}
		/>
	</div>

	<div class='w-v20'>
		<button
			type='button'
			class='but-ye'
			disabled={!compInn.isValid || !kpp.isValid}
			onclick={changeCtrpty}
		>
			Добавить нового контрагента
		</button>
	</div>
</dialog>


<dialog 
	class='dial-ver'
	id='OperManualAllContracts'
>
	<section class='w-100'>
		<span class='w-v100 t-fill x-ce'>Выберите договор</span>
	</section>
	{#each curOper.allPossContracts as contract}
		<section class='w-100'>
			<div class='w-g100'>
				<button
					type='button'
					class='but-ye'
					onclick={()=>changeContract(contract)}
				>
					<span class='t-fill x-ce' use:fitText></span>
					{curOper.anyContractStr(contract)}
				</button>
			</div>
		</section>
	{/each}

</dialog>

<dialog
	class='dial-ver'
	id='OperManualNewContractDialgo'
>
	<div class='w-100'>
		<g4 class='w-g100 t-fill x-ce'>Введите данные нового договора</g4>
	</div>
	
	<div class='w-100'>
		<div class='w-v100'>
			<label class='label-close' for='OperManualNewContrNum'>
				Номер договора
			</label>
			<input
				type='text'
				class='input-ye'
				id='OperManualNewContrNum'
				bind:value={curOper.newContrData.contractNum.value}
				placeholder='Строка до 50 знаков'
				class:input-error={!curOper.newContrData.contractNum.isValid}
			/>
		</div>
	</div>
	<div class='w-100'>
		<div class='w-v100'>
			<label class='label-close' for='OperManualNewContrDate'>
				Дата договора
			</label>
			<input
				type='text'
				class='input-ye'
				id='OperManualNewContrDate'
				bind:value={curOper.newContrData.contractDate.value}
				placeholder='00.00.0000'
				class:input-error={!curOper.newContrData.contractDate.isValid}
			/>
		</div>
	</div>
	<div class='w-100'>
		<div class='w-v100'>
			<label class='label-close' for='OperManualNewContrTittle'>
				Название договора
			</label>
			<input
				type='text'
				class='input-ye'
				id='OperManualNewContrTittle'
				bind:value={curOper.newContrData.contractTitle.value}
				placeholder='Строка до 50 знаков'
				class:input-error={!curOper.newContrData.contractTitle.isValid}
			/>
		</div>
	</div>
	<div class='w-100'>
		<div class='w-v100'>
			<label class='label-close' for='OperManualNewContrStFDate'>
				Дата начала
			</label>
			<input
				type='text'
				class='input-ye'
				id='OperManualNewContrStFDate'
				bind:value={curOper.newContrData.contractStDate.value}
				placeholder='00.00.0000'
				class:input-error={!curOper.newContrData.contractStDate.isValid}
			/>
		</div>
	</div>
	<div class='w-100'>
		<div class='w-v100'>
			<label class='label-close' for='OperManualNewContrEndFDate'>
				Дата окончания
			</label>
			<input
				type='text'
				class='input-ye'
				id='OperManualNewContrEndFDate'
				bind:value={curOper.newContrData.contractEndDate.value}
				placeholder='00.00.0000'
				class:input-error={!curOper.newContrData.contractEndDate.isValid}
			/>
		</div>
	</div>
	<div class='w-100'>
		<div class='w-v100'>
			<label class='label-close' for='OperManualNewContrCurrency'>
				Валюта договора
			</label>
			<input
				type='text'
				class='input-ye'
				id='OperManualNewContrCurrency'
				bind:value={curOper.newContrData.contractCurrency.value}
				placeholder='РУБ'
				class:input-error={!curOper.newContrData.contractCurrency.isValid}
			/>
		</div>
	</div>

	<div class='w-100'>
		<div class='w-v100'>
			<label class='label-close' for='OperManualNewContramnt'>
				Сумма договора
			</label>
			<input
				type='text'
				class='input-ye'
				id='OperManualNewContramnt'
				bind:value={curOper.newContrData.contractTotAmnt.value}
				placeholder='Сумма в валюте договора'
				class:input-error={!curOper.newContrData.contractTotAmnt.isValid}
			/>
		</div>
	</div>

	<div class='w-100'>
		<div class='w-v100'>
			<label class='label-close' for='OperManualNewContrDeffDays'>
				Рассрочка в
			</label>
			<input
				type='text'
				class='input-ye'
				id='OperManualNewContrDeffDays'
				bind:value={curOper.newContrData.contractDefDays.value}
				placeholder='Сумма в валюте договора'
				class:input-error={!curOper.newContrData.contractDefDays.isValid}
			/>
		</div>
	</div>

	<div class='w-100'>
		<div class='w-v100'>
			<label class='label-close' for='OperManualNewContrDeffDays'>
				Рассрочка в днях
			</label>
			<input
				type='text'
				class='input-ye'
				id='OperManualNewContrDeffDays'
				bind:value={curOper.newContrData.contractDefDays.value}
				placeholder='Количество дней'
				class:input-error={!curOper.newContrData.contractDefDays.isValid}
			/>
		</div>
	</div>

	<div class='w-100'>
		<div class='w-v100'>
			<label class='label-close' for='OperManualNewContrDescr'>
				Описание договора
			</label>
			<input
				type='text'
				class='input-ye'
				id='OperManualNewContrDescr'
				bind:value={curOper.newContrData.contractDescr.value}
				placeholder='Количество дней'
				class:input-error={!curOper.newContrData.contractDescr.isValid}
			/>
		</div>
	</div>

	<div class='w-100'>
		<div class='w-v100'>
			<button 
				class='but-ye'
				type='button'
				onclick={cmdAddNewContract}
				disabled={curOper.isNewContractValid || isNewContractPushed}
			>
				Добавить договор
			</button>
		</div>
	</div>
</dialog>