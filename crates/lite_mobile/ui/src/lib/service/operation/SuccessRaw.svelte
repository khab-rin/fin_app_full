<script lang="ts">

    import { info } from '@tauri-apps/plugin-log';

    import {invoke} from '@tauri-apps/api/core';
    import {onMount} from 'svelte';

    import {operStep} from '$lib/models/Operation/OperationManager.svelte';
    import {OperationSvelte} from '$lib/models/Operation/OperationSvelte.svelte';
    import {OperationType} from '$lib/models/Operation/OperationValues';

    import {StateProcessor} from '$lib/models/Operation/StatementProcessor.svelte';
	import { FieldValidator } from '$lib/models/Auth/FieldValidator.svelte';

    import type {OperationRaw} from '$lib/models/rustModels/OperationRaw';
    import type {Operation} from '$lib/models/rustModels/Operation';
    import type {OperationStep} from '$lib/models/rustModels/OperationStep';
    import type {Company} from '$lib/models/rustModels/Company';
    

    let processor = $state(new StateProcessor([]));
    let curIndex = $state(0);

    let kpp = new FieldValidator("Kpp", "");
    let compInn  = new FieldValidator("CompInn", "");
    let openRefreshCtrpty = $state(false);
    let refreshCtrpyDisable = $derived(!kpp.isValid || !compInn.isValid);

    function openCloseRefreshCtrpty() {
        openRefreshCtrpty = !openRefreshCtrpty;
    }

    async function refreshCtrpty() {
        if (refreshCtrpyDisable) {return;}
        try {
            let data = {compInn: compInn.value, kpp: kpp.value};
            const newCompany = await invoke<Company>("cmd_get_comp_by_inn_kpp", data);
        } catch (err) {
            const next_step: OperationStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            console.error("cmd_get_comp_by_inn_kpp FAILED, err = ",  err);
            operStep.add(next_step);
        }
    }


    function nextOper() {
        processor?.next()
    }

    function prevOper() {
        processor?.prev()
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

{#if processor && processor.opersSvelte.length > 0}
    <section class='input-section'>

        <div class="input-group">
            
            <span class='input-field-span'>
                Название организации
            </span>

            {#if openRefreshCtrpty}
                <div class="input-group">
                    <span class='input-field-span'>
                        Инн организации
                    </span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={compInn.value} 
                        disabled={true}
                        placeholder="строка до 50 знаков"
                        class:input-error={!compInn.isValid}
                    />
                </div>

                <div class="input-group">
                    <span class='input-field-span'>
                        Кпп организации
                    </span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={kpp.value} 
                        disabled={true}
                        placeholder="строка до 50 знаков"
                        class:input-error={!kpp.isValid}
                    />
                </div>

                <div class='medium-button-section'>
                    <div class='medium-button-group'>

                    </div>


                </div>
            {/if}

            
            <input 
                class="input-field"
                type="text" 
                bind:value={processor.opersSvelte[processor.curInd].data.ctrptyName.value} 
                disabled={true}
                placeholder="строка до 50 знаков"
                class:input-error={!processor.opersSvelte[processor.curInd].data.ctrptyName.isValid}
            />
            <button class='medium-button'
                type='button'
                id='statementPathButton'
                onclick={openCloseRefreshCtrpty}
                >
                Сменить организацию
            </button>
        </div>




        <div class='input-group'>
            <span class='input-field-span'>
                Счет Дебет
            </span>
            <input 
                class="input-field"
                type="text" 
                bind:value={processor.opersSvelte[processor.curInd].data.debet.value} 
                disabled={true}
                placeholder="строка до 50 знаков"
                class:input-error={!processor.opersSvelte[processor.curInd].data.debet.isValid}
            />
        </div>

        <div class="input-group">
            <input 
                class="input-field"
                type="text" 
                bind:value={processor.opersSvelte[processor.curInd].data.credit.value} 
                disabled={true}
                placeholder="строка до 50 знаков"
                class:input-error={!processor.opersSvelte[processor.curInd].data.credit.isValid}
            />
        </div>


    </section>
{/if} -->
 -->

