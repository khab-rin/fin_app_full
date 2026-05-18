import { goto } from '$app/navigation';

export function goTo(path: string) {
    return goto(path);
}