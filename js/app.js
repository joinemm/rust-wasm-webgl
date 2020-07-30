console.log("running app.js");

/*jshint -W024 */
const rust =
    import ('../pkg');

const canvas = document.getElementById("rustCanvas");
const gl = canvas.getContext("webgl", { antialias: true });

rust.then(module => {
    if (!gl) {
        alert("Failed to initialize WebGL");
        return;
    } else {
        console.log("succesfully initialized WebGL");
    }

    gl.enable(gl.BLEND);
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

    const FPS = 1000.0 / 60.0; // ms / frames
    const rustClient = new module.Client();
    const initialTime = Date.now();
    var lastDrawTime = -1;

    function render() {
        window.requestAnimationFrame(render);
        const currTime = Date.now();

        if (currTime >= lastDrawTime + FPS) {
            lastDrawTime = currTime;

            if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
                canvas.height = window.innerHeight;
                canvas.clientHeight = window.innerHeight;
                canvas.style.height = window.innerHeight;

                canvas.width = window.innerWidth;
                canvas.clientWidth = window.innerWidth;
                canvas.style.width = window.innerWidth;

                gl.viewport(0, 0, window.innerWidth, window.innerHeight);
            }

            let elapsedTime = currTime - initialTime;
            rustClient.update(elapsedTime, window.innerHeight, window.innerWidth);
            rustClient.render();
        }
    }

    render();
});