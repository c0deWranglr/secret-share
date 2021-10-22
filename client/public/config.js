const baseUrl = "{{base_url}}";

window['config'] = {
    baseUrl: baseUrl.startsWith("http") ? baseUrl : "http://127.0.0.1:8080"
};
