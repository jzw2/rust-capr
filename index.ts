import init, {
  create_law,
  SoundLaw,
  transduce_context,
  transduce_context_invert,
} from "./pkg/rust_capr";

var currentInput: SoundLaw | null = null;

function setLaw() {
  const left = (document.getElementById("left") as HTMLInputElement).value;
  const right = (document.getElementById("right") as HTMLInputElement).value;
  const to = (document.getElementById("to") as HTMLInputElement).value;
  const from = (document.getElementById("from") as HTMLInputElement).value;
  currentInput = create_law(left, right, from, to);
}

function transduce() {
  (document.getElementById("output") as HTMLParagraphElement).innerText =
    "Loading...";

  const input = (document.getElementById("input") as HTMLInputElement).value;
  const backward = (document.getElementById("backward") as HTMLInputElement)
    .value;

  if (currentInput != null) {
    let result = currentInput.transduce_text(input);
    let backward_result = currentInput.transduce_text_backwards(backward);
    console.log(`${result}, ${result.length}`);
    console.log(`${backward_result}, ${backward.length}`);
    (document.getElementById("output") as HTMLParagraphElement).innerText =
      result
        .join("")
        .split(" ")
        .filter((s) => s != " ")
        .join("");
    (
      document.getElementById("backwards-output") as HTMLParagraphElement
    ).innerText = "\n" + backward_result.join("\n");
  }
}

async function run() {
  await init();

  console.log("Event listeners!");

  // List of all input field ids that need an event listener
  const inputIds = ["input", "backward"];

  inputIds.forEach((id) => {
    (document.getElementById(id) as HTMLInputElement).addEventListener(
      "input",
      () => transduce(),
    );
  });

  (document.getElementById("create-law") as HTMLButtonElement).addEventListener(
    "onClick",
    () => setLaw(),
  );
}

run();
