import init, { transduce_context, greet } from "./pkg/rust_capr";

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
