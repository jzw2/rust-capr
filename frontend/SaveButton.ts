import { RegexFst, SoundLaw } from "../pkg/rust_capr";
import { SoundClass } from "./types";

export class SaveButton {
  saveButton: HTMLButtonElement;
  loadButton: HTMLButtonElement;
  laws: SoundLaw[];

  soundClasses: Map<string, RegexFst>;

  loadListen: () => void;
  constructor(
    soundClasses: Map<string, RegexFst>,
    laws: SoundLaw[],
    loadListen: () => void,
  ) {
    this.loadListen = loadListen;
    this.laws = laws;
    this.soundClasses = soundClasses;

    this.saveButton = document.getElementById("save") as HTMLButtonElement;
    this.loadButton = document.getElementById("load") as HTMLButtonElement;

    this.saveButton.addEventListener("click", () => {
      // ideally make this smarter
      let stringMap = new Map<string, string>();
      this.soundClasses.forEach((v, k) => {
        console.log(v.to_json());
        stringMap.set(k, v.to_json());
      });
      localStorage.setItem("classes", JSON.stringify(stringMap));
      //console.log(this.soundClasses);
      console.log(stringMap);
      console.log("Sound Classes");
      console.log(JSON.stringify(stringMap));

      let soundLawStrings = this.laws.map((x) => x.to_json());
      localStorage.setItem("laws", JSON.stringify(soundLawStrings));
      console.log("Laws");
      console.log(JSON.stringify(soundLawStrings));
      alert("Saved!");
    });
    this.loadButton.addEventListener("click", () => {
      //alert("Broken right now, don't use this");
      const classesStorage = localStorage.getItem("classes");
      if (classesStorage) {
        let classes = JSON.parse(classesStorage) as Map<string, string>;
        classes.forEach((v, k) => {
          this.soundClasses.set(k, RegexFst.from_json(v));
        });
      } else {
        alert("No sound classes found");
      }
      const storage = localStorage.getItem("laws");
      if (storage) {
        const laws = JSON.parse(storage) as string[];
        const parsed = laws.map((s) => SoundLaw.from_json(s));
        this.laws.push(...parsed);
      } else {
        alert("No sound laws were found");
      }
      loadListen();
    });
  }
}
