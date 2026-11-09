<script lang='ts'>
	import {invoke} from '@tauri-apps/api/core';

	import {FieldValidator} from '$lib/models/Auth/FieldValidator.svelte';
	import { operStep } from '$lib/models/Operation/OperationManager.svelte';

	import type { OperationStep } from '$lib/models/rustModels/OperationStep';

	let isNewAccPushed = $state(false);

	let bic = new FieldValidator('Bic', '');
	let rasAcc = new FieldValidator('RasAcc', '');

	async function addAcc() {
		if (!bic.isValid || !rasAcc.isValid || isNewAccPushed) {
			return;
		}
		isNewAccPushed = true;

		try {
			let data = {
                bic: bic.value,
                rasAcc: rasAcc.value
            };
			await invoke(
				"cmd_add_comp_bank_acc", data
			);
			bic.value = "";
			rasAcc.value = "";
			isNewAccPushed = false;
		} catch(err) {
			let next_step: OperationStep = {TryLater: {text: 'Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение'}};
			console.error("isNewAccPushed FAILED, err = ",err);
			isNewAccPushed = false;
			operStep.step = next_step;
		}
	}
</script>


<div class='w-100'>
	<div class='w-v30'>
		<label class='label-close' for='operAccBic'>
			Введите БИК
		</label>
		<input
			type='text'
			class='input-gr'
			id='operAccBic'
			disabled={false}
			bind:value={bic.value}
			placeholder="9 цифр"
			class:input-error={!bic.isValid}
		/>
	</div>
		
	<div class='w-v70'>
		<label class='label-close' for='operAccInn'>
			Введите номер расчетного счета
		</label>
		<input
			type='text'
			class='input-gr'
			id='operAccInn'
			disabled={false}
			bind:value={rasAcc.value}
			placeholder="20 цифр"
			class:input-error={!rasAcc.isValid}
		/>
	</div>

</div>

<div class='w-100'>
	<button
        type='button'
        class='but-gr'
        disabled={!bic.isValid || !rasAcc.isValid || isNewAccPushed}
        onclick ={addAcc}
    >	
		{#if isNewAccPushed}
			<span class="loading-content">
				<span class="loading-spinner"></span>
				Добавление...
			</span>
		{:else}
			Добавить расчетный счет
		{/if}
    </button>
</div>





