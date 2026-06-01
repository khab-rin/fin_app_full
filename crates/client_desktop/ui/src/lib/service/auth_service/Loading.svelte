<script lang="ts">
    import { onMount } from "svelte";
    import { currAuthStep } from "$lib/models/svelte_models/auth_service/SvelteAuthStep.svelte";
    import { invoke } from "@tauri-apps/api/core";

    import type { NickData } from "$lib/models/NickData";

    onMount(async() => {
        try {
            const data = await invoke<NickData>('cmd_get_nick_names');
            if (!data.nick_names || data.nick_names.length === 0) {
                currAuthStep.step = { NeedPassword: {text: "Пользователь не найден на устройстве, требуется авторизоваться по паролю или пройти регистрацию"} };
            } else {
                currAuthStep.nick_names = data;
                currAuthStep.step = { NickName: { text: "Выберите из списка нужного пользователя, в случае отсутствия авторизуйтесь через пароль, либо зарегистрируйтесь"} };
            }
        } catch (err) {
            currAuthStep.step = { TryLater: {text:"Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            console.error("Error:", err);
        }
    });

</script>

<div>
    <p class="info-text">
        {currAuthStep.currentText}
    </p>
</div>