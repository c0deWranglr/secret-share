const baseUrl = "{{base_url}}";
const siteName = "{{site_name}}";
const hCaptchaKey = "{{hCaptcha_key}}";

window['config'] = {
    baseUrl: baseUrl.startsWith("http") ? baseUrl : "http://127.0.0.1:8080",
    siteName: siteName.startsWith("{{") ? "Secret Share" : siteName,
    hCaptchaKey: hCaptchaKey.startsWith("{{") ? "10000000-ffff-ffff-ffff-000000000001" : hCaptchaKey
};