<script lang='ts'>
    import {currAuthStep} from "$lib/models/Auth/AuthStep.svelte";
    import {invoke} from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    import {AuthStepType} from "$lib/models/Auth/AuthValues";
    import type {AuthStep} from "$lib/models/rustModels/AuthStep";
	import { pageManager } from "$lib/models/MainManager/MainManager.svelte";
    
    let isPolling = $state(false);
    
    let countdown = $state(4); 
    
    const externalId = AuthStepType.CallIn in currAuthStep.step ? currAuthStep.step.CallIn.external_id : "";
    const phone = AuthStepType.CallIn in currAuthStep.step ? currAuthStep.step.CallIn.phone : "";

    let err_step: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};

    let data = {
        externalId: externalId,
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
				pageManager.compName = await invoke<string>("cmd_get_comp_name", {});
                currAuthStep.step = next_step;
            }
        } catch (err) {
            console.error("Error:", err);
            currAuthStep.step = err_step;
        }
    }

    onMount(() => {
        if (!externalId) {
            currAuthStep.step = err_step;
            return;
        }

        isPolling = true;
        const interval = setInterval(() => {
            if (!isPolling) return;
            countdown -= 1;
            if (countdown <= 0) {
                countdown = 4;
                poll_back_api();
            }
        }, 1000);

        return () => {
            clearInterval(interval);
        };
    });
</script>


<div class="pooling-section">
    <!-- Блок статуса -->
    <div class="pooling-active">
        <span class="pooling-dot"></span>
        Автоматический мониторинг: Активен
    </div>

    <div>
        <p> Совершите бесплатный звонок по указанному номеру </p>
        <span class="pooling-info-span">
            {phone}
        </span>
    </div>

    <div class="pooling-timestamp">
        Следующая проверка через: <strong>{countdown} сек</strong>
    </div>
</div>
