<script lang='ts'>
    import { onMount } from "svelte";
    import {writeFile} from "@tauri-apps/plugin-fs";
    import {save} from "@tauri-apps/plugin-dialog";
    
    import {currAuthStep} from "$lib/models/Auth/AuthStep.svelte";

    import type {AuthStep} from "$lib/models/rustModels/AuthStep";
    import { AuthStepType } from "$lib/models/Auth/AuthValues";


    let isSaved = $state({
        doc: false,
        json: false
    });

    let filePath = $state({
        doc: "",
        json: ""
    });

    let fileNames = {
        doc: "",
        json: ""
    };


    let docFileBytesArray: Uint8Array | null = null;
    let jsonFileByteArray: Uint8Array | null = null;




    async function saveDoc(file: Uint8Array | null, fileType: 'doc' | 'json') {
        if (!file) {
            console.error("Ошибка: Попытка сохранить пустой файл (null)");
            return;
        }

        if (isSaved[fileType]) return;
        isSaved[fileType] = true;

        try {
            const path = await save({
                title: "Сохранить файл",
                defaultPath: fileNames[fileType],
                filters: [{name: "Документы", extensions: [fileType]}]

            });
            if (path) {
                await writeFile(path, file);
                filePath[fileType] = path;
            } else {
                isSaved[fileType] = false;
            }
            
        } catch (err) {
            console.error("FILE SAVING FAILED, ERROR = ", err);
            const next_step: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            isSaved[fileType] = false;
            currAuthStep.add(next_step);
        }
    }

    onMount(async() => {
        if (AuthStepType.RegisterStep1Success in currAuthStep.step) {
            const data = currAuthStep.step.RegisterStep1Success;

            fileNames.doc = data.doc_name;
            fileNames.json = data.json_name;

            docFileBytesArray = new Uint8Array(data.doc_file);
            jsonFileByteArray = new Uint8Array(data.json_file);
        } else {    
            const next_step: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
            console.error("System Logic Error, wrong current step");
            currAuthStep.add(next_step);         
        }
    });

</script>

<section class='input-section'>
    <div class='input-wide-button-grid'>
        <label class='input-wide-button-grid-label' for='docFile'>
            Сохранить файл в формате doc
        </label>
        <input
            id='docFile'
            type='text'
            value={filePath.doc}
            class='input-field'
        >

        <button
            type='button'
            class='wide-button'
            onclick={() => saveDoc(docFileBytesArray, 'doc')}
            disabled={isSaved['doc']}
        >
            Сохранить
        </button>
    </div>

    <div class='input-wide-button-grid'>
        <label class='input-wide-button-grid-label' for='jsonFile'>
            Сохранить файл в формате json
        </label>
        <input
            id='jsonFile'
            type='text'
            value={filePath.json}
            class='input-field'
        >

        <button
            type='button'
            class='wide-button'
            onclick={() => saveDoc(jsonFileByteArray, 'json')}
            disabled={isSaved['json']}
        >
            Сохранить
        </button>
    </div>


</section>
