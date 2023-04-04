
//const BASE_URL = 'http://localhost:8000';
const BASE_URL = '';

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
        return response.json();
    });
}