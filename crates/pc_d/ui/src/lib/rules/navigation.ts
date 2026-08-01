import { goto } from '$app/navigation';

export async function goTo(path: string) {
    // eslint-disable-next-line svelte/no-navigation-without-resolve
    return await goto(path);
}