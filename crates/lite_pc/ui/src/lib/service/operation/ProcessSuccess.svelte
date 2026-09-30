<script lang='ts'>
	import { onMount } from "svelte";

	import type { OperationStep } from "$lib/models/rustModels/OperationStep";
	import { OperationType } from "$lib/models/Operation/OperationValues";
	import { operStep } from "$lib/models/Operation/OperationManager.svelte";

	let count = 0;

	onMount(() => {
		if (OperationType.ProcessSuccess in operStep.step) {
			count = operStep.step.ProcessSuccess.count
		} else {
			let nextStep: OperationStep = {TryLater: {text:'Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение'}};
			operStep.step = nextStep;
		}
	})
</script>

<span>
	Успешно обработано - {count} операций
</span>