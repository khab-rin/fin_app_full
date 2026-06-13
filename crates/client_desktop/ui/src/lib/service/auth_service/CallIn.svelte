<script lang='ts'>
    import {currAuthStep} from "$lib/models/Auth/AuthStep.svelte";
    import {invoke} from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    import {AuthStepType} from "$lib/models/Auth/AuthValues";

    import type {AuthStep} from "$lib/models/rustModels/AuthStep";
	
    
    let isPolling = $state(false);
    
    const externalId = AuthStepType.CallIn in currAuthStep.step ? currAuthStep.step.CallIn.external_id : "";
    const phone = AuthStepType.CallIn in currAuthStep.step ? currAuthStep.step.CallIn.phone : "";

    let err_step: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};

    let data = {
        externalId: externalId,
        nick: currAuthStep.data.nick.value
    };

    async function poll_back_api() {
        if (!externalId) return;
        try {
            let next_step = await invoke<AuthStep>("cmd_session_by_tel_call", data);
            if (AuthStepType.CallInWaiting in next_step) {
                if (currAuthStep.step && AuthStepType.CallIn in currAuthStep.step) {
                    currAuthStep.step.CallIn.text = "Звонок по указанному номеру не был осуществлен, позвоните по этому номеру"; 
                }
            } else {
                currAuthStep.add(next_step);
            }
        } catch (err) {
            console.error("Error:", err);
            currAuthStep.add(err_step);
        }
    }

    onMount(() => {
        if (!externalId) {
            currAuthStep.add(err_step);
            return;
        }

        isPolling = true;


        const interval = setInterval(() => {
            if (isPolling) {
                poll_back_api();
            }
        }, 4000);

        return () => {
            clearInterval(interval);
        };
    });

</script>


<div class="auth-card">
    <p class="auth-text-step">
        {currAuthStep.currentText}
    </p>

    <div class="callin">

        {#if phone}
            <div class="phone-display">
                <span class="label">Номер для звонка:</span>
                <span class="phone-number">{phone}</span>
            </div>
        {/if}

        <div class="actions">
            <a 
                href="tel:{phone}"
                class="call-button" 
                class:disabled={isPolling}
                aria-disabled={isPolling}
                onclick={(e) => {
                    if (isPolling) {
                        e.preventDefault();
                    }
                }}
            >
                {#if isPolling}
                    <span class="spinner"></span>
                    Проверяем звонок...
                {:else}
                    Позвонить
                {/if}
            </a>
        </div>
    </div>
</div>
