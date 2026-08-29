<script lang='ts'>
	import {invoke} from '@tauri-apps/api/core';

	import {FieldValidator} from '$lib/models/Auth/FieldValidator.svelte';
	import { operStep } from '$lib/models/Operation/OperationManager.svelte';

	import type { OperationStep } from '$lib/models/rustModels/OperationStep';

	let isNewAccPushed = $state(false);

	let bic = new FieldValidator('Bic', '');
	let rasAcc = new FieldValidator('RasAcc', '');

	async function name() {
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
		} catch(err) {
			let next_step: OperationStep = {TryLater: {text: 'Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение'}};
			console.error("isNewAccPushed FAILED, err = ",err);
			operStep.step = next_step;
		}
	}
</script>


<div class='group-one'>
	<label class='green-field-label' for='operAccBic'>
		Введите БИК
	</label>
	<input
		type='text'
		class='green-field'
		id='operAccBic'
		disabled={false}
		bind:value={bic.value}
		placeholder="9 цифр"
		class:input-error={!bic.isValid}
	/>

	<label class='green-field-label' for='operAccInn'>
		Введите номер расчетного счета
	</label>
	<input
		type='text'
		class='green-field'
		id='operAccInn'
		disabled={false}
		bind:value={rasAcc.value}
		placeholder="20 цифр"
		class:input-error={!rasAcc.isValid}
	/>
</div>



