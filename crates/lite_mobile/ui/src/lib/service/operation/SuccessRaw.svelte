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

    import type {NewContrData} from '$lib/models/rustModels/NewContrData';
	import type { Currency } from '$lib/models/rustModels/Currency';
	import type { Contract } from '$lib/models/rustModels/Contract';

    let processor = $state(new StateProcessor());

    $effect(() => {
        const op = processor.opersSvelte?.[processor.curInd];
        if (op?.data) {
            const _ = op.data.credit.value;
            op.compateAccounts();
        }
    });

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
            processor.opersSvelte[processor.curInd].refreshCtrpty(compInn.value, kpp.value);
            refreshCtrptyPushed = false;
            openCtrpty = false;
        } catch (err) {
            const next_step: OperationStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            console.error("cmd_get_comp_by_inn_kpp FAILED, err = ",  err);
            refreshCtrptyPushed = false;
            operStep.add(next_step);
        }
    }


    let isContrOpened = $state(false);
    let isNewContractOpened = $state(false);
    let isSwitchContractOpened = $state(false);

    let isMakeContrPushed = $state(false);

    let contractNum = new FieldValidator("DocNum", "125");
    let contractDate = new FieldValidator("Date", "18.08.2026");
    let contractTitle= new FieldValidator("String", "договор");
    let contractStDate = new FieldValidator("Date", "18.08.2025");
    let contractEndDate = new FieldValidator("Date", "31.12.2030");
    let contractCurrency = new FieldValidator("Currency", "руб");
    let contractTotAmnt = new FieldValidator("RubF", "1000000");
    let contractDefDays = new FieldValidator("Integ", "15");
    let contractDescr = new FieldValidator("String", "Охуенный договор");

    let isContrValid = $derived(
        !contractNum.isValid ||
        !contractDate.isValid ||
        !contractTitle.isValid ||
        !contractStDate.isValid ||
        !contractEndDate.isValid ||
        !contractCurrency.isValid ||
        !contractTotAmnt.isValid ||
        !contractDefDays.isValid ||
        !contractDescr.isValid);

    function openContract() {
        isContrOpened = !isContrOpened;
        isNewContractOpened = false;
    }

    function openNewContract() {
        isSwitchContractOpened = false;
        isNewContractOpened = !isNewContractOpened;

    }

    function openSwitchContract() {
        isNewContractOpened = false;
        isSwitchContractOpened = !isSwitchContractOpened;
        
    }

    function switchContract(contract: Contract) {
        processor.opersSvelte[processor.curInd].refreshContract(contract)
        isContrOpened = false;
        isNewContractOpened = false;
        isSwitchContractOpened = false;
    }

    async function addNewContract() {
        if (isMakeContrPushed) {return;}

        isMakeContrPushed = true;
        const data: NewContrData = {
            ctrpty_id: processor.opersSvelte[processor.curInd].data.ctrptyId.value,
            contract_num: contractNum.value,
            contract_date: contractDate.value,
            contract_title: contractTitle.value,
            contract_st_date: contractStDate.value,
            contract_end_date: contractEndDate.value,
            contract_currency: contractCurrency.value as Currency,
            contract_tot_amnt: contractTotAmnt.value,
            contract_def_days: contractDefDays.value,
            contract_descr: contractDescr.value
        }

        try {
            processor.opersSvelte[processor.curInd].refreshContracts(data);
            isMakeContrPushed = false;
            isContrOpened = true;
            isNewContractOpened = false;
            isSwitchContractOpened = true;  

        } catch(err) {
            const next_step: OperationStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            console.error("cmd_add_new_contract FAILED, err = ",  err);
            isMakeContrPushed = false;
            operStep.add(next_step);
        }
    }


    function nextOper() {
        processor?.next()
    }

    function prevOper() {
        processor?.prev()
    }
    
    onMount(async () => {
        if (OperationType.SuccessRaw in operStep.step) {
            await processor.init(operStep.step.SuccessRaw.operations);
        } else {
            const next_step: OperationStep = {
                TryLater: {
                    text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"
                }
            };
            console.error("System Logic Error, wrong current step");
            operStep.add(next_step);
        }
    });

</script>

{#if processor}
    <p>Осталось обработать - {processor.unProceed} операций</p>
{/if}

{#if processor && processor.opersSvelte.length > 0}

    <div class="input-group">
        {#if openCtrpty}
            <span class='input-field-span'>
                Инн организации
            </span>
            <input 
                class="input-field"
                type="text" 
                bind:value={compInn.value} 
                placeholder="10 | 12 цифр"
                class:input-error={!compInn.isValid}
            />

            <span class='input-field-span'>
                Кпп организации
            </span>
            <input 
                class="input-field"
                type="text" 
                bind:value={kpp.value} 
                placeholder="0 или 9 цифр"
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
            Название организации контрагента
        </span>
        <input 
            class="input-field"
            type="text" 
            bind:value={processor.opersSvelte[processor.curInd].data.ctrptyName.value}
            disabled={true} 
            class:input-error={!processor.opersSvelte[processor.curInd].data.ctrptyName.isValid}
        />

        <button 
            class='medium-button'
            type='button'
            onclick={switchCtrpty}
            >
            Контрагент
        </button>
    </div>
    
    <div class='input-group'>
        <span class='input-group-span'>
            Информация о договоре
        </span>

        <input 
            class="input-field"
            type="text" 
            bind:value={processor.opersSvelte[processor.curInd].contractStr}
            disabled={true} 
        />
    
        <button
            type='button'
            class='medium-button'
            onclick={openContract}
            >
                Договор
        </button>

        {#if isContrOpened}
            <section class='navi-button-section'>
                <button
                    type='button'
                    class='medium-button'
                    onclick={openSwitchContract}
                >
                    выбрать договор
                </button>

                <button
                    type='button'
                    class='medium-button'
                    onclick={openNewContract}
                >
                    добавить договор
                </button>
            </section>

            {#if isSwitchContractOpened}
                <section class='wide-button-section'>
                    <span class='wide-button-span'>Выберите нужный договор</span>
                    {#each processor.opersSvelte[processor.curInd]._allPossContracts as contract}
                        <div class='wide-button-group'>
                            <span class='wide-button-span'>{processor.getContractInfo(contract)}</span>
                            <button
                                type='button'
                                class='wide-button'
                                onclick={() => switchContract(contract)}
                            >
                                Выбрать договор
                            </button>
                        </div>
                        
                    {/each}


                </section>
            {/if}


            {#if isNewContractOpened}
                <section class="input-section">
                    <span class='input-field-span'>Номер договора</span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={contractNum.value} 
                        placeholder="строка до 50 знаков"
                        class:input-error={!contractNum.isValid}
                    />

                    <span class='input-field-span'>Дата договора</span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={contractDate.value} 
                        placeholder="дд.мм.гггг"
                        class:input-error={!contractDate.isValid}
                    />

                    <span class='input-field-span'>Название договора</span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={contractTitle.value} 
                        placeholder="строка до 50 знаков"
                        class:input-error={!contractTitle.isValid}
                    />

                    <span class='input-field-span'>Дата начала</span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={contractStDate.value} 
                        placeholder="дд.мм.гггг"
                        class:input-error={!contractStDate.isValid}
                    />

                    <span class='input-field-span'>Дата завершения</span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={contractEndDate.value} 
                        placeholder="дд.мм.гггг"
                        class:input-error={!contractEndDate.isValid}
                    />

                    <span class='input-field-span'>Валюта договора</span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={contractCurrency.value} 
                        placeholder="РУБ"
                        class:input-error={!contractCurrency.isValid}
                    />

                    <span class='input-field-span'>Сумма договора</span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={contractTotAmnt.value} 
                        placeholder="Сумма в валюте договора"
                        class:input-error={!contractTotAmnt.isValid}
                    />

                    <span class='input-field-span'>Рассрочка в днях</span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={contractDefDays.value} 
                        placeholder="количество дней"
                        class:input-error={!contractDefDays.isValid}
                    />

                    <span class='input-field-span'>Описание</span>
                    <input 
                        class="input-field"
                        type="text" 
                        bind:value={contractDescr.value} 
                        placeholder="строка до 50 знаков"
                        class:input-error={!contractDescr.isValid}
                    />

                    <button class='medium-button'
                        type='button'
                        onclick={addNewContract}
                        disabled={isMakeContrPushed || isContrValid}
                    >
                        Добавить договор
                    </button>
                </section>
            {/if}
        {/if}
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
            placeholder="Номер счета"
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
            disabled={false}
            placeholder="Номер счета"
            class:input-error={!processor.opersSvelte[processor.curInd].data.credit.isValid ||
                !processor.opersSvelte[processor.curInd].isAccountsCompatible
            }
        />
    </div>

    <div class="input-group">
        <span class='input-field-span'>
            Сумма операции
        </span>
        <input
            class = 'input-field'
            type='text'
            bind:value={processor.opersSvelte[processor.curInd].data.amount.value}
            disabled={false}
            placeholder='xxx.xx'
            class:input-error={!processor.opersSvelte[processor.curInd].data.amount.isValid}
        />
    </div>

    <div class="input-group">
        <span class='input-field-span'>
            Дата операции
        </span>
        <input
            class = 'input-field'
            type='text'
            bind:value={processor.opersSvelte[processor.curInd].data.operDate.value}
            disabled={true}
            placeholder='xx.xx.xxxx'
            class:input-error={!processor.opersSvelte[processor.curInd].data.operDate.isValid}
        />
    </div>

    <div class="input-group">
        <span class='input-field-span'>
            Тип банковского документа
        </span>
        <input
            class = 'input-field'
            type='text'
            bind:value={processor.opersSvelte[processor.curInd].data.docType.value}
            disabled={true}
            class:input-error={!processor.opersSvelte[processor.curInd].data.docType.isValid}
        />
    </div>

    <div class="input-group">
        <span class='input-field-span'>
            Номер банковского документа
        </span>
        <input
            class = 'input-field'
            type='text'
            bind:value={processor.opersSvelte[processor.curInd].data.docNum.value}
            disabled={true}
            class:input-error={!processor.opersSvelte[processor.curInd].data.docNum.isValid}
        />
    </div>

    <div class="input-group">
        <span class='input-field-span'>
            Дата банковского документа
        </span>
        <input
            class = 'input-field'
            type='text'
            bind:value={processor.opersSvelte[processor.curInd].data.docDate.value}
            disabled={true}
            class:input-error={!processor.opersSvelte[processor.curInd].data.docDate.isValid}
        />
    </div>

    <div class="input-group">
        <span class='input-field-span'>
            Дата составления операции
        </span>
        <input
            class = 'input-field'
            type='text'
            bind:value={processor.opersSvelte[processor.curInd].data.entrDate.value}
            disabled={true}
            class:input-error={!processor.opersSvelte[processor.curInd].data.entrDate.isValid}
        />
    </div>

{/if}

