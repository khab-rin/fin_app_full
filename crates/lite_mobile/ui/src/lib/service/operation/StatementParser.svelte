<script lang='ts'>
    import {onMount} from 'svelte';
    import {invoke} from '@tauri-apps/api/core';
    import {open as OpenFileDialog} from '@tauri-apps/plugin-dialog';

    import type {OperationStep} from '$lib/models/rustModels/OperationStep';
    import type {RasBicAcc} from '$lib/models/rustModels/RasBicAcc';
	import { operStep } from '$lib/models/Operation/OperationManager.svelte';


    let path = $state('');
    let selectedBankAcc = $state<RasBicAcc | null>(null);
    let bankAccounts = $state<RasBicAcc[]>([]);

    let isPushedAccLoad = $state(false);
    let isPushFileLoad = $state(false);
    let isPushParseStatement = $state(false);

    let dialogRef = $state<HTMLDialogElement | null>(null);

    let parseStatementDisabled = $derived(
        isPushParseStatement ||
        path === '' ||
        selectedBankAcc === null
    );

    function openAccModal() {
        if (dialogRef) dialogRef.showModal();
    }

    function closeAccModal() {
        if (dialogRef) dialogRef.close();
    }

    function selectAcc(acc: RasBicAcc) {
        if (isPushedAccLoad) return;

        isPushedAccLoad = true;

        selectedBankAcc = acc;
        closeAccModal();

        isPushedAccLoad = false;
        
    }

    async function selectFile() {
        if (isPushFileLoad) return;
        isPushFileLoad = true;

        try {
            const selected = await OpenFileDialog({
                multiple: false,
                directory: false,
                title: "Выберите банковскую выписку",
                filters: [{name: "документ txt", extensions: ["txt"]}]
            });

            if (selected) {
                if (typeof selected === 'string') {
                    path = selected;
                    isPushFileLoad = false;
                } else {
                    isPushFileLoad = false;
                }
            } else {
                isPushFileLoad = false;
            }
        } catch(err) {
            const next_step: OperationStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            console.error("get_own_bank_accs FAILED BY PATH LOAD, err = ", err);
            operStep.add(next_step);
        }
    }


    onMount(async() => {
        try {
            bankAccounts = await invoke<RasBicAcc[]>('get_own_bank_accs', {});

        } catch (err) {
            const next_step: OperationStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            console.error("get_own_bank_accs FAILED, err = ", err);
            operStep.add(next_step);
        }
    });

    async function parseStatement() {
        if (isPushParseStatement) return;
        if (path == '') return;
        if (selectedBankAcc == null) return;

        isPushParseStatement = true;
        
        let data = {
            RasBicAcc: selectedBankAcc,
            path: path
        };


        try {
            const next_step: OperationStep = await invoke<OperationStep>("cmd_load_bank_statement", {data});
            isPushParseStatement = false;
            operStep.add(next_step);
        } catch(err) {
            console.error("FUN cmd_load_bank_statement FAILED, err = ", err);
            const next_step: OperationStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            isPushParseStatement = false;
            operStep.add(next_step);

        }
    }


</script>

<section class='input-section'>
    <div class='input-wide-button-grid'>
        <label class="input-wide-button-grid-label" for="statementPath">
            Укажите путь до файла выписки
        </label>
        <input
            type='text'
            id='statementPath'
            value={path}
            class='input-field'

        />
        <button
            type='button'
            id='statementPathButton'
            class='wide-button'
            onclick={selectFile}
            disabled={isPushFileLoad}
            >
            Загрузите файл выписки
        </button>

    </div>
</section>


<div class="selector-wrapper">
    <button
        type="button"
        class="wide-button"
        disabled={isPushedAccLoad}
        
        onclick={openAccModal}
    >

        <span class="wide-button-span">
             - {selectedBankAcc ? "Текущий выбор" : "Нажмите для просмотра списка"}
        </span>

        <span class="wide-button-span">
            {selectedBankAcc || "Выбрать аккаунт на устройстве"} >
        </span>
    </button>
</div>

<dialog 
    bind:this={dialogRef} 
    class="selector-dialog"
    onclick={(e) => { if (e.target === dialogRef) closeAccModal(); }}
>

    <h5>Выбор счета</h5>


    <div class="selector-dialog-content">
        {#if bankAccounts.length > 0}
            <ul class="dialog-list">
                {#each bankAccounts as acc (acc)}
                    <li>
                        <div class="wide-button-grid">
                            <button 
                                type="button" 
                                class="wide-button"
                                onclick={() => selectAcc(acc)}
                            >
                                <span class="wide-button-span">
                                    Счет: {acc.ras_acc} (БИК: {acc.bic})
                                </span>
                            </button>
                        </div>
                    </li>
                {/each}
            </ul>

            <div class="wide-button-grid">
                <button class="wide-button"
                    type="button"
                    onclick={closeAccModal}
                    >
                    
                    <span class="wide-button-span">
                        Отмена
                    </span>


                </button>
            </div>

        {:else}
            <p>На этом устройстве еще нет сохраненных счетов</p>
        {/if}
    </div>

</dialog>

<div class = 'main-button-group'>
    <button
        type='button'
        class='main-button'
        disabled={parseStatementDisabled}
        onclick ={parseStatement}
    >

        Загрузить выписку

    </button>
</div>