const baseUrl = "{{base_url}}";
const siteName = "{{site_name}}";

window['config'] = {
    baseUrl: baseUrl.startsWith("http") ? baseUrl : "http://127.0.0.1:8080",
    siteName: siteName.startsWith("{{") ? "Secret Share" : siteName
};
