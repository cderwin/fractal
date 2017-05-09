window.onload = function(){

window.map = L.map('map').setView([0.00, 0.00], 0);

L.tileLayer(window.location + 'render/{z}/{y}/{x}?max_iter=1024', {
    'maxZoom': 1000
}).addTo(window.map);

};
