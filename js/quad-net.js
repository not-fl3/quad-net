function on_init() {
}

register_plugin = function (importObject) {
    importObject.env.ws_connect = ws_connect;
    importObject.env.ws_is_connected = ws_is_connected;
    importObject.env.ws_send = ws_send;
    importObject.env.ws_try_recv = ws_try_recv;
}

miniquad_add_plugin({ register_plugin, on_init });

var socket;
var connected = 0;
var received_buffer = [];

function ws_is_connected() {
    return connected;
}

function ws_connect(addr) {
    socket = new WebSocket(consume_js_object(addr));
    socket.binaryType = 'arraybuffer';
    socket.onopen = function() {
        connected = 1;
    };

    socket.onmessage = function(msg) {
        var buffer = new Uint8Array(msg.data);
        received_buffer.push(buffer);
    }
};

function ws_send(data) {
    var array = consume_js_object(data);
    socket.send(array.buffer);
};

function ws_try_recv() {
    if (received_buffer.length != 0) {
        return js_object(received_buffer.shift())
    }
    return -1;
}
