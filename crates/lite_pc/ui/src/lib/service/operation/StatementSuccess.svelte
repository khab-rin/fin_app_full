

<script lang='ts'>
	import {onMount} from 'svelte';
	import {invoke} from '@tauri-apps/api/core';
	import {dialogBackdrop} from '$lib/rules/dialogBorders'
	import {openDialogRight} from '$lib/rules/dialogBorders';
	import {fitText} from '$lib/rules/text';
	import { FieldValidator } from '$lib/models/Auth/FieldValidator.svelte';
	import {operStep} from '$lib/models/Operation/OperationManager.svelte';
	import { OperationType } from '$lib/models/Operation/OperationValues';
	import { StateProcessor } from '$lib/models/Operation/StatementProcessor.svelte';
	import type { Company } from '$lib/models/rustModels/Company';
	import type { Contract } from '$lib/models/rustModels/Contract';
	import type { OperationStep } from '$lib/models/rustModels/OperationStep';

	let processor = new StateProcessor;

	let compInn = new FieldValidator('CompInn', '');
	let kpp = new FieldValidator('Kpp', '');
	let changeCtrptyPushed = $state(false);

	let isNewContractPushed = $state(false);

	function changeContract(contract: Contract) {
		processor.curOper?.changeContract(contract);
		(document.getElementById('operStatSuccContrDial') as HTMLDialogElement)?.close();
	}

	async function selectCtrPty(ctrPty: Company) {
		if (processor == null || processor.curOper == null) {return;}
		await processor.curOper.selectCtrPty(ctrPty);
		(document.getElementById('operStateSuccAllCompanys') as HTMLDialogElement)?.close();
	}

	let isProcessOperationsPushed = $state(false);

	async function cmdAddNewContract() {
		if (isNewContractPushed) return;
		try {
			isNewContractPushed = true;
			await processor.curOper?.cmdAddNewContract();
			isNewContractPushed = false;
			(document.getElementById('OperStatSeccNewContractDialog') as HTMLDialogElement)?.close()
		} catch(err) {
			const next_step: OperationStep = {
				TryLater:{text:'Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение'}
			}
			console.error("cmdAddNewContract FAILED, err = ", err);
			isNewContractPushed = false;
			operStep.step = next_step;
		}
	}

	async function changeCtrpty() {
		if (processor == null || processor.curOper == null) {return}
		if (changeCtrptyPushed || !kpp.isValid || !compInn.isValid) {return;}
		changeCtrptyPushed = true;
		try {
			await processor.curOper.cmdChangeCtrPty(compInn.value, kpp.value);
			(document.getElementById('OperStatSuccNewCtrptyDialog') as HTMLDialogElement)?.close();

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
			if (processor.curOper == null) {return;}
			await processor.curOper.cmdGetAllCompanys();
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
				onclick={(e)=>openDialogRight(e, 'OperStatSuccNewCtrptyDialog')}
				id='OperStatSuccChangCtrty'
			>	
				<span class='t-but'>
					Новый контрагент
				</span>
				
			</button>
		</div>

		<div class='w-v30'>
			<label class='label-close' for="OperStatSuccSelectCtrPty">&nbsp;</label>
			<button 
				type='button'
				class='but-gr x-self-c'
				disabled={false}
				onclick={(e)=>openDialogRight(e, 'operStateSuccAllCompanys')}
				id='OperStatSuccChangCtrty'
			>	
				<span class='t-but'>
					Сменить контрагента
				</span>
				
			</button>
		</div>	
	</section>

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
				onclick={(e)=>openDialogRight(e, 'OperStatSeccNewContractDialog')}
			>
				<span class='t-fill' use:fitText>
					Новый договор
				</span>
				
			</button>
		</div>

		<div class='w-v20'>
			<button 
				type='button'
				class='but-ye'
				disabled={false}
				onclick={(e)=>openDialogRight(e,'operStatSuccContrDial')}
			>
				<span class='t-fill' use:fitText>
					Список договоров
				</span>
			</button>
		</div>
	</section>

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
	class='dial-ver' 
	id='operStatSuccContrDial'
	onclick={dialogBackdrop}
	
>
	<section class='w-100'>
		<span class='w-g100 t-fill x-ce'>
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

<dialog
	class='dial-ver'
	id='OperStatSeccNewContractDialog'
	onclick={dialogBackdrop}
>
	
	<div class='w-100'>
		<g4 class='w-g100 t-fill x-ce'>Введите данные нового договора</g4>
	</div>
	
	{#if processor && processor.curOper} 
		<div class='w-100'>
			<div class='w-v100'>
				<label class='label-close' for='OperManualNewContrNum'>
					Номер договора
				</label>
				<input
					type='text'
					class='input-ye'
					id='OperManualNewContrNum'
					bind:value={processor.curOper.newContrData.contractNum.value}
					placeholder='Строка до 50 знаков'
					class:input-error={!processor.curOper.newContrData.contractNum.isValid}
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
					bind:value={processor.curOper.newContrData.contractDate.value}
					placeholder='00.00.0000'
					class:input-error={!processor.curOper.newContrData.contractDate.isValid}
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
					bind:value={processor.curOper.newContrData.contractTitle.value}
					placeholder='Строка до 50 знаков'
					class:input-error={!processor.curOper.newContrData.contractTitle.isValid}
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
					bind:value={processor.curOper.newContrData.contractStDate.value}
					placeholder='00.00.0000'
					class:input-error={!processor.curOper.newContrData.contractStDate.isValid}
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
					bind:value={processor.curOper.newContrData.contractEndDate.value}
					placeholder='00.00.0000'
					class:input-error={!processor.curOper.newContrData.contractEndDate.isValid}
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
					bind:value={processor.curOper.newContrData.contractCurrency.value}
					placeholder='РУБ'
					class:input-error={!processor.curOper.newContrData.contractCurrency.isValid}
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
					bind:value={processor.curOper.newContrData.contractTotAmnt.value}
					placeholder='Сумма в валюте договора'
					class:input-error={!processor.curOper.newContrData.contractTotAmnt.isValid}
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
					bind:value={processor.curOper.newContrData.contractDefDays.value}
					placeholder='Сумма в валюте договора'
					class:input-error={!processor.curOper.newContrData.contractDefDays.isValid}
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
					bind:value={processor.curOper.newContrData.contractDefDays.value}
					placeholder='Количество дней'
					class:input-error={!processor.curOper.newContrData.contractDefDays.isValid}
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
					bind:value={processor.curOper.newContrData.contractDescr.value}
					placeholder='Количество дней'
					class:input-error={!processor.curOper.newContrData.contractDescr.isValid}
				/>
			</div>
		</div>
		<div class='w-100'>
			<div class='w-v100'>
				<button 
					class='but-ye'
					type='button'
					onclick={cmdAddNewContract}
					disabled={processor.curOper.isNewContractValid || isNewContractPushed}
				>
					Добавить договор
				</button>
			</div>
		</div>
	{/if}
</dialog>

<dialog 
	class='dial-ver'
	id='operStateSuccAllCompanys'
>
	<section class='w-100'>
		<div class='w-v100'>
			<h4 class='t-fill w-g100 x-ce'>
				Выберите контрагента
			</h4>
		</div>
	</section>
	{#if processor && processor.curOper}
		{#each processor.curOper.allCtrPtys as ctrPty}
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
	{/if}
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
	id='OperStatSuccNewCtrptyDialog'
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