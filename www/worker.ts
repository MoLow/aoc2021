import * as wasm from './pkg';

wasm.init();
postMessage({ type: 'ready' });

addEventListener('message', (event) => {
    const { day, type } = event.data || {};
    if (type !== 'runDay') {
        return;
    }
    try {
        postMessage({ type, day, result: wasm.run(day) })
    } catch (e) {
        postMessage({ type, day, error: e.stack });
    }
});