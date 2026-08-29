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
            const next_step: MchdStep = {TryLater: {text: "Критическая ошибка на устройстве..."}};
            currentMchdStep.add(next_step);
        }
    });
</script>

<section class='group-one'>
    <h3 class='h3'> Полномочия для доступа к разделам системы </h3>

    <ul class='info-group'>
        {#each homePowers as power (power)}
            <li class='key-value-info'>
                {#await currentMchdStep.get_power_info(power)}
                    <span class="green-field-span for={power}">Загрузка...</span>
                {:then info} 
					<label class="green-field-label" for={power}>{power}</label>
                    <span class="info-value-span" id={power}>{info?.name}</span>
                {:catch error}
                    <span title={error} class="info-value-error-span">Ошибка</span>
                {/await}
            </li>
        {/each}
    </ul>
</section>

<section class='group-one'>
    <h3 class='h3'> Полномочия для отчетности в ФНС </h3>

	<ul class='info-group'>
        {#each fnsPowers as power (power)}
            <li class='key-value-info'>
                {#await currentMchdStep.get_power_info(power)}
                    <span class="green-field-span for={power}">Загрузка...</span>
                {:then info} 
					<label class="green-field-label" for={power}>{power}</label>
                    <span class="info-value-span" id={power}>{info?.name}</span>
                {:catch error}
                    <span title={error} class="info-value-error-span">Ошибка</span>
                {/await}
            </li>
        {/each}
    </ul>
</section>

<section class='group-one'>
    <h3 class='h3'> Полномочия для ЭДО с контрагентами </h3>

    <ul class='info-group'>
        {#each btbPowers as power (power)}
            <li class='key-value-info'>
                {#await currentMchdStep.get_power_info(power)}
                    <span class="green-field-span for={power}">Загрузка...</span>
                {:then info} 
					<label class="green-field-label" for={power}>{power}</label>
                    <span class="info-value-span" id={power}>{info?.name}</span>
                {:catch error}
                    <span title={error} class="info-value-error-span">Ошибка</span>
                {/await}
            </li>
        {/each}
    </ul>
</section>