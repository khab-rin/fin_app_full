import type {AuthStep} from '$lib/models/AuthStep';

class SvelteAuthStep {
    step = $state<AuthStep>({Init:{}});
    isLoading = $state(true);

    setStep(step: AuthStep) {
        this.step = step
    }
}

export const currAuthStep = new SvelteAuthStep();