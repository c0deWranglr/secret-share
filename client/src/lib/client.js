import axios from "axios";

axios.defaults.baseURL = window['config'].baseUrl;

export async function storeSecret(data, ttl = null, attempts = null) {
    try {
        let res = await axios.post("/api/save", { value: data }, { params: { ttl: ttl, attempts: attempts } });
        return res.data.key;
    } catch (err) {
        console.log(err);
    }
}

export async function getSecret(key) {
    try {
        let res = await axios.get("/api/load/"+key);
        return res.data.data;
    } catch(err) {
        console.log(err);
    }
}