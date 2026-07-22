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

<section class='info-section'>
    <h3 class='h3'> Полномочия для доступа к разделам системы </h3>

    <ul class='info-group'>
        {#each homePowers as power (power)}
            <li class='key-value-info'>
                
                {#await currentMchdStep.get_power_info(power)}
                    
                    <span class="info-value-span">Загрузка...</span>
                {:then info} 
                    <span class='info-key-span'>{info?.code}</span>
                    <span class="info-value-span">{info?.name}</span>
                {:catch error}
                    <span title={error} class="info-value-error-span">Ошибка</span>
                {/await}
            </li>
        {/each}
    </ul>
</section>

<section class='info-section'>
    <h3 class='h3'> Полномочия для отчетности в ФНС </h3>

    <ul class='info-group'>
        {#each fnsPowers as power (power)}
            <li class='key-value-info'>
                <span class='info-key-span'>{power}</span>
                {#await currentMchdStep.get_power_info(power)}
                    <span class="info-value-span">Загрузка...</span>
                {:then info} 
                    <span class="info-value-span">{info?.name}</span>
                {:catch error}
                    <span title={error} class="info-value-error-span">Ошибка</span>
                {/await}
            </li>
        {/each}
    </ul>
</section>

<section class='info-section'>
    <h3 class='h3'> Полномочия для ЭДО с контрагентами </h3>

    <ul class='info-group'>
        {#each btbPowers as power (power)}
            <li class='key-value-info'>
                <span class='info-key-span'>{power}</span>
                {#await currentMchdStep.get_power_info(power)}
                    <span class="info-value-span">Загрузка...</span>
                {:then info} 
                    <span class="info-value-span">{info?.name}</span>
                {:catch error}
                    <span title={error} class="info-value-error-span">Ошибка</span>
                {/await}
            </li>
        {/each}
    </ul>
</section>