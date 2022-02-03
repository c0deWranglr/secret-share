
export function resetInputs() {
    Array.from(document.getElementsByTagName("input")).forEach(item => {
        if (item.value !== item.defaultValue) {
            item.value = item.defaultValue;
        }
    });
}