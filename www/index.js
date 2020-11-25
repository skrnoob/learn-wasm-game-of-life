import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg"

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();
const cvs = document.getElementById("game-of-life-canvas");

cvs.height = ( CELL_SIZE + 1 ) * height + 1;
cvs.width = ( CELL_SIZE + 1 ) * width + 1;

const ctx = cvs.getContext('2d');

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i<= width; i++) {
        ctx.moveTo(i*(CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i*(CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    for( let j = 0; j<= height; j++ ) {
        ctx.moveTo(0, j*(CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j*(CELL_SIZE + 1) + 1);
    }

    ctx.stroke();

}

const getIndex = (row, col) => {
    return row * width + col;
}

const bitIsSet = (n, arr) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) === mask;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);
    ctx.beginPath();

    for(let row = 0; row<height; row++) {
        for (let col = 0;col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = bitIsSet(idx, cells)
                ? ALIVE_COLOR
                : DEAD_COLOR;
            
            ctx.fillRect(
                col * ( CELL_SIZE + 1 ) + 1,
                row * ( CELL_SIZE + 1 ) + 1,
                CELL_SIZE,
                CELL_SIZE
            )
        }
    }

    ctx.stroke()
}

let animationId = null;
let tickTime = 1;
let underCtrl = false;

const playPauseButton = document.getElementById("play-pause");
const tickInput = document.getElementById("volume");
const labelInput = document.getElementById("label_volume");
const resetButton = document.getElementById("reset");
const clearButton = document.getElementById("clear");

tickInput.setAttribute("value",tickTime.toString());
labelInput.textContent = `${tickTime.toString()} ticks per frame`;

const play = () =>{
    playPauseButton.textContent = "⏸";
    renderLoop()
}

const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
}

const renderLoop = () => {
    drawGrid();
    drawCells();
    for(let i = 1;i<=tickTime;i++){
        universe.tick()
    }

    animationId = requestAnimationFrame(renderLoop);
    
}

const isPaused = ()=> {
    return animationId == null;
}

playPauseButton.addEventListener("click",e => {
    if(isPaused()) {
        play();
    } else {
        pause();
    }
})

cvs.addEventListener("click", event => {
    const boundingRect = cvs.getBoundingClientRect();
  
    const scaleX = cvs.width / boundingRect.width;
    const scaleY = cvs.height / boundingRect.height;
  
    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;
  
    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);
    
    if(event.ctrlKey) {
        universe.set_glider(row, col)
    } else {
        universe.toggle_cell(row, col);
    }

    drawGrid();
    drawCells();
});
  
tickInput.addEventListener("change",e => {
    let value;

    try {
        value = e.target.value;
    } catch (error) {
        return
    }

    if(isNaN(value) || value == null || value == undefined) {
        return
    };

    tickTime = parseInt(value);

    labelInput.textContent = `${value.toString()} ticks per frame`;

})

resetButton.addEventListener("click", e => {
    universe.reset()
})

clearButton.addEventListener("click", e => {
    universe.clear()
})

play();
