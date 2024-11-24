import init, {
  create_law,
  SoundLaw,
  transduce_context,
  transduce_context_invert,
  SoundLawComposition,
} from "./pkg/rust_capr";

import { Drawable } from "./krist_lib/editor";
import Victor from "victor";
import { RuleNode } from "./krist_lib/rule-node";

type Operation = {
  left: string;
  right: string;
  to: string;
  from: string;
};

const setLaw = (
  currentLaws: SoundLaw[],
  fst: SoundLawComposition,
  operations: Operation[],
) => {
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
  operations.push({ left, right, from, to });
  currentLaws.push(currentInput);
  fst.add_law(currentInput);

  transduce(fst);
};

const updateRulesList = (currentLaws: SoundLaw[]) => {
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

const transduce = (fst: SoundLawComposition) => {
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

const serializeOps = (operations: Operation[]) => {
  localStorage.setItem("operations", JSON.stringify(operations));
};

const deserializeOps = (): [SoundLaw[], SoundLawComposition, Operation[]] => {
  let ops: Operation[] = [];
  try {
    const serialized = localStorage.getItem("operations");
    if (serialized) {
      ops = JSON.parse(serialized);
    }
  } catch (err) {
    // nothing
  } finally {
    const currentLaws: SoundLaw[] = [];
    const fst: SoundLawComposition = SoundLawComposition.new();
    for (const { left, right, from, to } of ops) {
      let currentInput = create_law(left, right, from, to);
      currentLaws.push(currentInput);
      fst.add_law(currentInput);
    }
    return [currentLaws, fst, ops];
  }
};

async function run() {
  await init();

  console.log("Event listeners!");

  // List of all input field ids that need an event listener

  const inputIds = ["input", "backward"];

  const [currentLaws, fst, operations] = deserializeOps();
  updateRulesList(currentLaws);

  inputIds.forEach((id) => {
    (document.getElementById(id) as HTMLInputElement).addEventListener(
      "input",
      () => transduce(fst),
    );
  });

  (document.getElementById("create-law") as HTMLButtonElement).addEventListener(
    "click",
    () => {
      setLaw(currentLaws, fst, operations);
      updateRulesList(currentLaws);
      serializeOps(operations);
    },
  );

  const rule = new RuleNode(
    document.getElementById("mainCanvas") as HTMLCanvasElement,
    new Victor(10, 10),
  );
  rule.render();
}

run();
