<script lang='ts'>
	import {invoke} from '@tauri-apps/api/core';
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

	let isCtrPtyOpen = $state(false);
	let isChangeCtrPtyPushed = $state(false);
	let kpp = new FieldValidator('Kpp', '');
	let compInn = new FieldValidator('CompInn', '');
	let allCtrPtys: Company[] = [];
	function openCtrpty() {isCtrPtyOpen = !isCtrPtyOpen;}
	async function changeCtrpty() {
		if (isChangeCtrPtyPushed || !kpp.isValid || !compInn.isValid) {return;}
		isChangeCtrPtyPushed = true;
		try {
			await curOper.cmdChangeCtrPty(compInn.value, kpp.value);
			if (curOper.ctrPty != null) {
				allCtrPtys.push(curOper.ctrPty);
			}
			isCtrPtyOpen = false;
			isChangeCtrPtyPushed = false;
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
		isCtrPtyOpen = false;
		(document.getElementById('operManualAllCompanys') as HTMLDialogElement)?.close();
	}

	let isContractOpen = $state(false);
	let isNewContractOpen = $state(false);
	let isChangeContractopen = $state(false);
	let isNewContractPushed = $state(false);
	function openContract() {
		isContractOpen = !isContractOpen;
		isNewContractOpen = false;
		isChangeContractopen = false;
	}
	function openNewContract() {
		isNewContractOpen = !isNewContractOpen;
		isChangeContractopen = false;
	}
	function openChangeContract() {
		isChangeContractopen = !isChangeContractopen;
		isNewContractOpen = false;
	}

	function changeContract(contract: Contract) {
		curOper.changeContract(contract);
		isContractOpen = false;
		isNewContractOpen = false;
		isChangeContractopen = false;
	}

	async function cmdAddNewCpntract() {
		if (isNewContractPushed) {return;}
		try {
			isNewContractPushed = true;
			await curOper.cmdAddNewContract();
			isNewContractPushed = false;
			isNewContractOpen = false;
			isContractOpen = true;
			isChangeContractopen = true;
		} catch(err) {
			const nextStep: OperationStep = {
				TryLater: {text:'Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение'}
			};
			isNewContractPushed = false;
			isNewContractOpen = false;
			isContractOpen = false;
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


<div class='group-one'>
	<div>
		<label class='green-field-label' for='operManualCtrPtyName'>
			Выбранный контрагент
		</label>
		<input
			id='operManualCtrPtyName'
			class='green-field'
			type='text'
			disabled={true}
			placeholder='Контрагент не выбран'
			value={curOper.ctrPty?.metadata.comp_name?.short_egrul_name ?? ''}
		/>
	</div>

	<button 
		class='yellow-button'
		type='button'
		disabled={false}
		onclick={openCtrpty}
	>
		Сменить контрагента
	</button>

	{#if isCtrPtyOpen}
		<div>
			<label class='yellow-field-label' for='operManualNewCtrPryInn'>
				Инн организации
			</label>
			<input
				class='yellow-field'
				id='operManualNewCtrPryInn'
				type='text'
				placeholder='10 | 12 цифр'
				bind:value={compInn.value}
				class:input-error={!compInn.isValid}
			/>
		</div>
		
		<div>
			<label class='yellow-field-label' for='operManualNewCtrPryKpp'>
				Кпп орназизации
			</label>
			<input
				class='yellow-field'
				id='operManualNewCtrPryKpp'
				type='text'
				placeholder='10 | 12 цифр'
				bind:value={kpp.value}
				class:input-error={!kpp.isValid}
			/>
		</div>
		
		<button
			type='button'
			class='yellow-button'
			disabled={!compInn.isValid || !kpp.isValid}
			onclick={changeCtrpty}
		>
			Добавить нового контрагента
		</button>

		<button
			type='button'
			class='yellow-button'
			disabled={false}
			onclick={() => (document.getElementById('operManualAllCompanys') as HTMLDialogElement)?.showModal()}
		>
			Выбрать контрагента
		</button>
	{/if}
</div>

<div class='group-one'>
	<div>
		<label class='green-field-label' for='operManualDebet'>
			Дебет {curOper.debetStr}
		</label>
		<input
			class ='green-field'
			type='text'
			id='operManualDebet'
			bind:value={curOper.data.debet.value}
			disabled={false}
			placeholder='Номер счета'
			class:input-error={!curOper.data.debet.isValid}
		/>
	</div>

	<div>
		<label class='green-field-label' for='operManualCred'>
			Кредит {curOper.creditStr}
		</label>
		<input
			class ='green-field'
			type='text'
			id='operManualCred'
			bind:value={curOper.data.credit.value}
			disabled={false}
			placeholder='Номер счета'
			class:input-error={!curOper.data.credit.isValid}
		/>
	</div>

	<div>
		<label class='green-field-label' for='operManualAmnt'>
			Сумма операции
		</label>
		<input
			class ='green-field'
			type='text'
			id='operManualAmnt'
			bind:value={curOper.data.amount.value}
			disabled={false}
			placeholder='Номер счета'
			class:input-error={!curOper.data.amount.isValid}
		/>
	</div>
</div>

<div class='group-one'>
	<div>
		<label class='green-field-label' for='operManualContrInfo'>Информация о договоре</label>
		<input
			class='green-field'
			type='text'
			id='operManualContrInfo'
			disabled={true}
			placeholder='без договора'
			bind:value={curOper.contrStr}
		/>
	</div>

	{#if isContractOpen}
		{#if isChangeContractopen}
			<section class='group-one'>
				<span class='yellow-field-span'>Выберите договор</span>
				{#each curOper.allPossContracts as contract}
					<button
						type='button'
						class='yellow-button'
						onclick={()=>changeContract(contract)}
					>
						{curOper.anyContractStr(contract)}
					</button>
				{/each}
			</section>
		{/if}

		<button
			type='button'
			class='yellow-button'
			disabled={false}
			onclick={openChangeContract}
		>
			Список договоров
		</button>

		{#if isNewContractOpen}
			<div class='group-one'>
				<span>Введите данные нового договра</span>
				<div>
					<label class='yellow-field-label' for='OperManualNewContrNum'>
						Номер договора
					</label>
					<input
						type='text'
						class='yellow-field'
						id='OperManualNewContrNum'
						bind:value={curOper.newContrData.contractNum.value}
						placeholder='Строка до 50 знаков'
						class:input-error={!curOper.newContrData.contractNum.isValid}
					/>
				</div>

				<div>
					<label class='yellow-field-label' for='OperManualNewContrDate'>
						Дата договора
					</label>
					<input
						type='text'
						class='yellow-field'
						id='OperManualNewContrDate'
						bind:value={curOper.newContrData.contractDate.value}
						placeholder='00.00.0000'
						class:input-error={!curOper.newContrData.contractDate.isValid}
					/>
				</div>

				<div>
					<label class='yellow-field-label' for='OperManualNewContrTittle'>
						Название договора
					</label>
					<input
						type='text'
						class='yellow-field'
						id='OperManualNewContrTittle'
						bind:value={curOper.newContrData.contractTitle.value}
						placeholder='Строка до 50 знаков'
						class:input-error={!curOper.newContrData.contractTitle.isValid}
					/>
				</div>

				<div>
					<label class='yellow-field-label' for='OperManualNewContrStFDate'>
						Дата начала
					</label>
					<input
						type='text'
						class='yellow-field'
						id='OperManualNewContrStFDate'
						bind:value={curOper.newContrData.contractStDate.value}
						placeholder='00.00.0000'
						class:input-error={!curOper.newContrData.contractStDate.isValid}
					/>
				</div>

				<div>
					<label class='yellow-field-label' for='OperManualNewContrEndFDate'>
						Дата окончания
					</label>
					<input
						type='text'
						class='yellow-field'
						id='OperManualNewContrEndFDate'
						bind:value={curOper.newContrData.contractEndDate.value}
						placeholder='00.00.0000'
						class:input-error={!curOper.newContrData.contractEndDate.isValid}
					/>
				</div>

				<div>
					<label class='yellow-field-label' for='OperManualNewContrCurrency'>
						Валюта договора
					</label>
					<input
						type='text'
						class='yellow-field'
						id='OperManualNewContrCurrency'
						bind:value={curOper.newContrData.contractCurrency.value}
						placeholder='РУБ'
						class:input-error={!curOper.newContrData.contractCurrency.isValid}
					/>
				</div>

				<div>
					<label class='yellow-field-label' for='OperManualNewContramnt'>
						Сумма договора
					</label>
					<input
						type='text'
						class='yellow-field'
						id='OperManualNewContramnt'
						bind:value={curOper.newContrData.contractTotAmnt.value}
						placeholder='Сумма в валюте договора'
						class:input-error={!curOper.newContrData.contractTotAmnt.isValid}
					/>
				</div>

				<div>
					<label class='yellow-field-label' for='OperManualNewContrDeffDays'>
						Рассрочка в
					</label>
					<input
						type='text'
						class='yellow-field'
						id='OperManualNewContrDeffDays'
						bind:value={curOper.newContrData.contractDefDays.value}
						placeholder='Сумма в валюте договора'
						class:input-error={!curOper.newContrData.contractDefDays.isValid}
					/>
				</div>

				<div>
					<label class='yellow-field-label' for='OperManualNewContrDeffDays'>
						Рассрочка в днях
					</label>
					<input
						type='text'
						class='yellow-field'
						id='OperManualNewContrDeffDays'
						bind:value={curOper.newContrData.contractDefDays.value}
						placeholder='Количество дней'
						class:input-error={!curOper.newContrData.contractDefDays.isValid}
					/>
				</div>

				<div>
					<label class='yellow-field-label' for='OperManualNewContrDescr'>
						Описание договора
					</label>
					<input
						type='text'
						class='yellow-field'
						id='OperManualNewContrDescr'
						bind:value={curOper.newContrData.contractDescr.value}
						placeholder='Количество дней'
						class:input-error={!curOper.newContrData.contractDescr.isValid}
					/>
				</div>

				<button 
					class='yellow-button'
					type='button'
					onclick={cmdAddNewCpntract}
					disabled={curOper.isNewContractValid || isNewContractPushed}
				>
					Добавить договор
				</button>

			</div>
		{/if}

		<button
			type='button'
			class='yellow-button'
			disabled={false}
			onclick={openNewContract}
		>
			Новый договор
		</button>

	{/if}

	<button
		type='button'
		class='green-button'
		disabled={false}
		onclick={openContract}
	>
		Изменить договор
	</button>
</div>

<div class='group-one'>
	<div>
		<label class='green-field-label' for='OperManualIsDupl'>
			Признак дуприката
		</label>
		<input
			class='green-field'
			type='text'
			id='OperManualIsDupl'
			bind:value={curOper.isDuplicateStr}
			disabled={true}
		/>

	</div>


	<div>
		<label
			class='green-field-label' 
			for='OperManualOperDate'
		>
			Дата операции
		</label>
		<input
			type='text'
			class='green-field'
			id='OperManualOperDate'
			placeholder='00.00.0000'
			bind:value={curOper.data.operDate.value}
			class:input-error={!curOper.data.operDate.isValid}
		/>
	</div>

	<div>
		<label
			class='green-field-label' 
			for='OperManuelDocType'
		>
			Тип первичного документа
		</label>
		<input
			type='text'
			class='green-field'
			id='OperManuelDocType'
			placeholder='строка до 50 знаков'
			bind:value={curOper.data.docType.value}
			class:input-error={!curOper.data.docType.isValid}
		/>
	</div>

	<div>
		<label
			class='green-field-label' 
			for='OperManuelDocNum'
		>
			Номер первичного документа
		</label>
		<input
			type='text'
			class='green-field'
			id='OperManuelDocNum'
			placeholder='строка до 50 знаков'
			bind:value={curOper.data.docNum.value}
			class:input-error={!curOper.data.docNum.isValid}
		/>
	</div>

	<div>
		<label
			class='green-field-label' 
			for='OperManuelDocDate'
		>
			Дата первичного документа
		</label>
		<input
			type='text'
			class='green-field'
			id='OperManuelDocDate'
			placeholder='00.00.0000'
			bind:value={curOper.data.docDate.value}
			class:input-error={!curOper.data.docDate.isValid}
		/>
	</div>
</div>

<section class='group-one'>
	<button
		type='button'
		class='blue-button'
		disabled={curOper.isValid || isAddOperationPushed}	
		onclick={addOperation}
	>
		сформировать операцию
	</button>

	<button
		type='button'
		class='blue-button'
		disabled={false}	
		onclick={cmdProcessOperations}
	>
		Загрузить операции
	</button>

</section>


<dialog 
	class='dialog-top-left'
	id='operManualAllCompanys'
	onclick={(e) => {
		// e.currentTarget — это сам тег dialog
		const rect = e.currentTarget.getBoundingClientRect();
		
		// Проверяем, находится ли клик внутри границ контента окна
		const isClickInside = 
			e.clientX >= rect.left &&
			e.clientX <= rect.right &&
			e.clientY >= rect.top &&
			e.clientY <= rect.bottom;

		// Если кликнули за пределами этих границ (по бэкдропу) — закрываем окно
		if (!isClickInside) {
			e.currentTarget.close();
		}
	}}
>
	<section class='group-one'>
		<span class='yellow-button-span'>
			Выберите контрагента
		</span>
		{#each curOper.allCtrPtys as ctrPty}
			<li>
				<button
					type='button'
					class='yellow-button'
					disabled={false}
					onclick={()=> selectCtrPty(ctrPty)}

				>
					{ctrPty.metadata.comp_name?.short_egrul_name ?? ""}
				</button>
			</li>
		{/each}
		<button
			type='button'
			class='yellow-button'
			disabled={false}
			onclick={()=>(document.getElementById('operManualAllCompanys') as HTMLDialogElement)?.close()}
		>
			Закрыть окно
		</button>


	</section>
</dialog>




<label
	class='green-field-label' 
	for='OperManuelDocDate1'
>
	operId
</label>
<input
	type='text'
	class='green-field'
	id='OperManuelDocDate1'
	placeholder='00.00.0000'
	bind:value={curOper.data.operId.value}
	class:input-error={!curOper.data.operId.isValid}
/>

<label
	class='green-field-label' 
	for='OperManuelDocDate2'
>
	userId
</label>
<input
	type='text'
	class='green-field'
	id='OperManuelDocDate2'
	placeholder='00.00.0000'
	bind:value={curOper.data.userId.value}
	class:input-error={!curOper.data.userId.isValid}
/>

<label
	class='green-field-label' 
	for='OperManuelDocDate3'
>
	compId
</label>
<input
	type='text'
	class='green-field'
	id='OperManuelDocDate3'
	placeholder='00.00.0000'
	bind:value={curOper.data.compId.value}
	class:input-error={!curOper.data.compId.isValid}
/>

<label
	class='green-field-label' 
	for='OperManuelDocDate4'
>
	ctrptyId
</label>
<input
	type='text'
	class='green-field'
	id='OperManuelDocDate4'
	placeholder='00.00.0000'
	bind:value={curOper.data.ctrptyId.value}
	class:input-error={!curOper.data.ctrptyId.isValid}
/>

<label
	class='green-field-label' 
	for='OperManuelDocDate5'
>
	debet
</label>
<input
	type='text'
	class='green-field'
	id='OperManuelDocDate5'
	placeholder='00.00.0000'
	bind:value={curOper.data.debet.value}
	class:input-error={!curOper.data.debet.isValid}
/>

<label
	class='green-field-label' 
	for='OperManuelDocDate6'
>
	credit
</label>
<input
	type='text'
	class='green-field'
	id='OperManuelDocDate6'
	placeholder='00.00.0000'
	bind:value={curOper.data.credit.value}
	class:input-error={!curOper.data.credit.isValid}
/>

<label
	class='green-field-label' 
	for='OperManuelDocDate7'
>
	amount
</label>
<input
	type='text'
	class='green-field'
	id='OperManuelDocDate7'
	placeholder='00.00.0000'
	bind:value={curOper.data.amount.value}
	class:input-error={!curOper.data.amount.isValid}
/>

<label
	class='green-field-label' 
	for='OperManuelDocDate8'
>
	operDate
</label>
<input
	type='text'
	class='green-field'
	id='OperManuelDocDate8'
	placeholder='00.00.0000'
	bind:value={curOper.data.operDate.value}
	class:input-error={!curOper.data.operDate.isValid}
/>

<label
	class='green-field-label' 
	for='OperManuelDocDate9'
>
	docType
</label>
<input
	type='text'
	class='green-field'
	id='OperManuelDocDate9'
	placeholder='00.00.0000'
	bind:value={curOper.data.docType.value}
	class:input-error={!curOper.data.docType.isValid}
/>

<label
	class='green-field-label' 
	for='OperManuelDocDate10'
>
	docNum
</label>
<input
	type='text'
	class='green-field'
	id='OperManuelDocDate10'
	placeholder='00.00.0000'
	bind:value={curOper.data.docNum.value}
	class:input-error={!curOper.data.docNum.isValid}
/>

<label
	class='green-field-label' 
	for='OperManuelDocDate11'
>
	docDate
</label>
<input
	type='text'
	class='green-field'
	id='OperManuelDocDate11'
	placeholder='00.00.0000'
	bind:value={curOper.data.docDate.value}
	class:input-error={!curOper.data.docDate.isValid}
/>

<label
	class='green-field-label' 
	for='OperManuelDocDate12'
>
	entrDate
</label>
<input
	type='text'
	class='green-field'
	id='OperManuelDocDate12'
	placeholder='00.00.0000'
	bind:value={curOper.data.entrDate.value}
	class:input-error={!curOper.data.entrDate.isValid}
/>




