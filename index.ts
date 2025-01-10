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
type UploadFile = { type: "UploadFile"; contents: String };
type StartDrag = { type: "StartDrag"; index: number };
type HoveringOver = { type: "HoveringOver"; index: number };
type ClickDelete = { type: "ClickDelete"; index: number };

type Message =
  | AddSoundLaw
  | UploadFile
  | StartDrag
  | HoveringOver
  | ClickDelete;

//todo: refactor so it isn't so big
type State = {
  soundLawInputs: SoundLawInput;
  laws: SoundLaw[];
  input: String;
  output: String[];
  reverseInput: String;
  revereseOutput: String[];
  composition: SoundLawComposition;
  fileStrings: String[];
  transducedFileStrings: String[][];
};

type SoundLawInput = {
  left: string;
  right: string;
  to: string;
  from: string;
};

const update = (message: Message, state: State) => {
  if (message.type === "AddSoundLaw") {
  } else {
    //whatever
  }
  return state; //change this
};

const renderInit = (sendMessage: (message: Message) => void) => {
  const
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
    soundLawInputs: {
      left: "",
      right: "",
      to: "",
      from: "",
    },
    laws: [],
    input: "",
    output: [],
    reverseInput: "",
    revereseOutput: [],
    composition: new SoundLawComposition(),
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
