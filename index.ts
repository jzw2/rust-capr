import init, {
  create_law,
  SoundLaw,
  transduce_context,
  transduce_context_invert,
  SoundLawComposition,
} from "./pkg/rust_capr";

let currentInput: SoundLaw | undefined;

const setLaw = () => {
  (document.getElementById("output") as HTMLParagraphElement).innerText =
    "Compiling FST";
  (
    document.getElementById("backwards-output") as HTMLParagraphElement
  ).innerText = "Compiling FST";
  const left = (document.getElementById("left") as HTMLInputElement).value;
  const right = (document.getElementById("right") as HTMLInputElement).value;
  const to = (document.getElementById("to") as HTMLInputElement).value;
  const from = (document.getElementById("from") as HTMLInputElement).value;
  let currentInput = create_law(left, right, from, to);
  currentLaws.push(currentInput);
  fst?.add_law(currentInput);
  transduce();
};

let currentLaws: SoundLaw[] = [];
let fst: SoundLawComposition | null = null;

const updateRulesList = () => {
  const rulesList = document.getElementById(
    "rulesList",
  ) as HTMLParagraphElement;
  rulesList.innerHTML = "";
  currentLaws.forEach((x) => {
    let s = `Rule: ${x.get_from()} → ${x.get_to()} / ${x.get_left_context()} _ ${x.get_right_context()}`;
    const listItem = document.createElement("li");
    listItem.textContent = s;
    rulesList.appendChild(listItem);
  });
};

const transduce = () => {
  (document.getElementById("output") as HTMLParagraphElement).innerText =
    "Loading...";
  (
    document.getElementById("backwards-output") as HTMLParagraphElement
  ).innerText = "Loading...";

  const input = (document.getElementById("input") as HTMLInputElement).value;
  const backward = (document.getElementById("backward") as HTMLInputElement)
    .value;

  if (fst != null) {
    let result = fst.transduce_text(input);
    let backward_result = fst.transduce_text_invert(backward);
    console.log(`${result}, ${result.length}`);
    //console.log(`${backward_result}, ${backward.length}`);
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
};

async function run() {
  await init();
  fst = SoundLawComposition.new();

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
    "click",
    () => {
      setLaw();
      updateRulesList();
    },
  );
}

run();
