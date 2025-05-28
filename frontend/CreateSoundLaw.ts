import { RegexFst } from "../pkg/rust_capr";
import { SoundClassName } from "./types";

type NewType = void;

export class CreateSoundLaw {
  left: HTMLInputElement;
  right: HTMLInputElement;
  from: HTMLInputElement;
  to: HTMLInputElement;
  createButton: HTMLButtonElement;

  leftSelect: HTMLInputElement;
  rightSelect: HTMLInputElement;
  soundClasses: Map<string, RegexFst>;
  setLoadingListener: () => void;
  createSoundLawListner: (
    arg0: RegexFst,
    arg1: RegexFst,
    arg2: RegexFst,
    arg3: RegexFst,
  ) => void;

  constructor(
    setLoadingListiner: () => void,
    createSoundLawListner: (
      arg0: RegexFst,
      arg1: RegexFst,
      arg2: RegexFst,
      arg3: RegexFst,
    ) => void,
    soundClasses: Map<string, RegexFst>,
  ) {
    this.createSoundLawListner = createSoundLawListner;
    this.setLoadingListener = setLoadingListiner;
    this.soundClasses = soundClasses;
    this.left = document.getElementById("left-input") as HTMLInputElement;
    this.right = document.getElementById("right-input") as HTMLInputElement;
    this.to = document.getElementById("to") as HTMLInputElement;
    this.from = document.getElementById("from") as HTMLInputElement;

    this.leftSelect = document.querySelector(
      "#left-select",
    ) as HTMLInputElement;
    this.rightSelect = document.querySelector(
      "#right-select",
    ) as HTMLInputElement;
    this.createButton = document.getElementById(
      "create-law",
    ) as HTMLButtonElement;

    this.createButton.addEventListener("click", () => {
      this.setLoadingListener();

      let leftRegex: RegexFst;
      let rightRegex: RegexFst;
      let fromRegex: RegexFst;
      let toRegex: RegexFst;

      let l: string | SoundClassName = this.left.value;
      let r: string | SoundClassName = this.right.value;
      if (this.leftSelect.value === "") {
        leftRegex = RegexFst.new_from_ipa(this.left.value);
      } else {
        // actually check for undefined
        leftRegex = soundClasses.get(this.leftSelect.value)!;
      }
      if (this.rightSelect.value === "") {
        rightRegex = RegexFst.new_from_ipa(this.right.value);
      } else {
        // actually check for undefined
        rightRegex = soundClasses.get(this.rightSelect.value)!;
      }

      fromRegex = RegexFst.new_from_ipa(this.from.value);
      toRegex = RegexFst.new_from_ipa(this.to.value);

      setTimeout(() =>
        // create the new law
        this.createSoundLawListner(leftRegex, rightRegex, fromRegex, toRegex),
      );
    });
  }
}
