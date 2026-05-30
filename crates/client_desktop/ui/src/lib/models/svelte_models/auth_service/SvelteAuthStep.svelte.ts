import type {AuthStep} from '$lib/models/AuthStep';

class SvelteAuthStep {
    step = $state<AuthStep>({Loading: {}});
}

export const currAuthStep = new SvelteAuthStep();