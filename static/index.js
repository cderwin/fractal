window.onload = function(){

var map = L.map('map').setView([0.00, 0.00], 0);

L.tileLayer(window.location + 'render/{z}/{y}/{x}?', {
    'maxZoom': 1000
}).addTo(map);

};
