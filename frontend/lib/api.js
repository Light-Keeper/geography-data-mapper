
const BASE_URL = process.env.NEXT_PUBLIC_API_BASE_URL || '';
export function apiRequest(method, path, data) {
    var url = BASE_URL + path;
    var options = {
        method: method,
        headers: {
            'Accept': 'application/json',
        }
    };
    if (data) {
        options.body = JSON.stringify(data);
        options.headers['Content-Type'] = 'application/json';
    }
    return fetch(url, options).then(function(response) {
        if (response.status >= 400) {
            throw new Error("Bad response from server");
        }
        return response.json();
    });
}