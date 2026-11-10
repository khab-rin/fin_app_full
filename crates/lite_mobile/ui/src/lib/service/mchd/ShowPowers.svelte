<script lang='ts'>
    import {onMount} from 'svelte';
    import {currentMchdStep} from '$lib/models/Mchd/mchdManager.svelte';

    import type {HomeMchdPower} from '$lib/models/rustModels/HomeMchdPower';
    import {MchdStepType} from '$lib/models/Mchd/MchdValues';
	import type { MchdStep } from '$lib/models/rustModels/MchdStep';

    let homePowers:HomeMchdPower[] = [];
    let fnsPowers:HomeMchdPower[] = [];
    let btbPowers:HomeMchdPower[] = [];

    onMount(async() => {
        if (MchdStepType.ShowPowers in currentMchdStep.step) {
            homePowers = currentMchdStep.step.ShowPowers.home;
            fnsPowers = currentMchdStep.step.ShowPowers.fns;
            btbPowers = currentMchdStep.step.ShowPowers.btb;
        } else {
            console.error("Ощибка логики менеджера мчд на странице ShowPowers");
            const nextStep: MchdStep = {TryLater: {text: "Критическая ошибка на устройстве..."}};
            currentMchdStep.step = nextStep;
        }
    });
</script>


<div class='w-100'>
	<p class='x-self-c x-cont-c with-info'>
		Полномочия для доступа к разделам системы
	</p>
</div>


{#each homePowers as power (power)}
	<div class="w-100">
		<span class='x-cont-l'>
			{#await currentMchdStep.get_power_info(power)}
				Загрузка...
			{:then info} 
				{info?.code} - {info?.name}
			{:catch error}
				Ошибка
			{/await}
		</span>
		
	</div>
{/each}

<div class='w-100'>
	<p class='x-self-c x-cont-c with-info'>
		Полномочия для ЭДО с контрагентами
	</p>
</div>


{#each btbPowers as power (power)}
	<div class="w-100">
		<span class='x-cont-l'>
			{#await currentMchdStep.get_power_info(power)}
				Загрузка...
			{:then info} 
				{info?.code} - {info?.name}
			{:catch error}
				Ошибка
			{/await}
		</span>
		
	</div>
{/each}

<div class='w-100'>
	<p class='x-self-c x-cont-c with-info'>
		Полномочия для отчетности в ФНС
	</p>
</div>


{#each fnsPowers as power (power)}
	<div class="w-100">
		<span class='x-cont-l'>
			{#await currentMchdStep.get_power_info(power)}
				Загрузка...
			{:then info} 
				{info?.code} - {info?.name}
			{:catch error}
				Ошибка
			{/await}
		</span>
		
	</div>
{/each}


