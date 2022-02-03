const baseUrl = "{{base_url}}";
const siteName = "{{site_name}}";
const hCaptchaKey = "{{hCaptcha_key}}";

window['config'] = {
    baseUrl: baseUrl.startsWith("http") ? baseUrl : "http://127.0.0.1:8080",
    siteName: siteName.startsWith("{{") ? "Secret Share" : siteName,
    hCaptchaKey: hCaptchaKey.startsWith("{{") ? "80bc2388-3e05-42ae-9cb2-ef9501d6485b" : hCaptchaKey
};