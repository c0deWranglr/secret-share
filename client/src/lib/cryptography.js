import CryptoJS from "crypto-js";

export function encrypt(token, data) {
    let key = hash(token, 1000);
    return CryptoJS.AES.encrypt(data, key).toString();
}

export function decrypt(token, data) {
    let key = hash(token, 1000);
    let bytes = CryptoJS.AES.decrypt(data, key);
    return bytes.toString(CryptoJS.enc.Utf8);
}

function hash(value, times) {
    for (var i = 0; i < times; i++) {
        value = CryptoJS.SHA256(value);
    }
    return value.toString();
}