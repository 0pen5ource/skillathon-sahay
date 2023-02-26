import { writable } from 'svelte/store';
import {subscribe} from "svelte/internal";

const messageStore = writable('');

let socket = null;

const sendMessage = (message) => {
    if (socket.readyState <= 1) {
        socket.send(message);
    }
}

const subscribeWs = (callback) => {
    socket = new WebSocket('wss://sahay.xiv.in/ws');

// Connection opened
    socket.addEventListener('open', function (event) {
        console.log("It's open");
    });

// Listen for messages
    socket.addEventListener('message', function (event) {
        messageStore.set(event.data);
    });
    messageStore.subscribe(callback);
}


export default {
    subscribe: subscribeWs,
    sendMessage
}

