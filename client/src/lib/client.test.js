/**
 * @jest-environment node
 */

import { storeSecret, getSecret } from "./client";
import assert from 'assert';

it('Can store and get secret', async () => {
    await storeSecret("12345").then(async key => {
        console.log("Key: "+key);
        await getSecret(key).then(data => {
            console.log("Data: "+data);
            assert.equal(data, "12345");
        });
    });
});