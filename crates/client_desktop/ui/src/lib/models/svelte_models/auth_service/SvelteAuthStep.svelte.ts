import type { AuthStep } from '$lib/models/AuthStep';
import type { NickData } from '$lib/models/NickData';

class SvelteAuthStep {
    step = $state<AuthStep>({
        Loading: { 
            text: "Страница загружается, подождите пожалуйста. В случае зависания попробуйте обновить или перезагрузить приложение"
        }
    });

    nick_names = $state<NickData>({ nick_names: [] });

    get currentText(): string {
        const currentStepObj = Object.values(this.step)[0];

        if (
            currentStepObj && 
            typeof currentStepObj === 'object' && 
            'text' in currentStepObj && 
            typeof (currentStepObj as { text: unknown }).text === 'string'
        ) {
            return (currentStepObj as { text: string }).text;
        }
        
        return '';
    }
}

export const currAuthStep = new SvelteAuthStep();