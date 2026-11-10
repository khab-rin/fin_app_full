<script lang='ts'>
	import {invoke} from '@tauri-apps/api/core';
	import {fitText} from '$lib/rules/text';
	import {dialogBackdrop} from '$lib/rules/dialogBorders';
	
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


	let changeCtrptyPushed = $state(false);
	let kpp = new FieldValidator('Kpp', '');
	let compInn = new FieldValidator('CompInn', '');

	let isNewContractPushed = $state(false);

	function openModal(id: string) {
		(document.getElementById(id) as HTMLDialogElement)?.showModal();
	}

	function closeModal(id: string) {
		(document.getElementById(id) as HTMLDialogElement)?.close();
	}

	async function changeCtrpty(id: string) {
		if (changeCtrptyPushed || !kpp.isValid || !compInn.isValid) {return;}
		changeCtrptyPushed = true;
		try {
			await curOper.cmdChangeCtrPty(compInn.value, kpp.value);
			closeModal(id);
			changeCtrptyPushed = false;
		} catch(err) {
			const next_step: OperationStep = {
				TryLater:{text:'Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение'}
			}
			console.error("cmdChangeCtrPty FAILED, err = ", err);
			closeModal(id);
			changeCtrptyPushed = false;
			operStep.step = next_step;
		}
	}

	async function selectCtrPty(id: string, ctrPty: Company) {
		await curOper.selectCtrPty(ctrPty);
		closeModal(id);
	}

	
	async function changeContract(id: string, contract: Contract) {
		await curOper.changeContract(contract);
		closeModal(id);
	}

	async function cmdAddNewContract(id: string) {
		if (isNewContractPushed) {
			closeModal(id);
			return;
		}
		if (!curOper.ctrPty) {
			operStep.currentText = "Выберите контрагента для добавления договора";
			closeModal(id);
			return;
		}
		isNewContractPushed = true;
		try {
			await curOper.cmdAddNewContract();
			isNewContractPushed = false;
			closeModal(id);
			isNewContractPushed = false;
		} catch(err) {
			const nextStep: OperationStep = {
				TryLater: {text:'Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение'}
			};
			isNewContractPushed = false;
			closeModal(id);
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

<div class='w-100'>
	<div class='w-v100'>
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

<div class='w-100'>
	<div class='w-v100'>
		<button
			type='button'
			class='but-ye'
			disabled={false}
			onclick={()=>openModal("OperManInpNewCtrptyDialog")}
		>
			<span>
				Добавить нового контрагента
			</span>
		</button>
	</div>
</div>

<div class='w-100'>
	<div class='w-v100'>
		<button
			type='button'
			class='but-ye'
			disabled={false}
			onclick={()=>openModal("operManInpAllCompanys")}
		>	
			<span>
				Выбрать контрагента
			</span>
		</button>
	</div>
</div>

<div class='w-100'>
	<div class='w-v50'>
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

	<div class='w-v50'>
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
</div>

<div class='w-100'>
	<div class='w-v50'>
		<label class='label-close' for='operManualAmnt'>
			Сумма операции
		</label>
		<input
			class ='input-gr'
			type='text'
			id='operManualAmnt'
			bind:value={curOper.data.amount.value}
			disabled={false}
			placeholder='Сумма операции'
			class:input-error={!curOper.data.amount.isValid}
		/>
	</div>
	<div class='w-v50'>
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
</div>


<div class='w-100'>
	<div class='w-v100'>
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
</div>

<div class='w-100'>
	<div class='w-v50'>
		<button
			type='button'
			class='but-ye'
			disabled={false}
			onclick={()=>openModal("operManInpContrDial")}
		>
			Список договоров
		</button>
	</div>

	<div class='w-v50'>
		<button
			type='button'
			class='but-ye'
			disabled={false}
			onclick={()=>openModal("OperManInpNewContractDialog")}
		>
			Создать договор
		</button>
	</div>
</div>

<div class='w-100'>
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

	<div class='w-v50'>
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
</div>

<div class='w-100'>
	<div class='w-v50'>
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

	<div class='w-v50'>
		<label class='label-close' for='OperManualIsDupl'>
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

<div class='w-100'>
	<div class='w-v50'>
		<button
			type='button'
			class='but-gr'
			disabled={!curOper.isValid || isAddOperationPushed}	
			onclick={addOperation}
		>
			сформировать операцию
		</button>
	</div>

	<div class='w-v50'>
		<button
			type='button'
			class='but-gr'
			disabled={isProcessOperationsPushed}	
			onclick={cmdProcessOperations}
		>
			Загрузить операции
		</button>
	</div>
</div>


<dialog 
	class='dial-t-l' 
	id='operManInpContrDial'
	onclick={dialogBackdrop}
>
	<div class='w-100'>
		<span class='x-self-c x-cont-c with-info'>
			Выберите договор
		</span>
	</div>

	{#each curOper.allPossContracts as contract}
		<div class='w-100'>
			<div class='w-v100'>
				<button
					class='but-ye'
					type='button'
					onclick={()=>changeContract("operStatSuccContrDial", contract)}
				>
					<span class='x-self-c x-cont-c t-fill' use:fitText>
						{curOper.anyContractStr(contract)}
					</span>
				</button>
			</div>
		</div>
	{/each}

	{#if curOper.allPossContracts.length == 0}
		<div class='w-100'>
			<span class='x-self-c x-cont-c with-info'>
				Отсутствуют сохраненные договора
			</span>
		</div>
	{/if}

	<div class='w-100'>
		<div class='w-v100'>
			<button
				class='but-ye'
				type='button'
				onclick={()=>closeModal("operManInpContrDial")}
			>
				<span class='x-self-c x-cont-c t-fill' use:fitText>
					Отмена
				</span>
			</button>
		</div>
	</div>
</dialog>

<dialog
	class='dial-t-l'
	id='OperManInpNewContractDialog'
	onclick={dialogBackdrop}
>
	
	<div class='w-100'>
		<p class='x-self-c x-cont-c t-fill with-info'>Введите данные нового договора</p>
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
				placeholder='Краткое описание'
				class:input-error={!curOper.newContrData.contractDescr.isValid}
			/>
		</div>
	</div>
	<div class='w-100'>
		<div class='w-v100'>
			<button 
				class='but-ye'
				type='button'
				onclick={()=>cmdAddNewContract("OperManInpNewContractDialog")}
				disabled={isNewContractPushed || !curOper.isNewContractValid || isNewContractPushed}
			>
				{#if isNewContractPushed}
					<span class='loading-content'>
						<span class='loading-spinner'></span>
						Добавление
					</span>
				{:else}
					Добавить договор
				{/if}
				
			</button>
		</div>
	</div>
</dialog>


<dialog 
	class='dial-t-l'
	id='operManInpAllCompanys'
	onclick={dialogBackdrop}
>
	<div class='w-100'>
		<p class='x-self-c x-cont-c with-info'>
			Выберите контрагента
		</p>
	</div>

	{#each curOper.allCtrPtys as ctrPty}
		<div class='w-100'>
			<div class='w-v100'>
				<button
					type='button'
					class='but-ye'
					disabled={false}
					onclick={()=> selectCtrPty("operManInpAllCompanys", ctrPty)}

				>
					<span class='t-fill' use:fitText>
						{ctrPty.metadata.comp_name?.short_egrul_name ?? ""}
					</span>
				</button>
			</div>
		</div>	
	{/each}

	<div class='w-100'>
		<div class='w-v100'>
			<button
				type='button'
				class='but-ye'
				disabled={false}
				onclick={()=>closeModal("operManInpAllCompanys")}
			>
				Закрыть окно
			</button>
		</div>
	</div>
</dialog>


<dialog
	class='dial-t-l'
	id='OperManInpNewCtrptyDialog'
	onclick={dialogBackdrop}
>
	<div class='w-100'>
		<p class="x-self-c t-fill with-info">
			Введите реквизиты
		</p>
	</div>
	<div class='w-100'>
		<div class='w-v100'>
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
	</div>
	<div class='w-100'>
		<div class='w-v100'>
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
	</div>

	<div class='w-100'>
		<div class='w-v100'>
			<button
				type='button'
				class='but-ye'
				disabled={changeCtrptyPushed || !compInn.isValid || !kpp.isValid}
				onclick={() => changeCtrpty("OperManInpNewCtrptyDialog")}
			>
				{#if changeCtrptyPushed}
					<span class='loading-content'>
						<span class='loading-spinner'></span>
						Добавление
					</span>
				{:else}
					Добавить нового контрагента
				{/if}
			</button>
		</div>
	</div>
</dialog>