<script lang="ts">
    import { onMount } from "svelte";
    import { currAuthStep } from "$lib/models/svelte_models/auth_service/SvelteAuthStep.svelte";
    import type {AuthStep} from "$lib/models/AuthStep";

    onMount(async() => {
        try {
            if (currAuthStep.nick_names.nick_names.length == 0) {
                let next_step: AuthStep = {NeedPassword: {text: "Пользователь не найден на устройстве, требуется авторизоваться по паролю или пройти регистрацию"}};
                currAuthStep.add(next_step);
            } else {
                let next_step: AuthStep = {NickName: {text: "Выберите из списка нужного пользователя, в случае отсутствия авторизуйтесь через пароль, либо зарегистрируйтесь"}};
                currAuthStep.add(next_step);
            }
        } catch (err) {
            console.error("Error:", err);
            const next_step: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            currAuthStep.add(next_step);
        }
    });

</script>

<div>
    <p class="info-text">
        {currAuthStep.currentText}
    </p>
</div>