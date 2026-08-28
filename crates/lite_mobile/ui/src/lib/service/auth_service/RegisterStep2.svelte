<script lang='ts'>
    import {open as openFileDialog} from '@tauri-apps/plugin-dialog';
    import {invoke} from '@tauri-apps/api/core';
    import {currAuthStep} from '$lib/models/Auth/AuthStep.svelte';

    import type {AuthStep} from '$lib/models/rustModels/AuthStep';
    import type {RegFilesPathData} from '$lib/models/rustModels/RegFilesPathData';


    let isPushedRegister = $state(false);
    let isLoadJsonFile = $state(false);
    let isLoadSignFile = $state(false);

    let jsonFilePath = $state('');
    let signFilePath = $state('');

    let IsDataReady = $derived(
        jsonFilePath.length == 0 ||
        signFilePath.length == 0
    );


    async function getJsonFilePath() {
        if (isLoadJsonFile) return;
        isLoadJsonFile = true;
        try {
            const selected = await openFileDialog({
                multiple: false,
                directory: false,
                title: "Выберите ранее созданный json файл регистрации",
                filters: [{name: 'документ json', extensions: ['json']}]
            });

            if (selected) {
                if (typeof selected === 'string') {
                    jsonFilePath = selected
                } else {
                    isLoadJsonFile = false;
                }
            } else {
                isLoadJsonFile = false;
            }

        } catch (err) {
            console.error("GET FILE PATH FAILED, ERROR = ", err);
            const next_step: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            isLoadJsonFile = false;
            currAuthStep.add(next_step);
        }
    }

    async function getSignFilePath() {
        if (isLoadSignFile) return;
        isLoadSignFile = true;
        try {
            const selected = await openFileDialog({
                multiple: false,
                directory: false,
                title: "Выберите отсоединенный файл подписи",
                filters: [{name: 'документ подписи', extensions: ['sig', 'p7s']}]
            });

            if (selected) {
                if (typeof selected === 'string') {
                    signFilePath = selected
                } else {
                    isLoadSignFile = false;
                }
            } else {
                isLoadSignFile = false;
            }

        } catch (err) {
            console.error("GET FILE PATH FAILED, ERROR = ", err);
            const next_step: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            isLoadSignFile = false;
            currAuthStep.add(next_step);
        }
    }

    async function register() {
        if (isPushedRegister) return;
        isPushedRegister = true;
        let data: RegFilesPathData = {
            jsonPath: jsonFilePath,
            signPath: signFilePath
        };
        try {
            const next_step: AuthStep = await invoke<AuthStep>("cmd_register_step2", {
                data: data
            });

            isPushedRegister = false;
            currAuthStep.add(next_step);
        } catch (err) {
            console.error("cmd_register FAILED, ERROR = ", err);
            const next_step: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            isPushedRegister = false;
            currAuthStep.add(next_step);
        }
    }

</script>



<section class="group-one">
    <div>
        <label class="green-field-label" for="jsonFilePath">
            Загрузите путь до json файла
        </label>
        <input
            type="text"
            id="jsonFilePath"
            value={jsonFilePath}
            class="green-field"
			disabled={true}
        />
        <button
            type="button"
            id="xmlFileButton"
            class="green-button"
            onclick={getJsonFilePath}
            disabled={isLoadJsonFile}
        >
            Загрузите xml файл
        </button>

    </div>

    <div>
        <label class="green-field-label" for="signFilePath">
            Загрузите путь до файла ЭЦП
        </label>
  
        <input
            type="text"
            id="signFilePath"
            value={signFilePath}
            class="green-field"
			disabled={true}
        />
        <button
            type="button"
            id="sigFileButton"
            class="green-button"
            onclick={getSignFilePath}
            disabled={isLoadSignFile}
        >
            Загрузите файл подписи
        </button>

    </div>

	<div>
		<button
			type="button"
			id='Register'
			class='purple-button'
			disabled={isPushedRegister || IsDataReady}
			onclick={register}

		>
			<span class='purple-button-span'>Отправить файлы на регистрацию</span>
		</button>
	</div>
</section>

