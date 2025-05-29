import { sendMessage } from "..";
import {
  create_with_arbitrary_regex_ipa,
  create_with_disjunctions,
  Disjunction,
  SoundLawComposition,
  RegexFst,
  SoundLaw,
  soundlaw_xsampa_to_ipa,
} from "../pkg/rust_capr";
import {
  Message,
  State,
  SoundLawInput,
  AddSoundLaw,
  SoundClassName,
  SoundClass,
} from "./types";

// Creating sound laws take a decent amout of time
export const updateLaw = async (
  message: AddSoundLaw,
  state: State,
): Promise<Message> => {
  let left: RegexFst;
  let right: RegexFst;
  let oldLeft = message.law.left;
  //great consistency in naming
  let oldright = message.law.right;
  if (typeof oldLeft == "string") {
    left = RegexFst.new_from_ipa(oldLeft);
  } else {
    left = state.soundClasses.find(
      (soundClass) => soundClass.name == oldLeft.name,
    )!.fst;
  }
  if (typeof oldright == "string") {
    right = RegexFst.new_from_ipa(oldright);
  } else {
    right = state.soundClasses.find(
      (soundClass) => soundClass.name == oldright.name,
    )!.fst;
  }
  let from = RegexFst.new_from_ipa(message.law.from); // account for when doing a sound law later
  let to = RegexFst.new_from_ipa(message.law.to); // account for when doing a sound law later

  let law = create_with_arbitrary_regex_ipa(left, right, from, to);
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
    state.fileStrings = message.contents
      .split("\n")
      .filter((x) => x !== "")
      .map((s) => soundlaw_xsampa_to_ipa(s));
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
    let regexType = message.regex;
    let fst: RegexFst;
    if (regexType.type == "Disjunction") {
      let regexfsts = message.sounds.map((sound) =>
        RegexFst.new_from_ipa(sound),
      );
      fst = regexfsts[0]; // should check to make sure it is not emty
      regexfsts.slice(1).forEach((r) => fst.disjoint(r)); //theoritcally works by doing dijsoint with self, but rust prevents this
    } else if (regexType.type == "Concat") {
      let regexfsts = message.sounds.map(
        (sound) =>
          // todo fix this
          state.soundClasses.find((soundClass) => soundClass.name == sound)!
            .fst,
      );
      fst = regexfsts[0]; // should check to make sure it is not emty
      regexfsts.slice(1).forEach((r) => fst.concat(r)); //theoritcally works by doing dijsoint with self, but rust prevents this
    } else {
      // todo! fix this
      fst = RegexFst.new_from_ipa("error");
    }

    state.soundClasses.push({
      type: message.regex,
      name: message.name,
      sounds: message.sounds,
      fst: fst,
    });
  } else {
    //whatever
    console.log("Very bad, message was not found");
  }
  return { ...state }; //change this
};
