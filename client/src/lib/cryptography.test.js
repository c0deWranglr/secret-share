import { encrypt, decrypt } from './cryptography'
import assert from "assert";

test('Can Encrypt and Decrypt Password', () => {
    let data = encrypt("12345", "my password");
    console.log("Encrypted data: "+data);
    let password = decrypt("12345", data);
    console.log("Decrypted password: "+password);
    assert.equal(password, "my password");
});