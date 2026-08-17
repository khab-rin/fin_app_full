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



    let kpp = new FieldValidator("Kpp", "");
    let compInn  = new FieldValidator("CompInn", "");
    let openCtrpty = $state(false);
    let refreshCtrptyPushed = $state(false);

    function switchCtrpty() {
        openCtrpty = !openCtrpty;
    }
    
    async function refreshCtrpty() {
        if (refreshCtrptyPushed) {return;}
        try {
            refreshCtrptyPushed = true;
            let data = {compInn: compInn.value, kpp: kpp.value};
            const newCompany: Company | null = await invoke<Company>("cmd_get_comp_by_inn_kpp", data);
            processor.opersSvelte[processor.curInd].refreshCtrpty(newCompany);
            refreshCtrptyPushed = false;
            
        } catch (err) {
            const next_step: OperationStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            console.error("cmd_get_comp_by_inn_kpp FAILED, err = ",  err);
            refreshCtrptyPushed = false;
            operStep.add(next_step);
        }
    }


    let isContrOpened = $state(false);
    let isNewContractOpened = $state(false);
    let isMakeContrPushed = $state(false);
    

    let contractNum = new FieldValidator("DocNum", "");
    let contractDate = new FieldValidator("Date", "");
    let contractTitle= new FieldValidator("String", "");
    let contractStDate = new FieldValidator("Date", "");
    let contractEndDate = new FieldValidator("Date", "");
    let contractCurrency = new FieldValidator("Currency", "");
    let contractTotAmnt = new FieldValidator("RubF", "");
    let contractDefDays = new FieldValidator("U32", "");
    let contractDescr = new FieldValidator("String", "");

    let isContrValid = $state(
        contractNum.isValid ||
        contractDate.isValid ||
        contractTitle.isValid ||
        contractStDate.isValid ||
        contractEndDate.isValid ||
        contractCurrency.isValid ||
        contractTotAmnt.isValid ||
        contractDefDays.isValid ||
        contractDescr.isValid);

    function switchContract() {
        isContrOpened = !isContrOpened;
    }

    function switchNewContract() {
        isNewContractOpened = !isNewContractOpened;
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
            {#if openCtrpty}
                <span class='input-field-span'>
                    Инн организации
                </span>
                <input 
                    class="input-field"
                    type="text" 
                    bind:value={compInn.value} 
                    placeholder="строка до 50 знаков"
                    class:input-error={!compInn.isValid}
                />

                <span class='input-field-span'>
                    Кпп организации
                </span>
                <input 
                    class="input-field"
                    type="text" 
                    bind:value={kpp.value} 
                    placeholder="строка до 50 знаков"
                    class:input-error={!kpp.isValid}
                />

                <button
                    type='button'
                    class='medium-button'
                    disabled={!compInn.isValid || !kpp.isValid || refreshCtrptyPushed}
                    onclick={refreshCtrpty}
                >
                    сменить контрагента
                </button>
            {/if}
            
            <span class='input-field-span'>
                Название организации
            </span>

            <strong>{processor.opersSvelte[processor.curInd].data.ctrptyName.value}</strong>

            <button 
                class='medium-button'
                type='button'
                onclick={switchCtrpty}
                >
                Редактировать
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
            <span class='input-field-span'>
                Счет Кредит
            </span>
            <input 
                class="input-field"
                type="text" 
                bind:value={processor.opersSvelte[processor.curInd].data.credit.value} 
                disabled={true}
                placeholder="строка до 50 знаков"
                class:input-error={!processor.opersSvelte[processor.curInd].data.credit.isValid}
            />
        </div>

        <div class='input-group'>
            <span class='input-group-span'>
                Информация о договоре
            </span>
            <strong>{processor.opersSvelte[processor.curInd].contractStr}</strong>

            <button
                type='button'
                class='medium-button'
                onclick={switchContract}
                >
                    Договор
            </button>

            <section class='navi-button-section'>
                <button
                    type='button'
                    class='medium-button'
                    onclick={switchContract}
                >
                    выбрать договор
                </button>

                <button
                    type='button'
                    class='medium-button'
                    onclick={switchNewContract}
                >
                    добавить договор
                </button>
            </section>


            {#if isNewContractOpened}
                <section class="input-section">
                    <span class='input-field-span'>Номер договора</span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={contractNum.value} 
                        disabled={true}
                        placeholder="строка до 50 знаков"
                        class:input-error={!contractNum.isValid}
                    />

                    <span class='input-field-span'>Дата договора</span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={contractDate.value} 
                        disabled={true}
                        placeholder="строка до 50 знаков"
                        class:input-error={!contractNum.isValid}
                    />

                    <span class='input-field-span'>Название договора</span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={contractTitle.value} 
                        disabled={true}
                        placeholder="строка до 50 знаков"
                        class:input-error={!contractNum.isValid}
                    />

                    <span class='input-field-span'>Дата начала</span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={contractStDate.value} 
                        disabled={true}
                        placeholder="строка до 50 знаков"
                        class:input-error={!contractNum.isValid}
                    />

                    <span class='input-field-span'>Дата завершения</span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={contractEndDate.value} 
                        disabled={true}
                        placeholder="строка до 50 знаков"
                        class:input-error={!contractNum.isValid}
                    />

                    <span class='input-field-span'>Валюта договора</span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={contractCurrency.value} 
                        disabled={true}
                        placeholder="строка до 50 знаков"
                        class:input-error={!contractNum.isValid}
                    />

                    <span class='input-field-span'>Сумма договора</span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={contractTotAmnt.value} 
                        disabled={true}
                        placeholder="строка до 50 знаков"
                        class:input-error={!contractNum.isValid}
                    />

                    <span class='input-field-span'>Рассрочка в днях</span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={contractDefDays.value} 
                        disabled={true}
                        placeholder="строка до 50 знаков"
                        class:input-error={!contractNum.isValid}
                    />

                    <span class='input-field-span'>Описание</span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={contractDescr.value} 
                        disabled={true}
                        placeholder="строка до 50 знаков"
                        class:input-error={!contractNum.isValid}
                    />


                </section>
            {/if}

            


        </div>




    </section>
{/if}

