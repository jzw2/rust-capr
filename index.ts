import init, {
  create_law,
  SoundLaw,
  transduce_context,
  transduce_context_invert,
  SoundLawComposition,
  soundlaw_xsampa_to_ipa,
} from "./pkg/rust_capr";

// import { Drawable } from "./krist_lib/editor";
// import Victor from "victor";
// import { RuleNode } from "./krist_lib/rule-node";

type AddSoundLaw = { type: "AddSoundLaw"; law: SoundLawInput };
type UploadFile = { type: "UploadFile"; contents: string };
type StartDrag = { type: "StartDrag"; index: number };
type HoveringOver = { type: "HoveringOver"; index: number };
type ClickDelete = { type: "ClickDelete"; index: number };

type Message =
  | AddSoundLaw
  | UploadFile
  | StartDrag
  | HoveringOver
  | ClickDelete
  | { type: "ChangeInput"; input: string }
  | { type: "ChangeBackwardsInput"; input: string };

//todo: refactor so it isn't so big
type State = {
  soundLawInputs: SoundLawInput[];
  laws: SoundLaw[];
  input: string;
  output: string[];
  reverseInput: string;
  revereseOutput: string[];
  composition: SoundLawComposition;
  fileStrings: string[];
  transducedFileStrings: string[][];
};

type SoundLawInput = {
  left: string;
  right: string;
  to: string;
  from: string;
};

const update = (message: Message, state: State) => {
  console.log("Found message" + message.type);
  if (message.type === "AddSoundLaw") {
    console.log(
      `${message.law.left} ${message.law.right} ${message.law.from} ${message.law.to}`,
    );
    state.soundLawInputs.push(message.law);
    const law = create_law(
      message.law.left,
      message.law.right,
      message.law.from,
      message.law.to,
    );
    console.log(law);
    console.log("starting");
    state.composition.add_law(law); // mentions that null pointer passed to rust
    console.log("oops we didn't make it here");
    state.output = state.composition.transduce_text(state.input);
    state.revereseOutput = state.composition.transduce_text(state.reverseInput);
  } else if (message.type === "ChangeInput") {
    state.input = message.input;
  } else if (message.type === "ChangeBackwardsInput") {
    state.reverseInput = message.input;
  } else {
    //whatever
  }
  return state; //change this
};

const renderInit = (sendMessage: (message: Message) => void) => {
  const left = document.getElementById("left") as HTMLInputElement;
  const right = document.getElementById("right") as HTMLInputElement;
  const to = document.getElementById("to") as HTMLInputElement;
  const from = document.getElementById("from") as HTMLInputElement;

  const createButton = document.getElementById(
    "create-law",
  ) as HTMLButtonElement;

  createButton.addEventListener("click", () => {
    sendMessage({
      type: "AddSoundLaw",
      law: {
        left: left.value,
        right: right.value,
        from: from.value,
        to: to.value,
      },
    });
  });

  const input = document.getElementById("input") as HTMLInputElement;
  const backwards = document.getElementById("backward") as HTMLInputElement;
  input?.addEventListener("input", () =>
    sendMessage({ type: "ChangeInput", input: input.value }),
  );
  backwards?.addEventListener("input", () =>
    sendMessage({ type: "ChangeBackwardsInput", input: backwards.value }),
  );
};

const render = (state: State, sendMessage: (message: Message) => void) => {
  const output = document.getElementById("output") as HTMLParagraphElement;
  output.innerHTML = state.output.join("\n");
  const backwardsOutput = document.getElementById(
    "backwards-output",
  ) as HTMLParagraphElement;
  backwardsOutput.innerHTML = state.revereseOutput.join("\n");
  const rulesList = document.getElementById(
    "rulesList",
  ) as HTMLParagraphElement;

  rulesList.innerHTML = "";
  state.laws.forEach((x, index) => {
    let s = `Rule: ${x.get_from()} → ${x.get_to()} / ${x.get_left_context()} _ ${x.get_right_context()}`;
    const listItem = document.createElement("li");
    listItem.textContent = s;
    listItem.draggable = true;

    const deleteButton = document.createElement("button");
    deleteButton.textContent = "Delete";
    deleteButton.classList.add("delete-button");
    deleteButton.addEventListener("click", () => {
      sendMessage({ type: "ClickDelete", index: index });
    });

    listItem.appendChild(deleteButton);
    rulesList.appendChild(listItem);
  });
};

async function run() {
  await init();

  let state: State = {
    soundLawInputs: [],
    laws: [],
    input: "",
    output: [],
    reverseInput: "",
    revereseOutput: [],
    composition: SoundLawComposition.new(),
    fileStrings: [],
    transducedFileStrings: [],
  };

  const sendMessage = (message: Message) => {
    state = update(message, state);
    render(state, sendMessage);
  };

  renderInit(sendMessage);
}

run();
