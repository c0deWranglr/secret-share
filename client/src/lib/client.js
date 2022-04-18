import axios from "axios";

axios.defaults.baseURL = window['config'].baseUrl;

export async function storeSecret(captcha, data, ttl = null, attempts = null) {
    try {
        let res = await axios.post("/api/save", { value: data }, { params: { ttl_min: ttl, attempts: attempts }, headers: { 'X-CAPTCHA-TOKEN': captcha } });
        return res.data.key;
    } catch (err) {
        console.log(err);
    }
}

export async function getSecret(captcha, key) {
    try {
        let res = await axios.get("/api/load/"+key, { headers: { 'X-CAPTCHA-TOKEN': captcha } });
        return res.data.data;
    } catch(err) {
        console.log(err);
    }
}