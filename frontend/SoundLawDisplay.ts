import { SoundLaw } from "../pkg/rust_capr";

export class SoundLawDisplay {
  laws: SoundLaw[];
  rulesList: HTMLParagraphElement;
  deleteListner: (index: number) => void;
  rearrangeListener: (from: number, to: number) => void;
  constructor(
    laws: SoundLaw[],

    deleteListner: (index: number) => void,
    rearrangeListener: (from: number, to: number) => void,
  ) {
    this.rearrangeListener = rearrangeListener;
    this.laws = laws;
    this.deleteListner = deleteListner;

    this.rulesList = document.getElementById(
      "rulesList",
    ) as HTMLParagraphElement;
  }
  render() {
    this.rulesList.replaceChildren();
    // console.log(this.laws);

    this.laws.forEach((x, index) => {
      let left = x.get_left_context();
      let right = x.get_right_context();
      let s = `Rule: ${x.get_from()} → ${x.get_to()} / ${left} _ ${right}`;
      const listItem = document.createElement("li");
      listItem.textContent = s;
      listItem.draggable = true;

      const deleteButton = document.createElement("button");
      deleteButton.textContent = "Delete";
      deleteButton.classList.add("delete-button");
      deleteButton.addEventListener("click", () => {
        this.deleteListner(index);
      });
      const list = document.getElementById("rulesList") as HTMLUListElement;

      const input = document.createElement("input");
      const button = document.createElement("button");
      button.innerHTML = "Click to Move Index";

      button.addEventListener("click", () => {
        const newIndex = parseInt(input.value, 10) - 1; // because lists are 1 based

        if (!isNaN(newIndex)) {
          this.rearrangeListener(index, newIndex);
        } else {
          console.log("Invalid index input");
        }
      });

      listItem.appendChild(deleteButton);
      listItem.appendChild(input);
      listItem.appendChild(button);
      this.rulesList.appendChild(listItem);
    });
  }
}
