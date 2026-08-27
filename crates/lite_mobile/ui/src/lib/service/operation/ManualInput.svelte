<script lang='ts'>
	import {onMount} from 'svelte';
	import { FieldValidator } from '$lib/models/Auth/FieldValidator.svelte';
	import { OperationSvelte } from '$lib/models/Operation/OperationSvelte.svelte';
	import {operStep} from '$lib/models/Operation/OperationManager.svelte';
	import type { Operation } from '$lib/models/rustModels/Operation';
	import type { OperationStep } from '$lib/models/rustModels/OperationStep';
	


	let currOper = new OperationSvelte();
	let rustOperations: Operation[] = $state<Operation[]>([]);

	let isCtrPtyOpen = $state(false);
	let isChangeCtrPtyPushed = $state(false);
	let kpp = new FieldValidator('Kpp', '');
	let compInn = new FieldValidator('CompInn', '');
	function openCtrpty() {isCtrPtyOpen = !isCtrPtyOpen;}
	async function changeCtrpty() {
		if (isChangeCtrPtyPushed || !kpp.isValid || !compInn.isValid) {return;}
		isChangeCtrPtyPushed = true;
		try {
			await currOper.cmdChangeCtrPty(compInn.value, kpp.value);
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


<section class='input-section'>
	<div class='input-group'>
		<span class=input-group-span>
			Выбранный контрагент
		</span>
		<input
			class='input-field'
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
			Контрагент
		</button>

	</div>

	{#if isCtrPtyOpen}
		<span class='input-field-span'>
			Инн организации
		</span>
		<input
			class='input-field'
			type='text'
			placeholder='10 | 12 цифр'
			bind:value={compInn.value}
			class:input-error={!compInn.isValid}

		/>

		<span class='input-field-span'>
			Кпп орназизации
		</span>
		<input
			class='input-field'
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
			Сменить контрагента
		</button>
	{/if}

</section>