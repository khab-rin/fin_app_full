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

<section class='w-100'>
    <h3 class='t-fill w-g100 x-ce'> Полномочия для доступа к разделам системы </h3>
</section>


{#each homePowers as power (power)}
	<div class="w-100">
		<span class='w-g100 x-le'>
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

<section class='w-100'>
    <h3 class='t-fill w-g100 x-ce'> Полномочия для ЭДО с контрагентами </h3>
</section>


{#each btbPowers as power (power)}
	<div class="w-100">
		<span class='w-g100 x-le'>
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

<section class='w-100'>
    <h3 class='t-fill w-g100 x-ce'> Полномочия для отчетности в ФНС </h3>
</section>


{#each fnsPowers as power (power)}
	<div class="w-100">
		<span class='w-g100 x-le'>
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


