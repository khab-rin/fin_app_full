export function fitText(node: HTMLElement) {
    function adjustFontSize() {
        let fontSize = 14; 
        node.style.fontSize = `${fontSize}px`;

        const parent = node.closest('button');
        if (!parent) return;

        while (
            (parent.scrollHeight > parent.clientHeight || node.scrollHeight > parent.clientHeight) 
            && fontSize > 7
        ) {
            fontSize -= 0.5;
            node.style.fontSize = `${fontSize}px`;
        }
    }

    adjustFontSize();

    const resizeObserver = new ResizeObserver(adjustFontSize);
    resizeObserver.observe(node);

    return {
        destroy() {
            resizeObserver.disconnect();
        }
    };
}