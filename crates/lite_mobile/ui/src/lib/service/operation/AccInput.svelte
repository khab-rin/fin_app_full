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

<section class='input-section'>
	<div class='input-group'>
		<span class='input-group-span'>
			Введите БИК
		</span>
		<input
			type='text'
			class='input-field'
			disabled={false}
			bind:value={bic.value}
			placeholder="9 цифр"
			class:input-error={!bic.isValid}
		/>
	</div>

	<div class='input-group'>
		<span class='input-group-span'>
			Введите номер расчетного счета
		</span>
		<input
			type='text'
			class='input-field'
			disabled={false}
			bind:value={bic.value}
			placeholder="20 цифр"
			class:input-error={!rasAcc.isValid}
		/>
	</div>



</section>