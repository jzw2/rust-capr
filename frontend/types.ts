import { RegexFst, SoundLaw, SoundLawComposition } from "../pkg/rust_capr";

export type AddSoundLaw = {
  type: "AddSoundLaw";
  law: SoundLawInput;
};
export type UploadFile = { type: "UploadFile"; contents: string };
export type StartDrag = { type: "StartDrag"; index: number };
export type HoveringOver = { type: "HoveringOver"; index: number };
export type ClickDelete = { type: "ClickDelete"; index: number };

// Sends a message that the state needs to be updated in some way
export type Message =
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
  | { type: "Save" }
  | { type: "Load" }
  | { type: "ChangeRegexType"; regex: RegexType }
  | { type: "AddSoundClass"; regex: RegexType; name: string; sounds: string[] }
  | { type: "DragEnd" };

export type RegexType =
  | { type: "Union" }
  | { type: "Disjunction" }
  | { type: "Concat" }
  | { type: "Star" };

export abstract class CMessage {
  abstract updateState(state: State): State;
}

export type SoundClass = {
  type: RegexType;
  name: string;
  sounds: string[];
  fst: RegexFst;
};

//todo: refactor so it isn't so big
export type State = {
  soundClasses: SoundClass[];
  regexType: RegexType;
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

export type DragType =
  | { type: "NoDrag" }
  | { type: "DraggingOver"; old: number; new: number };

// this needs to be updated to include the fst, which can be anarbitrary regex
export type SoundLawInput = {
  left: string | SoundClassName;
  right: string | SoundClassName;
  to: string;
  from: string;
};

export type SoundClassName = {
  name: string;
};
