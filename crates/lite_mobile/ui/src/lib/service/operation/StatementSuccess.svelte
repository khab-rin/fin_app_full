

<script lang='ts'>
	import {onMount} from 'svelte';
	import {invoke} from '@tauri-apps/api/core';
	import {operStep} from '$lib/models/Operation/OperationManager.svelte';
	import { OperationType } from '$lib/models/Operation/OperationValues';
	import { StateProcessor } from '$lib/models/Operation/StatementProcessor.svelte';
	import type { OperationStep } from '$lib/models/rustModels/OperationStep';
	import { FieldValidator } from '$lib/models/Auth/FieldValidator.svelte';
	import type { Contract } from '$lib/models/rustModels/Contract';

	let processor = new StateProcessor;

	let openCtrpty = $state(false);
	let compInn = new FieldValidator('CompInn', '');
	let kpp = new FieldValidator('Kpp', '');
	let changeCtrptyPushed = $state(false);
	function showCtrPty() {
		openCtrpty = !openCtrpty;
	}

	let isContractsOpen = $state(false);
	let isNewContractOpen = $state(false);
	let isChangeContractOpen = $state(false);
	let isNewContractPushed = $state(false);

	function openContracts() {
		isContractsOpen = !isContractsOpen;
	}

	function openNewContract() {
		isChangeContractOpen = false;
		isNewContractOpen = !isNewContractOpen;
	}

	function openChangeContract() {
		isNewContractOpen = false;
		isChangeContractOpen = !isChangeContractOpen;
	}

	function changeContract(contract: Contract) {
		processor.curOper?.changeContract(contract);
		isChangeContractOpen = false;
		isNewContractOpen = false;
		isContractsOpen = false;
	}

	let isProcessOperationsPushed = $state(false);

	async function cmdAddNewContract() {
		if (isNewContractPushed) return;
		try {
			isNewContractPushed = true;
			await processor.curOper?.cmdAddNewContract();
			isNewContractPushed = false;
			isNewContractOpen = false;
			isContractsOpen = true;
			isChangeContractOpen = true;
		} catch(err) {
			const next_step: OperationStep = {
				TryLater:{text:'Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение'}
			}
			console.error("cmdAddNewContract FAILED, err = ", err);
			isNewContractPushed = false;
			isNewContractOpen = false;
			isContractsOpen = false;
			operStep.step = next_step;
		}
	}

	async function changeCtrpty() {
		if (changeCtrptyPushed) {return;}
		changeCtrptyPushed = true;

		try {
			await processor.curOper?.cmdChangeCtrPty(compInn.value, kpp.value);
			changeCtrptyPushed = false;
		} catch(err) {
			const next_step: OperationStep = {
				TryLater:{text:'Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение'}
			}
			console.error("cmdChangeCtrPty FAILED, err = ", err);
			changeCtrptyPushed = false;
			operStep.step = next_step;
		}
	}

	async function cmdProcessOperations() {
		if (isProcessOperationsPushed) {return;}
		try {
			const next_step = await invoke<OperationStep>(
				"cmd_process_operations",
				{optionOperations: processor.rustOperations}
			);
			operStep.step = next_step;
		} catch(err) {
			const next_step: OperationStep = {TryLater: {text: 'Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение'}};
			console.error("cmdProcessOperations FAILED, err = ", err);
			operStep.step = next_step;
		}
	}


	onMount(async() => {
		if (OperationType.StatementSuccess in operStep.step) {
			await processor.init(operStep.step.StatementSuccess.operations)
		} else {
			const next_step: OperationStep = {
				TryLater: {
					text: 'Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение'
				}
			};
			console.error('System Logic Error, wrong current step');
			operStep.step = next_step;

		}
	})
</script>

{#if processor}
	колечество необработанных операций {processor.unProcceed}
{/if}

{#if processor && processor.curOper}
	<section class='group-one'>
		<div>
			<label class='green-field-label' for='operStatSuccCtrPtyName'>
				Выбранный контрагент
			</label>
			<input
				class='green-field'
				type='text'
				id='operStatSuccCtrPtyName'
				disabled={true}
				placeholder='Контрагент не выбран'
				value={processor.curOper.ctrPty?.metadata.comp_name?.short_egrul_name ?? ''}
			/>
		</div>

		{#if openCtrpty}
			<label class='yellow-field-label' for='operStatSuccCtrPtyInn'>
				Инн орназизации
			</label>
			<input
				class='yellow-field'
				type='text'
				id='operStatSuccCtrPtyInn'
				placeholder='10 | 12 цифр'
				bind:value={compInn.value}
				class:input-error={!compInn.isValid}

			/>

			<label class='yellow-field-label' for='operStatSuccCtrPtyKpp'>
				Кпп орназизации
			</label>
			<input
				class='yellow-field'
				type='text'
				id='operStatSuccCtrPtyKpp'
				placeholder='10 | 12 цифр'
				bind:value={kpp.value}
				class:input-error={!kpp.isValid}
			/>

			<button
				type='button'
				class='yellow-button'
				disabled={!compInn.isValid || !kpp.isValid}
				onclick={changeCtrpty}
			>
				Сменить контрагента
			</button>

		{/if}

		<button 
			type='button'
			class='green-button'
			disabled={false}
			onclick={showCtrPty}
		>
			Контрагент
		</button>
	</section>

	<section class=group-one>
		<label class='green-field-label' for='operStatSuccDebet'>
			Дебет {processor.curOper.debetStr}
		</label>
		<input
			class = 'green-field'
			type='text'
			id='operStatSuccDebet'
			bind:value={processor.curOper.data.debet.value}
			disabled={false}
			placeholder='Номер счета'
			class:input-error={!processor.curOper.data.debet.isValid}
		/>

		<label class='green-field-label' for='operStatSuccCredit'>
			Кредит {processor.curOper.creditStr}
		</label>
		<input
			class = 'green-field'
			type='text'
			id='operStatSuccCredit'
			bind:value={processor.curOper.data.credit.value}
			disabled={false}
			placeholder='Номер счета'
			class:input-error={!processor.curOper.data.credit.isValid ||
				!processor.curOper.isCompare
			}
		/>

		<label class='green-field-label' for='operStatSuccAmnt'>
			Сумма операции
		</label>
		<input
			class = 'green-field'
			type='text'
			id='operStatSuccAmnt'
			bind:value={processor.curOper.data.amount.value}
			disabled={false}
			placeholder='xxx.xx'
			class:input-error={!processor.curOper.data.amount.isValid}
		/>

		<label class='green-field-label' for='operStatSuccOperDate'>
			Дата операции
		</label>
		<input
			class='green-field'
			type='text'
			id='operStatSuccOperDate'
			bind:value={processor.curOper.data.operDate.value}
			disabled={false}
			placeholder='xx.xx.xxxx'
			class:input-error={!processor.curOper.data.operDate.isValid}
		/>
	</section>

	<section class='group-one'>

		<label class='green-field-label' for='operStatSuccContrInfo'>
			Информация о договоре
		</label>
		<input
			class='green-field'
			type='text'
			id='operStatSuccContrInfo'
			disabled={true}
			placeholder='без договора'
			bind:value={processor.curOper.contrStr}
		/>
			
		{#if isContractsOpen}
			<div class='group-one'>
				{#if isChangeContractOpen}
					<label class='yellow-field-span' for='operStatSuccContrSelSect'>Выберите договор</label>
					<section class='group-one' id='operStatSuccContrSelSect'>
						{#each processor.curOper.allPossContracts as contract}
							<button
								type='button'
								class='yellow-button'
								onclick={() => changeContract(contract)}
							>
								{processor.curOper.anyContractStr(contract)}
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
			</div>


			{#if isNewContractOpen}
				<section class='input-section'>
					<span class='input-field-span'>Номер договора</span>
					<input 
						class='input-field'
						type='text' 
						bind:value={processor.curOper.newContrData.contractNum.value} 
						placeholder='строка до 50 знаков'
						class:input-error={!processor.curOper.newContrData.contractNum.isValid}
					/>

					<span class='input-field-span'>Дата договора</span>
					<input 
						class='input-field'
						type='text' 
						bind:value={processor.curOper.newContrData.contractDate.value} 
						placeholder='дд.мм.гггг'
						class:input-error={!processor.curOper.newContrData.contractDate.isValid}
					/>

					<span class='input-field-span'>Название договора</span>
					<input 
						class='input-field'
						type='text' 
						bind:value={processor.curOper.newContrData.contractTitle.value} 
						placeholder='строка до 50 знаков'
						class:input-error={!processor.curOper.newContrData.contractTitle.isValid}
					/>

					<span class='input-field-span'>Дата начала</span>
					<input 
						class='input-field'
						type='text' 
						bind:value={processor.curOper.newContrData.contractStDate.value} 
						placeholder='дд.мм.гггг'
						class:input-error={!processor.curOper.newContrData.contractStDate.isValid}
					/>

					<span class='input-field-span'>Дата завершения</span>
					<input 
						class='input-field'
						type='text' 
						bind:value={processor.curOper.newContrData.contractEndDate.value} 
						placeholder='дд.мм.гггг'
						class:input-error={!processor.curOper.newContrData.contractEndDate.isValid}
					/>

					<span class='input-field-span'>Валюта договора</span>
					<input 
						class='input-field'
						type='text' 
						bind:value={processor.curOper.newContrData.contractCurrency.value} 
						placeholder='РУБ'
						class:input-error={!processor.curOper.newContrData.contractCurrency.isValid}
					/>

					<span class='input-field-span'>Сумма договора</span>
					<input 
						class='input-field'
						type='text' 
						bind:value={processor.curOper.newContrData.contractTotAmnt.value} 
						placeholder='Сумма в валюте договора'
						class:input-error={!processor.curOper.newContrData.contractTotAmnt.isValid}
					/>

					<span class='input-field-span'>Рассрочка в днях</span>
					<input 
						class='input-field'
						type='text' 
						bind:value={processor.curOper.newContrData.contractDefDays.value} 
						placeholder='количество дней'
						class:input-error={!processor.curOper.newContrData.contractDefDays.isValid}
					/>

					<span class='input-field-span'>Описание</span>
					<input 
						class='input-field'
						type='text' 
						bind:value={processor.curOper.newContrData.contractDescr.value} 
						placeholder='строка до 50 знаков'
						class:input-error={!processor.curOper.newContrData.contractDescr.isValid}
					/>

					<button class='medium-button'
						type='button'
						onclick={cmdAddNewContract}
						disabled={processor.curOper.isNewContractValid || isNewContractPushed}
					>
						Добавить договор
					</button>
				</section>
			{/if}

			<button 
				type='button'
				class='medium-button'
				disabled={false}
				onclick={openNewContract}
			>
				Добавить договор
			</button>
		{/if}
			
		<button 
			type='button'
			class='green-button'
			disabled={false}
			onclick={openContracts}
		>
			Изменить договор
		</button>

	</section>




		<div class='input-group'>
			<span class='input-field-span'>
				Признак дубликата
			</span>
			<input
				class = 'input-field'
				type='text'
				bind:value={processor.curOper.isDuplicateStr}
				disabled={true}
			/>
		</div>

		<div class='wide-button-group'>
			<span class='wide-button-group-span'>
				Обработать без возможности дальнейшего редактирования
			</span>
			<button
				type='button'
				class='wide-button'
				onclick={() => processor.makeRust()}
				disabled={processor.curOper.isValid}
			>
				Обработать
			</button>
		</div>


		<div class='medium-button-group'>
			<button
				type='button'
				class='wide-button'
				onclick={() => processor.prev()}
			>
				Пред. операция
			</button>
		</div>
		
		<div class='medium-button-group'>
			<button
				type='button'
				class='wide-button'
				onclick={() => processor.next()}
			>
				След. операция
			</button>
		</div>
		

{/if}


{#if (processor && processor.unProcceed == 0)}
	<div class='wide-button-group'>
		<button
			type='button'
			class='wide-button'
			disabled={processor.unProcceed > 0}
			onclick={cmdProcessOperations}
		>
			сохранить операции
		</button>

	</div>
{/if}