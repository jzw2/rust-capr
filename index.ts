import init, {
  transduce_context,
  transduce_context_invert,
  greet,
} from "./pkg/rust_capr";

function transduce() {
  (document.getElementById("output") as HTMLParagraphElement).innerText =
    "Loading...";

  const left = (document.getElementById("left") as HTMLInputElement).value;
  const right = (document.getElementById("right") as HTMLInputElement).value;
  const to = (document.getElementById("to") as HTMLInputElement).value;
  const from = (document.getElementById("from") as HTMLInputElement).value;
  const input = (document.getElementById("input") as HTMLInputElement).value;
  const backward = (document.getElementById("backward") as HTMLInputElement)
    .value;

  let result = transduce_context(left, right, from, to, input);
  let backward_result = transduce_context_invert(
    left,
    right,
    from,
    to,
    backward,
  );

  console.log(`${result}, ${result.length}`);
  console.log(`${backward_result}, ${backward.length}`);
  (document.getElementById("output") as HTMLParagraphElement).innerText = result
    .join("")
    .split(" ")
    .filter((s) => s != " ")
    .join("");
  (
    document.getElementById("backwards-output") as HTMLParagraphElement
  ).innerText =
    "num outputs outputs" +
    backward_result.length +
    "\n" +
    backward_result.join("\n");
}

async function run() {
  await init();

  console.log("Event listeners!");

  // List of all input field ids that need an event listener
  const inputIds = ["left", "right", "from", "to", "input", "backward"];

  inputIds.forEach((id) => {
    (document.getElementById(id) as HTMLInputElement).addEventListener(
      "input",
      () => transduce(),
    );
  });
}

run();
