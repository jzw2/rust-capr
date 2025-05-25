import { soundlaw_xsampa_to_ipa, SoundLawComposition } from "../pkg/rust_capr";

// probably not the most descriptive name
class ForwardBackwards {
  input: HTMLInputElement;
  backwards: HTMLInputElement;
  output: HTMLParagraphElement;
  backwardsOutput: HTMLParagraphElement;
  composition: SoundLawComposition;
  constructor() {
    this.input = document.getElementById("input") as HTMLInputElement;
    this.backwards = document.getElementById("backward") as HTMLInputElement;
    this.output = document.getElementById("output") as HTMLParagraphElement;
    this.backwardsOutput = document.getElementById(
      "backwards-output",
    ) as HTMLParagraphElement;

    this.input.addEventListener("input", () => {
      this.transduce();
    });
    this.backwards.addEventListener("input", () => {
      this.transduce();
    });
  }

  transduce() {
    let r = this.composition.transduce_text_invert(this.backwards.value);
    this.backwardsOutput.innerHTML = r
      .map((x) => soundlaw_xsampa_to_ipa(x.replaceAll(" ", "")))
      .join("\n");

    let o = this.composition.transduce_text(this.input.value);
    this.output.innerHTML = o
      .map((x) => soundlaw_xsampa_to_ipa(x.replaceAll(" ", "")))
      .join("\n");
  }
}
