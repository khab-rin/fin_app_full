<script lang='ts'>
    import {currAuthStep} from "$lib/models/Auth/AuthStep.svelte";
    import {invoke} from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    import {AuthStepType} from "$lib/models/Auth/AuthValues";
    import type {AuthStep} from "$lib/models/rustModels/AuthStep";
    
    let isPolling = $state(false);
    
    // 1. Добавляем реактивную переменную для отображения секунд в HTML
    let countdown = $state(4); 
    
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

        // 2. Интервал теперь срабатывает каждую 1 секунду вместо 4
        const interval = setInterval(() => {
            if (!isPolling) return;

            // Уменьшаем секундомер на единицу
            countdown -= 1;

            // Когда счетчик дошел до 0 — пришло время стучаться на бэкенд
            if (countdown <= 0) {
                countdown = 4; // Сбрасываем визуальный счетчик обратно на 4
                poll_back_api(); // Перенаправляем запрос в Tauri
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

    <!-- Блок времени -->
    <div class="pooling-timestamp">
        Следующая проверка через: <strong>{countdown} сек</strong>
    </div>
</div>
