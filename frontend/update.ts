import { sendMessage } from "..";
import {
  create_with_disjunctions,
  Disjunction,
  SoundLawComposition,
} from "../pkg/rust_capr";
import { Message, State, SoundLawInput, AddSoundLaw } from "./types";

// Creating sound laws take a decent amout of time
export const updateLaw = async (
  message: AddSoundLaw,
  state: State,
): Promise<Message> => {
  let left: string[] = [];
  let right: string[] = [];
  let oldLeft = message.law.left;
  if (typeof oldLeft == "string") {
    left = [oldLeft];
  } else {
    let x = state.soundClasses.find(
      (sound_class) => sound_class.name == oldLeft.name,
    );
    if (x) {
      left = x.sounds;
    }
  }
  let oldright = message.law.right;
  if (typeof oldright == "string") {
    right = [oldright];
  } else {
    let x = state.soundClasses.find(
      (sound_class) => sound_class.name == oldright.name,
    );
    if (x) {
      right = x.sounds;
    }
  }
  const law = create_with_disjunctions(
    Disjunction.new(left),
    Disjunction.new(right),
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

export const transduce = (state: State): State => {
  state.output = state.composition.transduce_text(state.input);
  state.revereseOutput = state.composition.transduce_text(state.reverseInput);
  state.transducedFileStrings = state.fileStrings.map((s) =>
    state.composition.transduce_text(s),
  );
  return state;
};
// If a message is sent, it takes the old state and
// returns a new state based on what the message was
export const update = (message: Message, state: State): State => {
  console.log("Found message: ", message);
  if (message.type === "AddSoundLaw") {
    state.soundLawInputs.push(message.law);
    state.isLoading = true;
    // Promise.resolve()
    //   .then(() => updateLaw(message, state))
    //   .then((res) => sendMessage(res));
    setTimeout(() => updateLaw(message, state).then((msg) => sendMessage(msg)));
    //updateLaw(message, state).then((msg) => sendMessage(msg));
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
    state = transduce(state);
  } else if (message.type === "UploadFile") {
    state.fileStrings = message.contents.split("\n").filter((x) => x !== "");
    state.transducedFileStrings = state.fileStrings.map((s) =>
      state.composition.transduce_text(s),
    );
    state = transduce(state);
  } else if (message.type === "ClickDelete") {
    state.composition.rm_law(message.index);
    state.laws.splice(message.index, 1);
    state.soundLawInputs.splice(message.index, 1);
    state = transduce(state);
  } else if (message.type === "Rearrange") {
    const oldIndex = message.old;
    const newIndex = message.new;
    const [movedInput] = state.soundLawInputs.splice(oldIndex, 1);
    const [movedLaw] = state.laws.splice(oldIndex, 1);
    state.laws.splice(newIndex, 0, movedLaw);
    state.soundLawInputs.splice(newIndex, 0, movedInput);
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
      state.drag.type = "NoDrag";
    } else {
      console.log("Drag over called without first starting a drag");
    }
  } else if (message.type === "Save") {
    localStorage.setItem("classes", JSON.stringify(state.soundClasses));
    localStorage.setItem("laws", JSON.stringify(state.soundLawInputs));
    console.log("Storing classes");
  } else if (message.type === "Load") {
    const classesStorage = localStorage.getItem("classes");
    if (classesStorage) {
      state.soundClasses = JSON.parse(classesStorage);
    }
    const storage = localStorage.getItem("laws");
    if (storage) {
      state.composition = SoundLawComposition.new();
      state.laws = [];
      state.soundLawInputs = [];
      const laws = JSON.parse(storage) as SoundLawInput[];
      state.isLoading = true;
      setTimeout(() =>
        laws.forEach((law) => {
          state.soundLawInputs.push(law);
          updateLaw({ type: "AddSoundLaw", law: law }, state).then((msg) =>
            sendMessage(msg),
          );
        }),
      );
    }
  } else if (message.type === "ChangeRegexType") {
    state.regexType = message.regex;
  } else if (message.type === "AddSoundClass") {
    state.soundClasses.push({
      type: "Disjunction",
      name: message.name,
      sounds: message.sounds,
    });
  } else {
    //whatever
    console.log("Very bad, message was not found");
  }
  return { ...state }; //change this
};
