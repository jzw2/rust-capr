import init, { transduce_context, greet } from "./pkg/rust_capr";
let mousePos = { x: -10, y: -10 };

type Point = [x: number, y: number];

class Drawable {
  coords: Point;

  constructor() {}

  move(coords: Point) {
    this.coords = coords;
  }

  draw(ctx: CanvasRenderingContext2D) {}
}

class Rule extends Drawable {
  width: number;
  height: number;

  constructor(width: number, height: number) {
    super();
    this.width = width;
    this.height = height;
  }

  draw(ctx: CanvasRenderingContext2D) {
    ctx.strokeStyle = "1px solid red";
    ctx.rect(this.coords[0], this.coords[1], this.width, this.height);
    ctx.stroke();
  }
}

document.addEventListener("mousemove", (event) => {
  mousePos = { x: event.clientX, y: event.clientY };
});

class Graph extends Drawable {
  nodes: Array<Rule>;

  draw(ctx: CanvasRenderingContext2D) {
    this.nodes.forEach((n) => n.draw(ctx));
  }
}

const myGraph = new Graph();
myGraph.nodes = [new Rule(10, 10), new Rule(20, 30)];
myGraph.nodes[0].move([10, 10]);
myGraph.nodes[1].move([20, 30]);

function update() {
  const canvas = document.getElementById("mainCanvas");
  if (canvas) {
    const ctx = (canvas as HTMLCanvasElement).getContext("2d");
    if (ctx) {
      ctx.clearRect(0, 0, 1000, 1000);
      myGraph.draw(ctx);
      // ctx.beginPath();
      // ctx.moveTo(0, 0);
      // ctx.lineTo(mousePos.x, mousePos.y);
      // ctx.closePath();
      // ctx.strokeStyle = "1px solid black";
      // ctx.stroke();
    }
  }
}

setInterval(() => update(), 1000 / 24);
// Use ES module import syntax to import functionality from the module
// that we have compiled.
//
// Note that the `default` import is an initialization function which
// will "boot" the module and make it ready to use. Currently browsers
// don't support natively imported WebAssembly as an ES module, but
// eventually the manual initialization won't be required!

function transduce() {
  (document.getElementById("output") as HTMLParagraphElement).innerText =
    "Loading...";
  const left = (document.getElementById("left") as HTMLInputElement).value;
  const right = (document.getElementById("right") as HTMLInputElement).value;
  const to = (document.getElementById("to") as HTMLInputElement).value;
  const from = (document.getElementById("from") as HTMLInputElement).value;
  const input = (document.getElementById("input") as HTMLInputElement).value;
  // const output = document.getElementById("output");

  // greet();
  const result = transduce_context(left, right, from, to, input);
  console.log(`${result}, ${result.length}`);
  (document.getElementById("output") as HTMLParagraphElement).innerText = result
    .join("")
    .split(" ")
    .filter((s) => s != " ")
    .join("");
}

async function run() {
  // First up we need to actually load the Wasm file, so we use the
  // default export to inform it where the Wasm file is located on the
  // server, and then we wait on the returned promise to wait for the
  // Wasm to be loaded.
  //
  // It may look like this: `await init('./pkg/without_a_bundler_bg.wasm');`,
  // but there is also a handy default inside `init` function, which uses
  // `import.meta` to locate the Wasm file relatively to js file.
  //
  // Note that instead of a string you can also pass in any of the
  // following things:
  //
  // * `WebAssembly.Module`
  //
  // * `ArrayBuffer`
  //
  // * `Response`
  //
  // * `Promise` which returns any of the above, e.g. `fetch("./path/to/wasm")`
  //
  // This gives you complete control over how the module is loaded
  // and compiled.
  //
  // Also note that the promise, when resolved, yields the Wasm module's
  // exports which is the same as importing the `*_bg` module in other
  // modes
  await init();

  console.log("Event listeners!");
  // And afterwards we can use all the functionality defined in wasm.
  (document.getElementById("left") as HTMLInputElement).addEventListener(
    "input",
    () => transduce(),
  );
  (document.getElementById("right") as HTMLInputElement).addEventListener(
    "input",
    () => transduce(),
  );
  (document.getElementById("from") as HTMLInputElement).addEventListener(
    "input",
    () => transduce(),
  );
  (document.getElementById("to") as HTMLInputElement).addEventListener(
    "input",
    () => transduce(),
  );
  (document.getElementById("input") as HTMLInputElement).addEventListener(
    "input",
    () => transduce(),
  );

  // const result = add(1, 2);
  // console.log(`1 + 2 = ${result}`);
  // if (result !== 3)
  //     throw new Error("wasm addition doesn't work!");
}

run();
