<script lang="ts">
    import { onMount } from "svelte";

    import { currAuthStep } from "$lib/models/Auth/AuthStep.svelte";
    
    import type {AuthStep} from "$lib/models/rustModels/AuthStep";

    onMount(async() => {
        try {
			await currAuthStep.init();
            if (currAuthStep.nick_names.length == 0) {
                let nextStep: AuthStep = {Password: {text: "Пользователь не найден на устройстве, требуется авторизоваться по паролю или пройти регистрацию"}};
                currAuthStep.step = nextStep;
            } else {
                let nextStep: AuthStep = {NickName: {text: "Выберите из списка нужного пользователя, в случае отсутствия авторизуйтесь через пароль, либо зарегистрируйтесь"}};
                currAuthStep.step = nextStep;
            }
        } catch (err) {
            console.error("Error:", err);
            const nextStep: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            currAuthStep.step = nextStep;
        }
    });

</script>
