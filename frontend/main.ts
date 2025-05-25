import {
  create_with_arbitrary_regex_ipa,
  RegexFst,
  SoundLaw,
  SoundLawComposition,
} from "../pkg/rust_capr";
import { CreateSoundLaw } from "./CreateSoundLaw";
import { FileArea } from "./FileArea";
import { DragType, RegexType, SoundClass, SoundLawInput } from "./types";

export class Main {
  isLoading: boolean;
  // should not be needed anymore because the information should be in the soud law itself
  // soundLawInputs: SoundLawInput[];
  input: string;
  output: string[];
  reverseInput: string;
  revereseOutput: string[];
  composition: SoundLawComposition;
  fileStrings: string[];
  transducedFileStrings: string[][];
  drag: DragType;
  laws: SoundLaw[];

  fileArea: FileArea;
  createSoundLaw: CreateSoundLaw;
  loadingScreen: HTMLElement;
  soundClassesMap: Map<string, RegexFst>;

  constructor() {
    this.laws = [];
    this.soundClassesMap = new Map<string, RegexFst>();
    this.composition = SoundLawComposition.new();

    this.fileArea = new FileArea(this.transduce, this.composition);
    this.createSoundLaw = new CreateSoundLaw(
      () => this.displayLoadingScreen(true),
      this.addSoundLaw,
      this.soundClassesMap,
    );

    this.loadingScreen = document.getElementById("loading")!;
  }

  transduce() {
    this.fileArea.transduce();
  }

  displayLoadingScreen(loading: boolean) {
    if (loading) {
      this.loadingScreen.style.display = "block";
    } else {
      this.loadingScreen.style.display = "none";
    }
  }

  addSoundLaw(
    leftRegex: RegexFst,
    rightRegex: RegexFst,
    fromRegex: RegexFst,
    toRegex: RegexFst,
  ) {
    let law = create_with_arbitrary_regex_ipa(
      leftRegex,
      rightRegex,
      fromRegex,
      toRegex,
    );
    this.laws.push(law);
    //composition also has some dupliation, since the laws should laready be inside it, so I hsould redeisng this to also remove thsi redundancy
    this.composition.add_law(law); // mentions that null pointer passed to rust
  }
}
