export class CreateSoundLaw {
  left: HTMLInputElement;
  right: HTMLInputElement;
  from: HTMLInputElement;
  to: HTMLInputElement;
  constructor() {
    this.left = document.getElementById("left-input") as HTMLInputElement;
    this.right = document.getElementById("right-input") as HTMLInputElement;
    this.to = document.getElementById("to") as HTMLInputElement;
    this.from = document.getElementById("from") as HTMLInputElement;
  }
}
