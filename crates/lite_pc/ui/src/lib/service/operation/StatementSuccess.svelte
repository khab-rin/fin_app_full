

<script lang='ts'>
	import {onMount} from 'svelte';
	import {invoke} from '@tauri-apps/api/core';
	import {dialogBackdrop} from '$lib/rules/dialogBorders'
	import { FieldValidator } from '$lib/models/Auth/FieldValidator.svelte';
	import {operStep} from '$lib/models/Operation/OperationManager.svelte';
	import { OperationType } from '$lib/models/Operation/OperationValues';
	import { StateProcessor } from '$lib/models/Operation/StatementProcessor.svelte';
	
	
	import type { Contract } from '$lib/models/rustModels/Contract';
	import type { OperationStep } from '$lib/models/rustModels/OperationStep';

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
		(document.getElementById('operStatSeccContrDial') as HTMLDialogElement)?.close();
		isChangeContractOpen = false;
		isNewContractOpen = false;
		isContractsOpen = false;
	}

	function openContrList() {
		(document.getElementById('operStatSeccContrDial') as HTMLDialogElement)?.showModal();
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
			isProcessOperationsPushed = false;
			operStep.step = next_step;
		} catch(err) {
			const next_step: OperationStep = {TryLater: {text: 'Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение'}};
			console.error("cmdProcessOperations FAILED, err = ", err);
			isProcessOperationsPushed = false;
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
	<section class='w-40'>
		<h4 class="t-fill w-g100 x-ce">
			Необработанных операций - {processor.unProcceed}
		</h4>
	</section>
{/if}

{#if processor && processor.curOper}
	<section class='w-40'>
		<div class='w-v70'>
			<label class='label-close' for='operStatSuccCtrPtyName'>
				Выбранный контрагент
			</label>
			<input
				class='input-gr'
				type='text'
				id='operStatSuccCtrPtyName'
				disabled={true}
				placeholder='Контрагент не выбран'
				value={processor.curOper.ctrPty?.metadata.comp_name?.short_egrul_name ?? ''}
			/>
		</div>

		<div class='w-v30'>
			<label class='label-close' for="OperStatSuccChangCtrty">&nbsp;</label>
			<button 
				type='button'
				class='but-gr x-self-c'
				disabled={false}
				onclick={showCtrPty}
				id='OperStatSuccChangCtrty'
			>
				Сменить контрагента
			</button>
		</div>		
	</section>

	{#if openCtrpty}
		<section class='w-60'>
			<div class='w-v40'>
				<label for='operStatSuccCtrPtyInn'>
					Инн орназизации
				</label>
				<input
					class='input-ye'
					type='text'
					id='operStatSuccCtrPtyInn'
					placeholder='10 | 12 цифр'
					bind:value={compInn.value}
					class:input-error={!compInn.isValid}

				/>
			</div>

			<div class='w-v40'>
				<label for='operStatSuccCtrPtyKpp'>
					Кпп орназизации
				</label>
				<input
					class='input-ye'
					type='text'
					id='operStatSuccCtrPtyKpp'
					placeholder='10 | 12 цифр'
					bind:value={kpp.value}
					class:input-error={!kpp.isValid}
				/>
			</div>

			<div class='w-v20'>
				<label class='label-close' for="OperStatNewCtrty">&nbsp;</label>
				<button
					type='button'
					class='but-ye'
					disabled={!compInn.isValid || !kpp.isValid}
					onclick={changeCtrpty}
					id='OperStatNewCtrty'
				>
					Сменить контрагента
				</button>
			</div>
		</section>
	{/if}


	<section class=w-60>
		<div class='w-v33'>
			<label class='label-close' for='operStatSuccDebet'>
				Дебет {processor.curOper.debetStr}
			</label>
			<input
				class = 'input-gr'
				type='text'
				id='operStatSuccDebet'
				bind:value={processor.curOper.data.debet.value}
				disabled={false}
				placeholder='Номер счета'
				class:input-error={!processor.curOper.data.debet.isValid}
			/>
		</div>


		<div class='w-v33'>
			<label class='label-close' for='operStatSuccCredit'>
				Кредит {processor.curOper.creditStr}
			</label>
			<input
				class = 'input-gr'
				type='text'
				id='operStatSuccCredit'
				bind:value={processor.curOper.data.credit.value}
				disabled={false}
				placeholder='Номер счета'
				class:input-error={!processor.curOper.data.credit.isValid ||
					!processor.curOper.isCompare
				}
			/>
		</div>


		<div class='w-v20'>
			<label class='label-close' for='operStatSuccAmnt'>
				Сумма операции
			</label>
			<input
				class = 'input-gr'
				type='text'
				id='operStatSuccAmnt'
				bind:value={processor.curOper.data.amount.value}
				disabled={false}
				placeholder='xxx.xx'
				class:input-error={!processor.curOper.data.amount.isValid}
			/>
		</div>

		<div class='w-v15'>
			<label class='label-close' for='operStatSuccOperDate'>
				Дата операции
			</label>
			<input
				class='input-gr'
				type='text'
				id='operStatSuccOperDate'
				bind:value={processor.curOper.data.operDate.value}
				disabled={false}
				placeholder='xx.xx.xxxx'
				class:input-error={!processor.curOper.data.operDate.isValid}
			/>
		</div>
	</section>

	<section class='w-40'>
		<div class='w-v60'>
			<label class='label-close' for='operStatSuccContrInfo'>
				Информация о договоре
			</label>
			<input
				class='input-gr'
				type='text'
				id='operStatSuccContrInfo'
				disabled={true}
				placeholder='без договора'
				bind:value={processor.curOper.contrStr}
			/>
		</div>

		<div class='w-v20'>
			<button 
				type='button'
				class='but-ye'
				disabled={false}
				onclick={openNewContract}
			>
				Новый договор
			</button>
		</div>

		<div class='w-v20'>
			<button 
				type='button'
				class='but-ye'
				disabled={false}
				onclick={openContrList}
			>
				Список договоров
			</button>
		</div>
	</section>


	{#if isNewContractOpen}
		<section class='w-85'>
			<div class='w-v30'>
				<label class='label-close' for='StateSuccNewContName'>Название договора</label>
				<input 
					class='input-ye'
					type='text' 
					id='StateSuccNewContName' 
					bind:value={processor.curOper.newContrData.contractTitle.value} 
					placeholder='строка до 50 знаков'
					class:input-error={!processor.curOper.newContrData.contractTitle.isValid}
				/>
			</div>
			<div class='w-v70'>
				<label class="label-close" for='StateSuccNewContDescr'>Описание</label>
				<input 
					class='input-ye'
					type='text' 
					id='StateSuccNewContDescr' 
					bind:value={processor.curOper.newContrData.contractDescr.value} 
					placeholder='строка до 50 знаков'
					class:input-error={!processor.curOper.newContrData.contractDescr.isValid}
				/>
			</div>
		</section>

		<section class='w-60'>
			<div class='w-v100'>
				<label class='label-close' for='StateSuccNewContNum'>Номер договора</label>
				<input 
					class='input-ye'
					type='text' 
					id='StateSuccNewContNum'
					bind:value={processor.curOper.newContrData.contractNum.value} 
					placeholder='строка до 50 знаков'
					class:input-error={!processor.curOper.newContrData.contractNum.isValid}
				/>
			</div>

			<div class='w-v33'>
				<label class='label-close' for='StateSuccNewContDate'>Дата договора</label>
				<input 
					class='input-ye'
					type='text' 
					id='StateSuccNewContDate' 
					bind:value={processor.curOper.newContrData.contractDate.value} 
					placeholder='дд.мм.гггг'
					class:input-error={!processor.curOper.newContrData.contractDate.isValid}
				/>
			</div>

			<div class='w-v33'>
				<label class='label-close' for='StateSuccNewContStDate'>Дата начала</label>
				<input 
					class='input-ye'
					type='text' 
					id='StateSuccNewContStDate'  
					bind:value={processor.curOper.newContrData.contractStDate.value} 
					placeholder='дд.мм.гггг'
					class:input-error={!processor.curOper.newContrData.contractStDate.isValid}
				/>
			</div>

			<div class='w-v33'>
				<label class='label-close' for='StateSuccNewContEndDate'>Дата завершения</label>
				<input 
					class='input-ye'
					type='text' 
					id='StateSuccNewContEndDate' 
					bind:value={processor.curOper.newContrData.contractEndDate.value} 
					placeholder='дд.мм.гггг'
					class:input-error={!processor.curOper.newContrData.contractEndDate.isValid}
				/>
			</div>
		</section>

		<section class='w-40'>
			<div class='w-v33'>
				<label class='label-close' for='StateSuccNewContAmnt'>Сумма договора</label>
				<input 
					class='input-ye'
					type='text' 
					id='StateSuccNewContAmnt'  
					bind:value={processor.curOper.newContrData.contractTotAmnt.value} 
					placeholder='Сумма в валюте договора'
					class:input-error={!processor.curOper.newContrData.contractTotAmnt.isValid}
				/>
			</div>

			<div class='w-v33'>
				<label class='label-close' for='StateSuccNewContCurrency'>Валюта договора</label>
				<input 
					class='input-ye'
					type='text' 
					id='StateSuccNewContCurrency' 
					bind:value={processor.curOper.newContrData.contractCurrency.value} 
					placeholder='РУБ'
					class:input-error={!processor.curOper.newContrData.contractCurrency.isValid}
				/>
			</div>
			<div class='w-v33'>
				<label class='label-close' for='StateSuccNewContDeffDays'>Рассрочка в днях</label>
				<input 
					class='input-ye'
					type='text' 
					id='StateSuccNewContDeffDays' 
					bind:value={processor.curOper.newContrData.contractDefDays.value} 
					placeholder='количество дней'
					class:input-error={!processor.curOper.newContrData.contractDefDays.isValid}
				/>
			</div>
		</section>

		<section class='w-40'>
			<div class='w-v100'>
				<button class='but-ye'
					type='button'
					onclick={cmdAddNewContract}
					disabled={processor.curOper.isNewContractValid || isNewContractPushed}
				>
					Добавить договор
				</button>
			</div>
		</section>
	{/if}

	<section class='w-20'>
		<div class='w-v100'>
			<label class='label-close' for='StateSuccIsDupl'>
				Признак дубликата
			</label>
			<input
				class = 'input-gr'
				type='text'
				id='StateSuccIsDupl'
				bind:value={processor.curOper.isDuplicateStr}
				disabled={true}
			/>
		</div>
	</section>

	<section class='w-60'>
		<div class='w-v100'>
			<label class='t-bl label-close' for='operStatSuccComment'>
				Комментарий операции
			</label>
			<p class='t-fill w-g100' id='operStatSuccComment'>
				{processor.curOper.comment}
			</p>
		</div>
	</section>

	<section class='w-40'>
		<div class='w-v100'>
			<button
				type='button'
				class='but-bl'
				id='StateSuccProcessBut'
				onclick={() => processor.makeRust()}
				disabled={processor.curOper.isValid}
			>
				Обработать
			</button>
		</div>
	</section>
		

	<section class='w-40'>
		<div class='w-v50'>
			<button
				type='button'
				class='but-pu'
				onclick={() => processor.prev()}
			>
				Пред. операция
			</button>
		</div>

		<div class='w-v50'>
			<button
				type='button'
				class='but-pu'
				onclick={() => processor.next()}
			>
				След. операция
			</button>
		</div>
	</section>

{/if}


{#if (processor && processor.unProcceed == 0)}
	<section class='w-40'>
		<div class='w-v100'>
			<button
				type='button'
				class='but-bl'
				disabled={processor.unProcceed > 0}
				onclick={cmdProcessOperations}
			>
				сохранить операции
			</button>
		</div>
	</section>
{/if}


<dialog 
	class='dialog-top-l' 
	id='operStatSeccContrDial'
	onclick={dialogBackdrop}
	
>
	<section class='w-100'>
		<span class='w-g100 t-fill'>
			Выберите договор
		</span>
	</section>
	{#if processor && processor.curOper}
		{#each processor.curOper?.allPossContracts as contract}
			<section class='w-100'>
				<div class='w-g100 t-fill'>
					<button
						class='but-ye'
						type='button'
						onclick={() => changeContract(contract)}
					>
						<span class='w-g100 x-ce t-fill'>
							{processor.curOper.anyContractStr(contract)}
						</span>
					</button>
				</div>
			</section>
		{/each}
	{/if}
</dialog>