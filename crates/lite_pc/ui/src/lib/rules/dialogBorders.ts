export function dialogBackdrop(e: MouseEvent): void {
	const dialog = e.currentTarget as HTMLDialogElement | null;
	if (!dialog) return;
	const rect = dialog.getBoundingClientRect();

	const isClickInside = 
		e.clientX >= rect.left &&
        e.clientX <= rect.right &&
        e.clientY >= rect.top &&
        e.clientY <= rect.bottom;
	
	if (!isClickInside) {
		dialog.close();
	}
}

export function openDialogRight(e: MouseEvent, dialogID: string) {
	e.stopPropagation;
	const button = e.currentTarget as HTMLBRElement;
	const dialog = document.getElementById(dialogID) as HTMLDialogElement;
	if(!dialog || !button) {
		return;
	}
	const rect = button.getBoundingClientRect();

	dialog.style.position = 'absolute'
	
}