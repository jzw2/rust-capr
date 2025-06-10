import {
  SoundLawComposition,
  SoundLaw,
  RegexFst,
  create_with_arbitrary_regex_ipa,
} from "../pkg/rust_capr";
import { CreateSoundLaw } from "./CreateSoundLaw";
import { FileArea } from "./FileArea";
import { ForwardBackwards } from "./ForwardBackwards";
import { SaveButton } from "./SaveButton";
import { SoundClassArea } from "./SoundClassArea";
import { SoundLawDisplay } from "./SoundLawDisplay";
import { DragType } from "./types";

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
  soundLawDisplay: SoundLawDisplay;
  forwardBackwards: ForwardBackwards;
  saveButton: SaveButton;
  soundClassArea: SoundClassArea;

  constructor() {
    this.laws = [];
    this.soundClassesMap = new Map<string, RegexFst>();
    this.composition = SoundLawComposition.new();
    this.forwardBackwards = new ForwardBackwards(this.composition);
    this.saveButton = new SaveButton(this.soundClassesMap, this.laws, () => {
      this.soundClassArea.renderSoundClassDisplay();
      this.soundLawDisplay.render();
      this.createSoundLaw.updateSoundClasses();
    });
    this.soundClassArea = new SoundClassArea(this.soundClassesMap, () => {
      this.createSoundLaw.updateSoundClasses();
    });

    let deleteListener = (index: number) => {
      this.composition.rm_law(index);
      this.laws.splice(index, 1);
      // state.soundLawInputs.splice(message.index, 1);
      this.transduce();
      this.soundLawDisplay.render();
    };

    let rearrangeListener = (from: number, to: number) => {
      const oldIndex = from;
      const newIndex = to;
      // const [movedInput] = this.soundLawInputs.splice(oldIndex, 1);
      const [movedLaw] = this.laws.splice(oldIndex, 1);
      this.laws.splice(newIndex, 0, movedLaw);
      // this.soundLawInputs.splice(newIndex, 0, movedInput);
      this.composition.clear();

      //probably not that that efficient
      this.laws.forEach((law) => this.composition.add_law(law));
      this.transduce();
      this.soundLawDisplay.render();
    };

    this.soundLawDisplay = new SoundLawDisplay(
      this.laws,
      deleteListener,
      rearrangeListener,
    );

    // javascript is so stupid, so I have to use () => this.transduce
    this.fileArea = new FileArea(() => this.transduce(), this.composition);
    this.createSoundLaw = new CreateSoundLaw(
      () => this.displayLoadingScreen(true),
      (a, b, c, d) => {
        this.addSoundLaw(a, b, c, d);
        this.displayLoadingScreen(false);
        this.soundLawDisplay.render();
      },
      this.soundClassesMap,
    );

    this.loadingScreen = document.getElementById("loading")!;
  }

  transduce() {
    this.fileArea.transduce();
    this.forwardBackwards.transduce();
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
