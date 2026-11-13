export function fitText(node: HTMLElement) {
    const parent = node.closest('button');
    if (!parent) return;

    const button: HTMLButtonElement = parent;
    const initialFontSize = parseFloat(getComputedStyle(node).fontSize);

    function adjustFontSize() {
        let fontSize = initialFontSize;
        node.style.fontSize = `${fontSize}px`;

        const overflows = () =>
            button.scrollHeight > button.clientHeight ||
            button.scrollWidth > button.clientWidth;

        while (fontSize > 7 && overflows()) {
            fontSize = Math.max(7, fontSize - 0.5);
            node.style.fontSize = `${fontSize}px`;
        }
    }

    adjustFontSize();

    const resizeObserver = new ResizeObserver(() => {
        requestAnimationFrame(adjustFontSize);
    });

    resizeObserver.observe(button);

    return {
        destroy() {
            resizeObserver.disconnect();
        }
    };
}