import {
  SoundLaw,
  soundlaw_xsampa_to_ipa,
  SoundLawComposition,
} from "../pkg/rust_capr";
import { DragType, RegexType, SoundClass, SoundLawInput } from "./types";

class FileArea {
  uploadFile: HTMLInputElement;
  fileStrings: string[];
  tranducer: SoundLawComposition;
  table: HTMLTableRowElement;
  tableHeader: HTMLTableRowElement;

  parent: Main;
  constructor(parent: Main) {
    this.table = document.getElementById("file-inputs") as HTMLTableRowElement;
    let tableHeader = document.getElementById(
      "file-headers",
    ) as HTMLTableRowElement;

    this.uploadFile = document.getElementById("upload") as HTMLInputElement;
    this.fileStrings = [];

    this.uploadFile.addEventListener("change", async () => {
      if (this.uploadFile.files) {
        let file = this.uploadFile.files[0];
        const text = await file.text();

        this.fileStrings = text.split("\n").filter((x) => x !== "");

        this.parent.transduce();
        this.renderHeader();
      } else {
        this.fileStrings = [];
      }
    });
  }

  renderHeader() {
    let newChildren = this.fileStrings.map((text) => {
      const elem = document.createElement("th");
      elem.textContent = soundlaw_xsampa_to_ipa(text);
      return elem;
    });
    this.tableHeader.replaceChildren(...newChildren);
  }

  transduce() {
    let body = this.table.querySelector("tbody");
    if (!body) {
      body = document.createElement("tbody");
      this.table.appendChild(body);
    }

    const transduced = this.fileStrings.map((s) =>
      this.tranducer.transduce_text(s),
    );

    const transpose = transduced[0].map((_, index) =>
      transduced.map((row) => {
        if (!row[index]) {
          return "";
        }
        return soundlaw_xsampa_to_ipa(row[index].replaceAll(" ", ""));
      }),
    );

    transpose.forEach((row) => {
      const htmlRow = document.createElement("tr");
      row.forEach((col) => {
        const item = document.createElement("td");
        item.textContent = col;
        htmlRow.appendChild(item);
      });
      body.append(htmlRow);
    });
    let newChildren = this.fileStrings.map((text) => {
      const elem = document.createElement("tr");
      elem.textContent = soundlaw_xsampa_to_ipa(text);
      return elem;
    });
    body.replaceChildren(...newChildren);
  }
}

class Main {
  soundClasses: SoundClass[];
  isLoading: boolean;
  soundLawInputs: SoundLawInput[];
  input: string;
  output: string[];
  reverseInput: string;
  revereseOutput: string[];
  composition: SoundLawComposition;
  fileStrings: string[];
  transducedFileStrings: string[][];
  drag: DragType;

  fileArea: FileArea;

  constructor() {
    this.fileArea = new FileArea(this);
  }

  transduce() {
    this.fileArea.transduce();
  }
}
