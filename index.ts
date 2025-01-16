import init, {
  create_law,
  SoundLaw,
  transduce_context,
  transduce_context_invert,
  SoundLawComposition,
  soundlaw_xsampa_to_ipa,
} from "./pkg/rust_capr";

// Send message needs to have access to the state
// which isn't created until after the rust stuff gets loaded
// so initially we give it a dummy function
let sendMessage = (_: Message) => {
  console.log("Send Message not initialize properly");
};

// import { Drawable } from "./krist_lib/editor";
// import Victor from "victor";
// import { RuleNode } from "./krist_lib/rule-node";

type AddSoundLaw = { type: "AddSoundLaw"; law: SoundLawInput };
type UploadFile = { type: "UploadFile"; contents: string };
type StartDrag = { type: "StartDrag"; index: number };
type HoveringOver = { type: "HoveringOver"; index: number };
type ClickDelete = { type: "ClickDelete"; index: number };

// Sends a message that the state needs to be updated in some way
type Message =
  | AddSoundLaw
  | UploadFile
  | StartDrag
  | HoveringOver
  | ClickDelete
  | { type: "ChangeInput"; input: string }
  | { type: "ChangeBackwardsInput"; input: string }
  | { type: "Rearrange"; old: number; new: number }
  | { type: "Transduce" }
  | {
      type: "FinishLoading";
      laws: SoundLaw[];
      composition: SoundLawComposition;
    }
  | { type: "DragStart"; index: number }
  | { type: "DragOver"; index: number }
  | { type: "DragEnd" };

//todo: refactor so it isn't so big
type State = {
  isLoading: boolean;
  soundLawInputs: SoundLawInput[];
  laws: SoundLaw[];
  input: string;
  output: string[];
  reverseInput: string;
  revereseOutput: string[];
  composition: SoundLawComposition;
  fileStrings: string[];
  transducedFileStrings: string[][];
  drag: DragType;
};

type DragType =
  | { type: "NoDrag" }
  | { type: "DraggingOver"; old: number; new: number };

type SoundLawInput = {
  left: string;
  right: string;
  to: string;
  from: string;
};

// Creating sound laws take a decent amout of time
const updateLaw = (message: AddSoundLaw, state: State): Message => {
  const law = create_law(
    message.law.left,
    message.law.right,
    message.law.from,
    message.law.to,
  );
  state.laws.push(law);
  state.composition.add_law(law); // mentions that null pointer passed to rust
  console.log("Finished computation");
  return {
    type: "FinishLoading",
    laws: state.laws,
    composition: state.composition,
  };
};

const transduce = (state: State): State => {
  state.output = state.composition.transduce_text(state.input);
  state.revereseOutput = state.composition.transduce_text(state.reverseInput);
  state.transducedFileStrings = state.fileStrings.map((s) =>
    state.composition.transduce_text(s),
  );
  return state;
};

// If a message is sent, it takes the old state and
// returns a new state based on what the message was
const update = (message: Message, state: State) => {
  console.log("Found message" + message.type);
  if (message.type === "AddSoundLaw") {
    console.log(
      `${message.law.left} ${message.law.right} ${message.law.from} ${message.law.to}`,
    );
    message;
    state.soundLawInputs.push(message.law);
    state.isLoading = true;
    // render(state, sendMessage);
    setTimeout(() => sendMessage(updateLaw(message, state)));
    // new Promise((resolve, reject) => {
    //   sendMessage(updateLaw(message, state));
    //   resolve(null);
    // });
    //render(state);
  } else if (message.type === "ChangeInput") {
    state.input = message.input;
    state.output = state.composition.transduce_text(state.input);
  } else if (message.type === "ChangeBackwardsInput") {
    state.reverseInput = message.input;
    state.revereseOutput = state.composition.transduce_text_invert(
      state.reverseInput,
    );
  } else if (message.type === "FinishLoading") {
    state.isLoading = false;
    state.laws = message.laws;
    state.composition = message.composition;
    console.log(state.transducedFileStrings);
    state = transduce(state);
  } else if (message.type === "UploadFile") {
    state.fileStrings = message.contents.split("\n").filter((x) => x !== "");
    state.transducedFileStrings = state.fileStrings.map((s) =>
      state.composition.transduce_text(s),
    );
    console.log(state.transducedFileStrings);
    state = transduce(state);
  } else if (message.type === "ClickDelete") {
    state.composition.rm_law(message.index);
    state.laws.splice(message.index, 1);
    state = transduce(state);
  } else if (message.type === "Rearrange") {
    const oldIndex = message.old;
    const newIndex = message.new;
    const [movedLaw] = state.laws.splice(oldIndex, 1);
    state.laws.splice(newIndex, 0, movedLaw);
    state.composition = SoundLawComposition.new();
    state.laws.forEach((law) => state.composition.add_law(law));
    state = transduce(state);
  } else if (message.type === "Transduce") {
    state = transduce(state);
  } else if (message.type === "DragStart") {
    state.drag = {
      type: "DraggingOver",
      old: message.index,
      new: message.index,
    }; // start with dragging over self
  } else if (message.type === "DragOver") {
    if (state.drag.type == "DraggingOver") {
      state.drag.new = message.index;
    } else {
      console.log("Drag over called without first starting a drag");
    }
  } else if (message.type === "DragEnd") {
    if (state.drag.type == "DraggingOver") {
      const oldIndex = state.drag.old;
      const newIndex = state.drag.new;
      const [movedLaw] = state.laws.splice(oldIndex, 1);
      state.laws.splice(newIndex, 0, movedLaw);
      state.composition = SoundLawComposition.new();
      state.laws.forEach((law) => state.composition.add_law(law));
      state = transduce(state);
    } else {
      console.log("Drag over called without first starting a drag");
    }
  } else {
    //whatever
    console.log("Very bad, message was not found");
  }
  return state; //change this
};

// Since some things are only set once
// putting it in render seems to be kind of wasteful
// this function is only called once
const renderInit = () => {
  const uploadFile = document.getElementById("upload") as HTMLInputElement;

  uploadFile.addEventListener("change", async () => {
    if (uploadFile.files) {
      let file = uploadFile.files[0];
      const text = await file.text();
      sendMessage({ type: "UploadFile", contents: text });
    } else {
      sendMessage({ type: "UploadFile", contents: "" });
    }
  });

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

  const list = document.getElementById("rulesList") as HTMLUListElement;

  list.addEventListener("dragstart", (e: DragEvent) => {
    const target = e.target as HTMLElement;
    const index = Array.from(list.children).indexOf(target);
    sendMessage({ type: "DragStart", index: index });
  });

  list.addEventListener("dragover", (e: DragEvent) => {
    const target = e.target as HTMLElement;
    const index = Array.from(list.children).indexOf(target);
    sendMessage({ type: "DragOver", index: index });
  });
  list.addEventListener("dragend", (e: DragEvent) => {
    sendMessage({ type: "DragEnd" });
  });
};

const render = (state: State) => {
  const loading = document.getElementById("loading");
  if (loading) {
    //console.log("isLoading: " + state.isLoading);
    if (state.isLoading) {
      loading.style.display = "block";
    } else {
      loading.style.display = "none";
    }
    // loading.style.display = "block";
  }
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

  let table = document.getElementById("file-inputs") as HTMLTableRowElement;
  table.innerHTML = ' <thead> <tr id="file-headers"></tr> </thead> ';
  let tableHeader = document.getElementById(
    "file-headers",
  ) as HTMLTableRowElement;

  state.fileStrings.forEach((line) => {
    const item = document.createElement("th");
    item.textContent = line;
    tableHeader.appendChild(item);
  });

  if (state.transducedFileStrings.length > 0) {
    const transpose = state.transducedFileStrings[0].map((_, index) =>
      state.transducedFileStrings.map((row) => row[index]),
    );

    transpose.forEach((row) => {
      const htmlRow = document.createElement("tr");
      row.forEach((col) => {
        const item = document.createElement("td");
        item.textContent = col;
        htmlRow.appendChild(item);
      });
      table.append(htmlRow);
    });
  }

  console.log("Finished rendering");
};

async function run() {
  await init();

  let state: State = {
    isLoading: false,
    soundLawInputs: [],
    laws: [],
    input: "",
    output: [],
    reverseInput: "",
    revereseOutput: [],
    composition: SoundLawComposition.new(),
    fileStrings: [],
    transducedFileStrings: [],
    drag: { type: "NoDrag" },
  };

  sendMessage = (message: Message) => {
    state = update(message, state);
    render(state);
  };

  renderInit();
}

run();
