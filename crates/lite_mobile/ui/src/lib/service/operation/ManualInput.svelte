<script lang='ts'>
	import { FieldValidator } from '$lib/models/Auth/FieldValidator.svelte';
	import { OperationSvelte } from '$lib/models/Operation/OperationSvelte.svelte';
	import {operStep} from '$lib/models/Operation/OperationManager.svelte';
	import type { Operation } from '$lib/models/rustModels/Operation';
	import type { OperationStep } from '$lib/models/rustModels/OperationStep';
	import type { Company } from '$lib/models/rustModels/Company';
	


	let currOper = new OperationSvelte();
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
			await currOper.cmdChangeCtrPty(compInn.value, kpp.value);
			if (currOper.ctrPty != null) {
				allCtrPtys.push(currOper.ctrPty);
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

	
</script>


<div class='group-one'>
	<label class='green-field-label' for='cur-ctrpty'>
		Выбранный контрагент
	</label>
	<input
		id='cur-ctrpty'
		class='green-field'
		type='text'
		disabled={true}
		placeholder='Контрагент не выбран'
		value={currOper.ctrPty?.metadata.comp_name?.short_egrul_name ?? ''}
	/>
	<button 
		class='yellow-button'
		type='button'
		disabled={false}
		onclick={openCtrpty}
	>
		Сменить контрагента
	</button>

	{#if isCtrPtyOpen}
		<label class='yellow-field-label' for='compInn'>
			Инн организации
		</label>
		<input
			class='yellow-field'
			id='compInn'
			type='text'
			placeholder='10 | 12 цифр'
			bind:value={compInn.value}
			class:input-error={!compInn.isValid}
		/>

		<label class='yellow-field-label' for='kpp'>
			Кпп орназизации
		</label>
		<input
			class='yellow-field'
			id='kpp'
			type='text'
			placeholder='10 | 12 цифр'
			bind:value={kpp.value}
			class:input-error={!kpp.isValid}

		/>
		<button
			type='button'
			class='blue-button'
			disabled={!compInn.isValid || !kpp.isValid}
			onclick={changeCtrpty}
		>
			Выбрать контрагента
		</button>
	{/if}

</div>


