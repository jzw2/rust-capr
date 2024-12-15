import init, {
  create_law,
  SoundLaw,
  transduce_context,
  transduce_context_invert,
  SoundLawComposition,
  soundlaw_xsampa_to_ipa,
} from "./pkg/rust_capr";

// import { Drawable } from "./krist_lib/editor";
import Victor from "victor";
// import { RuleNode } from "./krist_lib/rule-node";

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

const updateRulesList = (state: State) => {
  const rulesList = document.getElementById(
    "rulesList",
  ) as HTMLParagraphElement;
  let currentLaws = state.laws;
  rulesList.innerHTML = "";
  currentLaws.forEach((x, index) => {
    let s = `Rule: ${x.get_from()} → ${x.get_to()} / ${x.get_left_context()} _ ${x.get_right_context()}`;
    const listItem = document.createElement("li");
    listItem.textContent = s;
    listItem.draggable = true;

    const deleteButton = document.createElement("button");
    deleteButton.textContent = "Delete";
    deleteButton.classList.add("delete-button");
    deleteButton.addEventListener("click", () => {
      currentLaws.splice(index, 1);
      state.composition.rm_law(index);
      transduce(state.composition);
      updateRulesList(state);

      serializeOps(
        state.laws.map((law) => ({
          left: law.get_left_context(),
          right: law.get_right_context(),
          from: law.get_from(),
          to: law.get_to(),
        })),
      );
    });

    listItem.appendChild(deleteButton);
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

  let result = fst.transduce_text(input);
  let backward_result = fst.transduce_text_invert(backward);
  console.log(`${result}, ${result.length}`);
  //console.log(`${backward_result}, ${backward.length}`);
  (document.getElementById("output") as HTMLParagraphElement).innerText = result
    .map((x) => x + "\t IPA: " + soundlaw_xsampa_to_ipa(x))
    .join("\n");
  (
    document.getElementById("backwards-output") as HTMLParagraphElement
  ).innerText =
    "\n" +
    backward_result
      .map((x) => x + "\t IPA: " + soundlaw_xsampa_to_ipa(x))
      .join("\n");
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

type State = { laws: SoundLaw[]; composition: SoundLawComposition };

const mouseListeners = (state: State) => {
  const list = document.getElementById("rulesList") as HTMLUListElement;

  list.addEventListener("dragstart", (e: DragEvent) => {
    const target = e.target as HTMLElement;
    if (target.tagName === "LI") {
      e.dataTransfer?.setData(
        "text/plain",
        Array.from(list.children).indexOf(target).toString(),
      );
      target.classList.add("dragging");
    }
  });

  list.addEventListener("dragover", (e: DragEvent) => {
    e.preventDefault();
    const target = e.target as HTMLElement;
    if (target && target.tagName === "LI") {
      e.dataTransfer!.dropEffect = "move";
      const draggingItem = list.querySelector(".dragging");
      if (draggingItem && target !== draggingItem) {
        const rect = target.getBoundingClientRect();
        const offset = e.clientY - rect.top;
        if (offset > rect.height / 2) {
          list.insertBefore(draggingItem, target.nextSibling);
        } else {
          list.insertBefore(draggingItem, target);
        }
      }
    }
  });
  list.addEventListener("drop", (e: DragEvent) => {
    e.preventDefault();
    const target = e.target as HTMLElement;
    if (target && target.tagName === "LI") {
      const oldIndex = parseInt(e.dataTransfer!.getData("text/plain"), 10);
      const newIndex = Array.from(list.children).indexOf(target);

      if (oldIndex !== -1 && newIndex !== -1 && oldIndex !== newIndex) {
        const [movedLaw] = state.laws.splice(oldIndex, 1);
        state.laws.splice(newIndex, 0, movedLaw);
        state.composition = SoundLawComposition.new();
        state.laws.forEach((law) => state.composition.add_law(law));
        updateRulesList(state);
        serializeOps(
          state.laws.map((law) => ({
            left: law.get_left_context(),
            right: law.get_right_context(),
            from: law.get_from(),
            to: law.get_to(),
          })),
        );
        transduce(state.composition);
      }
    }
  });
  list.addEventListener("dragend", () => {
    const draggingItem = list.querySelector(".dragging");
    if (draggingItem) {
      draggingItem.classList.remove("dragging");
    }
  });
};
async function run() {
  await init();

  console.log("Event listeners!");

  // List of all input field ids that need an event listener

  const inputIds = ["input", "backward"];

  const [currentLaws, fst, operations] = deserializeOps();
  let state: State = { laws: currentLaws, composition: fst };
  updateRulesList(state);

  inputIds.forEach((id) => {
    (document.getElementById(id) as HTMLInputElement).addEventListener(
      "input",
      () => transduce(fst),
    );
  });

  (document.getElementById("create-law") as HTMLButtonElement).addEventListener(
    "click",
    () => {
      console.log("pressed the button");
      setLaw(currentLaws, fst, operations);
      updateRulesList(state);
      serializeOps(operations);
    },
  );

  // const rule = new RuleNode(
  //   document.getElementById("mainCanvas") as HTMLCanvasElement,
  //   new Victor(10, 10),
  // );
  // rule.render();

  mouseListeners(state);
}

run();
