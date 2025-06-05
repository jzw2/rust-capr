import { RegexFst, soundlaw_xsampa_to_ipa } from "../pkg/rust_capr";
import { RegexType } from "./types";

export class SoundClassArea {
  regexSelect: HTMLSelectElement;
  regexType: RegexType;
  createSoundClass: HTMLButtonElement;
  area: HTMLDivElement;
  phonemes: HTMLInputElement; // todo: rename to a better name
  name: HTMLInputElement;
  soundClassesMap: Map<string, RegexFst>;
  addSoundClassListener: () => void;

  constructor(
    soundClassesMap: Map<string, RegexFst>,
    addSoundClassListener: () => void,
  ) {
    this.addSoundClassListener = addSoundClassListener;
    this.soundClassesMap = soundClassesMap;
    this.area = document.querySelector(".sound-class-area") as HTMLDivElement;
    this.regexSelect = document.getElementById("regex") as HTMLSelectElement;
    this.regexType = { type: this.regexSelect.value } as RegexType;
    this.regexSelect.addEventListener("input", (e) => {
      if (this.regexType.type != this.regexSelect.value) {
        this.regexType = { type: this.regexSelect.value } as RegexType;
        this.render();
      }
    });

    this.createSoundClass = document.querySelector(
      ".create-sound-class",
    ) as HTMLButtonElement;
    this.createSoundClass.addEventListener("click", () => {
      let nameValue = this.name.value;
      let sounds = soundlaw_xsampa_to_ipa(this.phonemes.value).split(" ");
      if (this.regexType.type == "Disjunction") {
        let regexfsts = sounds.map((sound) => RegexFst.new_from_ipa(sound));
        if (regexfsts.length == 0) {
          alert("don't make something with empty sounds");
          return;
        }

        let fst = regexfsts[0]; // should check to make sure it is not emty
        regexfsts.slice(1).forEach((r) => fst.disjoint(r)); //theoritcally works by doing dijsoint with self, but rust prevents this
        this.soundClassesMap.set(nameValue, fst);
      } else if (this.regexType.type == "Concat") {
        let bad = false;
        let regexfsts = sounds.map((sound) => {
          let f = this.soundClassesMap.get(sound);
          if (!f) {
            alert(`Sound class for ${sound} not found`);
            bad = true;
            return;
          }
          return f;
        });
        if (bad) {
          return;
        }
        if (regexfsts.length == 0) {
          alert("Sound classes was empty");
          return;
        }
        let fst = regexfsts[0]!; // should check to make sure it is not emty
        regexfsts.slice(1).forEach((r) => fst.concat(r!)); //theoritcally works by doing dijsoint with self, but rust prevents this
        this.soundClassesMap.set(nameValue, fst);
      }
      this.renderSoundClassDisplay();
      this.addSoundClassListener();
    });
  }

  renderSoundClassDisplay() {
    const soundList = document.querySelector("#sound-classes");
    if (soundList) {
      soundList.innerHTML = "";
      this.soundClassesMap.forEach((fst, name) => {
        const li = document.createElement("li");
        li.innerHTML = `${name}: ${fst.string_form()}`;
        soundList.append(li);
      });
    }
  }

  render() {
    this.area.innerHTML = ""; // probably should do this the right way
    let instructions = "";
    if (this.regexType.type == "Disjunction") {
      instructions = "Put the phonemes with one space between each one";
    } else if (this.regexType.type == "Concat") {
      instructions = "Put the sound classes with one space in between";
    } else {
      alert("This sound class is not yet supported");
      return;
    }
    const nameDiv = document.createElement("div");
    const nameHeader = document.createElement("p");
    nameHeader.innerHTML = "Name";
    nameDiv.append(nameHeader);
    this.name = document.createElement("input");
    this.name.classList.add("name");
    nameDiv.append(this.name);
    this.area?.append(nameDiv);

    const instructionsElem = document.createElement("p");
    instructionsElem.innerHTML =
      "Put the phonemes with one space between each one";
    this.phonemes = document.createElement("input");
    this.phonemes.classList.add("input");
    this.area?.append(instructionsElem);

    this?.area.append(this.phonemes);
  }
}
