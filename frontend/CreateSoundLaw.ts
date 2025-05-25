import { RegexFst } from "../pkg/rust_capr";
import { Main } from "./main";
import { SoundClassName } from "./types";

export class CreateSoundLaw {
  left: HTMLInputElement;
  right: HTMLInputElement;
  from: HTMLInputElement;
  to: HTMLInputElement;
  createButton: HTMLButtonElement;

  leftSelect: HTMLInputElement;
  rightSelect: HTMLInputElement;
  parent: Main;
  soundClasses: Map<string, RegexFst>;
  constructor(parent: Main, soundClasses: Map<String, RegexFst>) {
    this.soundClasses = soundClasses;
    this.parent = parent;
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
      this.parent.displayLoadingScreen(true);

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
        parent.addSoundLaw(leftRegex, rightRegex, fromRegex, toRegex),
      );
    });
  }
}
