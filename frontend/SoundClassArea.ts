import { RegexType } from "./types";

export class SoundClassArea {
  regexSelect: HTMLSelectElement;
  regexType: RegexType;
  createSoundClass: HTMLButtonElement;
  area: HTMLDivElement;
  phonemes: HTMLInputElement; // todo: rename to a better name
  name: HTMLInputElement;

  constructor() {
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
      alert("Create sound law");
    });
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
