import './index.css';
import * as wasm from './pkg';

const root = document.createElement('div');

function navigate(path: string) {
    history.pushState({}, '', path);
    render();
}

function render() {
    const day = Number(location.pathname.match(/\d+/)?.[0])
    root.innerHTML = '';

    const daysContainer = document.createElement('div');
    daysContainer.classList.add('days');
    Array(25).fill(0).forEach((_, day) => {
        day++;
        const a = document.createElement('a');
        a.innerText = `Day ${day}`;
        a.href = `/day${day}`;
        a.addEventListener('click', (e) => {
            navigate(`/day${day}`);
            e.preventDefault();
        });
        daysContainer.appendChild(a);
    });

    const result = wasm.run(day);
    const pre = document.createElement('pre');
    pre.innerText = `${result[2]}Part1: ${result[0]}\nPart2: ${result[1]}`;

    root.appendChild(daysContainer);
    root.appendChild(pre);
};

(function () {
    wasm.init();
    document.body.appendChild(root);
    window.addEventListener('popstate', render);
    render();
}());