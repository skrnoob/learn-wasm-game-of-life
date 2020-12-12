import * as WebGLUtils from 'webgl-utils.js';

const vsSource = `
  attribute vec2 a_position;
  attribute vec2 a_rec_position;
  attribute float a_point_size;
  uniform vec2 u_resolution;

  void main(){
    vec2 zeroToOne = a_position / u_resolution;
    vec2 zeroToTwo = zeroToOne * 2.0;
    vec2 clipSpace = zeroToTwo - 1.0;

    gl_Position = vec4(clipSpace * vec2(1,-1) , 0, 1);
    gl_PointSize = a_point_size;
  }`

const fsSource = `
  precision mediump float;
  uniform vec4 u_color;

  void main() {
    gl_FragColor = u_color;
  }
  `

let gl_vertexShader = null;
let gl_fragmentShader = null;
let gl_program = null;
let hasResized = false;
let hasSetViewPort = false;
let gl_gl = null;

function createShader(gl, type, source) {
    var shader = gl.createShader(type);
    gl.shaderSource(shader, source);
    gl.compileShader(shader);

    var success = gl.getShaderParameter(shader, gl.COMPILE_STATUS);
    if (success) {
        return shader
    }

    gl.deleteShader(shader);
}

function createProgram(gl, vertexShader, fragmentShader) {
    var program = gl.createProgram();
    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    gl.linkProgram(program);

    var success = gl.getProgramParameter(program, gl.LINK_STATUS);
    if (success) {
        return program;
    }

    console.log(gl.getProgramInfoLog(program));
    gl.deleteProgram(program);
}

function draw(row = 64, col = 64, cellSize = 10, cellsState = [], color = [0, 0, 0, 1]) {

    if (!gl_gl) {
        const canvas = document.getElementById("game-of-life-canvas");
        gl_gl = canvas.getContext("webgl");
    }
    const gl = gl_gl;
    // const canvas = document.getElementById("game-of-life-canvas");
    // const gl = canvas.getContext("webgl");
    if (!gl) {
        alert("无法初始化WebGL，你的浏览器、操作系统或硬件等可能不支持WebGL。");
        return;
    }
    var vertexShader;
    if (!gl_vertexShader) {
        vertexShader = createShader(gl, gl.VERTEX_SHADER, vsSource);
        gl_vertexShader = vertexShader;
    } else {
        vertexShader = gl_vertexShader;
    }

    var fragmentShader;
    if (!gl_fragmentShader) {
        fragmentShader = createShader(gl, gl.FRAGMENT_SHADER, fsSource);
        gl_fragmentShader = fragmentShader;
    } else {
        fragmentShader = gl_fragmentShader;
    }

    var program;
    if (!gl_program) {
        program = createProgram(gl, vertexShader, fragmentShader);
        gl_program = program;
    } else {
        program = gl_program;
    }


    var positionAttributeLocation = gl.getAttribLocation(program, "a_position");
    var resolutionUniformLocation = gl.getUniformLocation(program, "u_resolution");
    var colorUniformLocation = gl.getUniformLocation(program, "u_color");

    if (!hasResized) {
        WebGLUtils.resizeCanvasToDisplaySize(gl.canvas);
        hasResized = true;
    }

    if (!hasSetViewPort) {
        gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);
        hasSetViewPort = true
    }

    gl.clearColor(1, 1, 1, 1);
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

    gl.useProgram(program);

    gl.enableVertexAttribArray(positionAttributeLocation);

    // draw grid
    var positionBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

    var size = 2;
    var type = gl.FLOAT;
    var normalized = false;
    var stride = 0;
    var offset = 0;

    gl.vertexAttribPointer(positionAttributeLocation, size, type, normalized, stride, offset);

    gl.uniform2f(resolutionUniformLocation, gl.canvas.width, gl.canvas.height);
    gl.uniform4f(colorUniformLocation, ...color);


    var positions = [];

    for (let i = 0; i <= row; i++) {
        positions.push(i * (cellSize + 1) + 1, 0, i * (cellSize + 1) + 1, row * (cellSize + 1) + 1);
    }

    for (let i = 0; i <= col; i++) {
        positions.push(0, i * (cellSize + 1) + 1, col * (cellSize + 1) + 1, i * (cellSize + 1) + 1);
    }

    let lineLength = positions.length;

    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);

    var primitiveType = gl.LINES;
    var offset = 0;
    var count = lineLength / 2;
    gl.drawArrays(primitiveType, offset, count);

    // draw points

    var pointPositionBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, pointPositionBuffer);

    // let pointPositions = cellsState.reduce((total, state, idx) => {
    //     if (state) {
    //         let x = (idx % row) * (cellSize + 1) + cellSize / 2 + 1;
    //         let y = Math.floor(idx / row) * (cellSize + 1) + cellSize / 2 + 1;
    //         return total = [...total, x, y];
    //     }
    //     return total
    // }, [])

    // gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(pointPositions), gl.STATIC_DRAW);

    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(cellsState), gl.STATIC_DRAW);
    gl.vertexAttribPointer(positions, size, type, normalized, stride, offset);

    var pointSizesBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, pointSizesBuffer);
    var positionSizeAttributeLocation = gl.getAttribLocation(program, "a_point_size");
    gl.enableVertexAttribArray(positionSizeAttributeLocation);
    let pointSizes = new Array(cellsState.length / 2).fill(cellSize);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(pointSizes), gl.STATIC_DRAW);
    gl.vertexAttribPointer(positionSizeAttributeLocation, 1, type, normalized, stride, offset);


    primitiveType = gl.POINTS;
    offset = 0;
    count = cellsState.length / 2;
    gl.drawArrays(primitiveType, offset, count);

    // draw cells
    // var rectPositionBuffer = gl.createBuffer();
    // gl.bindBuffer(gl.ARRAY_BUFFER, rectPositionBuffer);

    // let rectPositions = [];

    // rectPositions = cellsState.reduce((total, state, idx) => {
    //     if (state) {
    //         let west = (idx % row) * (cellSize + 1);
    //         let north = Math.floor(idx / row) * (cellSize + 1);
    //         let east = (idx % row + 1) * (cellSize + 1);
    //         let south = Math.floor(idx / row + 1) * (cellSize + 1);
    //         let wn = [west, north];
    //         let en = [east, north];
    //         let es = [east, south];
    //         let ws = [west, south];
    //         return total = [...total, ...wn, ...en, ...ws, ...en, ...ws, ...es]
    //     }
    //     return total
    // }, [])

    // gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(rectPositions), gl.STATIC_DRAW);

    // gl.bindBuffer(gl.ARRAY_BUFFER, rectPositionBuffer);

    // gl.vertexAttribPointer(positions, size, type, normalized, stride, offset);

    // primitiveType = gl.TRIANGLES;
    // offset = 0;
    // count = rectPositions.length / 2;
    // gl.drawArrays(primitiveType, offset, count)
}

export { draw }