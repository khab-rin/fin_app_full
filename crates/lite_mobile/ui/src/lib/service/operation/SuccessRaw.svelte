<script lang="ts">
    import {invoke} from '@tauri-apps/api/core';
    import {onMount} from 'svelte';

    import {operStep} from '$lib/models/Operation/OperationManager.svelte';
    import {OperationSvelte} from '$lib/models/Operation/OperationSvelte.svelte';
    import {OperationType} from '$lib/models/Operation/OperationValues';

    import type {OperationRaw} from '$lib/models/rustModels/OperationRaw';
    import type {Operation} from '$lib/models/rustModels/Operation';
    import type {OperationStep} from '$lib/models/rustModels/OperationStep';
    import {StateProcessor} from '$lib/models/Operation/StatementProcessor.svelte';



    let processor = $derived(new StateProcessor([]));
    let curIndex = $state(0);

    function nextOper() {
        processor.next()
    }

    function prevOper() {
        processor.prev()
    }
    
    onMount (async() => {
        if (OperationType.SuccessRaw in operStep.step) {
            processor = new StateProcessor(operStep.step.SuccessRaw.operations);
        } else {
            const next_step: OperationStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            console.error("System Logic Error, wrong current step");
            operStep.add(next_step);
        }
    });

</script>

<section class='input-section'>
    <div class="input-group">
        <input 
            class="input-field"
            type="text" 
            bind:value={processor.opersSvelte[processor.curInd].data.ctrptyName.value} 
            disabled={true}
            placeholder="строка до 50 знаков"
            class:input-error={processor.opersSvelte[processor.curInd].data.ctrptyName.isValid}
        />
    </div>


</section>


