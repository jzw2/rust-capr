import { RegexFst, SoundLaw } from "../pkg/rust_capr";
import { SoundClass } from "./types";

export class SaveButton {
  saveButton: HTMLButtonElement;
  loadButton: HTMLButtonElement;
  laws: SoundLaw[];

  soundClasses: Map<string, RegexFst>;

  saveListen: () => void;
  loadListen: () => void;
  constructor(soundClasses: Map<string, RegexFst>, laws: SoundLaw[]) {
    this.laws = laws;
    this.soundClasses = soundClasses;

    this.saveButton = document.getElementById("save") as HTMLButtonElement;
    this.loadButton = document.getElementById("load") as HTMLButtonElement;

    this.saveButton.addEventListener("click", () => {
      // ideally make this smarter
      localStorage.setItem("classes", JSON.stringify(this.soundClasses));
      // try to use rust for this loading side
      // localStorage.setItem("laws", JSON.stringify(state.soundLawInputs));
      alert("Broken right now, don't use this");
      console.log("Storing classes");
    });
    this.loadButton.addEventListener("click", () => {
      alert("Broken right now, don't use this");
      //todo!!!!!
      // it would be a lot better to handle this in the rust side of things
      // to make sure that we don't have horrible loading times
      //   const classesStorage = localStorage.getItem("classes");
      //         if (classesStorage) {
      //           state.soundClasses = JSON.parse(classesStorage);
      //         }
      //         const storage = localStorage.getItem("laws");
      //         if (storage) {
      //           state.composition = SoundLawComposition.new();
      //           state.laws = [];
      //           state.soundLawInputs = [];
      //           const laws = JSON.parse(storage) as SoundLawInput[];
      //           state.isLoading = true;
      //           setTimeout(() =>
      //             laws.forEach((law) => {
      //               state.soundLawInputs.push(law);
      //               updateLaw({ type: "AddSoundLaw", law: law }, state).then((msg) =>
      //                 sendMessage(msg),
      //               );
      //             }),
      //           );
    });
  }
}
