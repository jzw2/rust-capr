import init, {
  create_law_async,
  SoundLaw,
  SoundLawComposition,
  create_law,
  create_with_disjunctions,
  Disjunction,
} from "./pkg/rust_capr";

import {
  AddSoundLaw,
  Message,
  State,
  CMessage,
  DragType,
} from "./frontend/types.ts";
import { update } from "./frontend/update.ts";
import { render, renderInit } from "./frontend/render.ts";

// Send message needs to have access to the state
// which isn't created until after the rust stuff gets loaded
// so initially we give it a dummy function
export let sendMessage = (_: Message) => {
  console.log("Send Message not initialize properly");
};

// import { Drawable } from "./krist_lib/editor";
// import Victor from "victor";
// import { RuleNode } from "./krist_lib/rule-node";

async function run() {
  await init();

  let state: State = {
    soundClasses: [],
    regexType: { type: "Union" },
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
    //state = message.updateState(state); //todo: refactor this in
    render(state);
  };

  renderInit();
}

run();
