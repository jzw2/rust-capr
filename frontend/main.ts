import { SoundLaw, SoundLawComposition } from "../pkg/rust_capr";
import { DragType, RegexType, SoundClass, SoundLawInput } from "./types";

class FileArea {
  uploadFile: HTMLInputElement;
  fileStrings: String[];
  tranducer: SoundLawComposition;
  parent: Main;
  constructor(parent: Main) {
    this.uploadFile = document.getElementById("upload") as HTMLInputElement;

    this.uploadFile.addEventListener("change", async () => {
      if (this.uploadFile.files) {
        let file = this.uploadFile.files[0];
        const text = await file.text();

        this.fileStrings = text.split("\n").filter((x) => x !== "");
        this.parent.transduce();
      } else {
        this.fileStrings = [];
      }
    });
  }

  transduce() {}
}

class TranductionArea {
  laws: SoundLaw[];

  constructor() {}
}

class SoundClassCreation {
  regexType: RegexType;
}

class Main {
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

  constructor() {
    this.fileArea = new FileArea(this);
  }

  transduce() {
    //todo
  }
}
