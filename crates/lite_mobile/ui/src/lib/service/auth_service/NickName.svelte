<script lang='ts'>

    import {onMount} from "svelte";
    import { invoke } from "@tauri-apps/api/core";
	import {dialogBackdrop} from '$lib/rules/dialogBorders'

	import { pageManager } from "$lib/models/MainManager/MainManager.svelte";
    import { currAuthStep } from "$lib/models/Auth/AuthStep.svelte";
    import type { AuthStep } from '$lib/models/rustModels/AuthStep';

    let IsPushed = $state(false);

    let currNick = $state("");

    function openAccountsModal(id: string) {
        (document.getElementById(id) as HTMLDialogElement)?.showModal();
    }

    function selectElem(id: string, value: string): string {
		(document.getElementById(id) as HTMLDialogElement)?.close()
		return value;
	}

    async function call_nick_handle(selectedNick: string) {
        if (IsPushed) return;
        
        IsPushed = true;
        
        try {
            let nextStep = await invoke<AuthStep>('cmd_session_by_nick', { nick: selectedNick });
			pageManager.compName = await invoke<string>("cmd_get_comp_name", {});
            IsPushed = false;
            currAuthStep.step = nextStep;
        } catch (err) {
            let nextStep: AuthStep = { 
                TryLater: { text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"} 
            };
            console.error("ОШИБКА В call_nick_handle:", err);
            IsPushed = false; 
            currAuthStep.step = nextStep;
        }
    }

    onMount(async() => {
        try {
            currAuthStep.nick_names = await invoke<string []>('cmd_get_nick_names');
            if (currAuthStep.nick_names.length == 0) {
                let nextStep: AuthStep = {Password: {text: "Пользователь не найден на устройстве, требуется авторизоваться по паролю или пройти регистрацию"}};
                currAuthStep.step = nextStep;
            }
        } catch(err) {
            console.error("NicnName page FAILED BY 'cmd_get_nick_names, err = ", err);
            const nextStep: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            currAuthStep.step = nextStep;
        }
    });

</script>