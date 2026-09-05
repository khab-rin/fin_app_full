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