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
	e.stopPropagation();
	const button = e.currentTarget as HTMLElement;
	const dialog = document.getElementById(dialogID) as HTMLDialogElement;
	
	if (!dialog || !button) {
		return;
	}

	if (dialog.open) {
		dialog.close();
		return;
	}
	
	const rect = button.getBoundingClientRect();

	dialog.style.position = 'fixed';
	dialog.style.margin = '0';

	dialog.style.top = `${rect.top}px`;
	dialog.style.left = `${rect.right + 8}px`; 

	dialog.show();

	const closeOnOutsideClick = (event: MouseEvent) => {
		const target = event.target as Node;
		
		if (!dialog.contains(target) && !button.contains(target)) {
			dialog.close();
			document.removeEventListener('click', closeOnOutsideClick);
		}
	};

	setTimeout(() => {
		document.addEventListener('click', closeOnOutsideClick);
	}, 1);
}