import { SoundLaw, SoundLawComposition } from "./pkg/rust_capr";

export type AddSoundLaw = {
  type: "AddSoundLaw";
  law: SoundLawInput;
  loading: boolean;
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
  | { type: "DragEnd" };

export abstract class CMessage {
  abstract updateState(state: State): State;
}

//todo: refactor so it isn't so big
export type State = {
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

export type SoundLawInput = {
  left: string;
  right: string;
  to: string;
  from: string;
};
