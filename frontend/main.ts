import { SoundLaw, SoundLawComposition } from "../pkg/rust_capr";
import { CreateSoundLaw } from "./CreateSoundLaw";
import { FileArea } from "./FileArea";
import { DragType, RegexType, SoundClass, SoundLawInput } from "./types";

export class Main {
  soundClasses: SoundClass[];
  isLoading: boolean;
  soundLawInputs: SoundLawInput[];
  input: string;
  output: string[];
  reverseInput: string;
  revereseOutput: string[];
  composition: SoundLawComposition;
  fileStrings: string[];
  transducedFileStrings: string[][];
  drag: DragType;

  fileArea: FileArea;
  createSoundLaw: CreateSoundLaw;

  constructor() {
    this.composition = SoundLawComposition.new();

    this.fileArea = new FileArea(this, this.composition);
    this.createSoundLaw = new CreateSoundLaw();
  }

  transduce() {
    this.fileArea.transduce();
  }
}
