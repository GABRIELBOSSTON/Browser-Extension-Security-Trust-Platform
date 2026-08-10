// Broad generic APIs
setTimeout(function() {
    var xhr = new XMLHttpRequest();
    xhr.open("GET", "https://any-domain.com/data", true);
    xhr.onload = function() {
        console.log(xhr.responseText);
    };
    xhr.send();
}, 5000);

// Using Function broadly but not maliciously
var func = new Function('a', 'b', 'return a + b');
console.log(func(2, 3));
