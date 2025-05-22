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
  constructor(parent: Main) {
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
      let l: string | SoundClassName = this.left.value;
      let r: string | SoundClassName = this.right.value;
      if (this.leftSelect.value !== "") {
        l = { name: this.leftSelect.value };
      }
      if (this.rightSelect.value !== "") {
        r = { name: this.rightSelect.value };
      }
      this.parent.displayLoadingScreen(true);

      // Promise.resolve()
      //   .then(() => updateLaw(message, state))
      //   .then((res) => sendMessage(res));
      setTimeout(() =>
        // create the new law
      );
    });
  }
}
