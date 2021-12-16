import './index.css';

const worker = new Worker('worker.js');
const pre = document.createElement('pre');

const state: { day: number, result: any, error?: string } = { day: 0, result: null };

function navigate(path: string) {
    history.pushState({}, '', path);
    runCurrentDay();
}

worker.addEventListener('message', (event) => {
    const { type, day, result, error } = event.data;
    if (type === 'ready') { 
        state.day = 0;
        return runCurrentDay();
    }
    if (type === 'runDay' && day === state.day) {
        state.result = result;
        state.error = error;
    }
    render();
});

function runCurrentDay() {
    let prevDay = state.day;
    state.day = Number(location.pathname.match(/\d+/)?.[0]);
    if (prevDay !== state.day && state.day) {
        state.result = null;
        state.error = null;
        render();
        worker.postMessage({ type: 'runDay', day: state.day });
    }
}

function render() {
    pre.classList.remove('error');

    if (state.result === null) {
        pre.innerHTML = `Calculating day ${state.day}...`;
    }

    if (state.error) {
        pre.innerHTML = state.error;
        pre.classList.add('error');
    }

    if (state.result) {
        const result = state.result;
        pre.innerText = `${result[2]}Part1: ${result[0]}\nPart2: ${result[1]}`;
    }

};

(function () {
    const baseUrl = location.pathname.split('/').slice(0, -1).join('/');
    const root = document.createElement('div');
    const daysContainer = document.createElement('div');
    daysContainer.classList.add('days');
    Array(25).fill(0).forEach((_, day) => {
        day++;
        const a = document.createElement('a');
        a.innerText = `Day ${day}`;
        a.href = `${baseUrl}/day${day}`;
        a.addEventListener('click', (e) => {
            navigate(`${baseUrl}/day${day}`);
            e.preventDefault();
        });
        daysContainer.appendChild(a);
    });


    root.appendChild(daysContainer);
    root.appendChild(pre);
    document.body.appendChild(root);
    window.addEventListener('popstate', runCurrentDay);
    runCurrentDay();
}());
